use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::{
        atomic::{AtomicU8, Ordering},
        Arc,
    },
};

use asyncsync::Notify;

use crate::{DataDescriptor, DataQuery, Store, StoreData};

#[derive(Debug)]
pub struct LazyStore<S>
where
    S: Store,
{
    uri: Box<str>,
    state: Arc<AtomicU8>,
    inner: UnsafeCell<MaybeUninit<S>>,
    on_unlock: Arc<Notify>,
}

impl<S> LazyStore<S>
where
    S: Store,
{
    async fn get(&self) -> Result<&S, S::Error> {
        let mut state = State::load(&self.state);

        // The inner store has been initialized.
        if state.is_init() {
            unsafe {
                let inner = &*self.inner.get();
                return Ok(inner.assume_init_ref());
            }
        }

        // The inner store is currently being initialized.
        // Wait for the inner store to be unlocked.
        while state.is_locked() {
            state = State::load(&self.state);
            self.on_unlock.notified().await;
        }

        if self.inner.is_none() {
            self.inner = Some(S::connect(&self.uri).await?);
        }

        Ok(self.inner.as_ref().unwrap())
    }

    pub async fn into_inner(self) -> Option<S> {}
}

impl<S> Store for LazyStore<S>
where
    S: Store,
{
    type DataStore = S;
    type Error = <S as Store>::Error;

    async fn connect(uri: &str) -> Result<Self, Self::Error> {
        let uri = uri.to_string().into_boxed_str();

        Self { uri, inner: None }
    }

    async fn create<T, D>(&self, descriptor: D) -> Result<(), Self::Error>
    where
        T: StoreData<Self::DataStore> + Send + Sync + 'static,
        D: DataDescriptor<T, Self::DataStore> + Send + Sync,
    {
    }
}

impl<S> Clone for LazyStore<S>
where
    S: Store,
{
    fn clone(&self) -> Self {
        Self {
            uri: self.uri.clone(),
            state: self.state.clone(),
        }
    }
}

unsafe impl<S> Send for LazyStore<S> where S: Store + Send {}
unsafe impl<S> Sync for LazyStore<S> where S: Store + Sync {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
struct State(u8);

impl State {
    /// Indicates whether the store has been successfully initialized.
    const INIT: u8 = 1 << 0;
    /// Indicates whether the store is currently being initialized.
    const LOCKED: u8 = 1 << 1;

    fn load(cell: &AtomicU8) -> Self {
        Self(cell.load(Ordering::Relaxed))
    }

    #[inline]
    fn is_init(&self) -> bool {
        self.0 & Self::INIT != 0
    }

    #[inline]
    fn is_locked(&self) -> bool {
        self.0 & Self::LOCKED != 0
    }

    fn lock(&self, cell: &AtomicU8) {
        cell.fetch_or(Self::LOCKED, Ordering::SeqCst);
    }
}
