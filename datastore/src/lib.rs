use async_trait::async_trait;

#[cfg(feature = "derive")]
pub use datastore_derive::StoreData;

#[async_trait]
pub trait Store: Sized + Send + Sync {
    type DataStore: Store;

    type Error;

    /// Connects to the store using the given uri.
    async fn connect(uri: &str) -> Result<Self, Self::Error>;

    /// Initializes the store for storing data of the type `T`. If `create` was not called before
    /// calling [`delete`], [`get`], [`get_all`], [`get_one`] or [`insert`] on the store, the
    /// operation might fail.
    ///
    /// Note: Calling `create` might not be required for all store types. Calling `create` on a
    /// store that does not require this call or has already initialized for storing `T` should not
    /// fail.
    ///
    /// [`delete`]: Self::delete
    /// [`get`]: Self::get
    /// [`get_all`]: Self::get_all
    /// [`get_one`]: Self::get_one
    /// [`insert`]: Self::insert
    async fn create<T, D>(&self, descriptor: D) -> Result<(), Self::Error>
    where
        T: StoreData<Self::DataStore> + Send + Sync + 'static,
        D: DataDescriptor<T, Self::DataStore> + Send + Sync;

    /// Deletes all items `T` matching the query `Q` from the store.
    async fn delete<T, D, Q>(&self, descriptor: D, query: Q) -> Result<(), Self::Error>
    where
        T: StoreData<Self::DataStore> + Send + Sync + 'static,
        D: DataDescriptor<T, Self::DataStore> + Send,
        Q: DataQuery<T, Self::DataStore> + Send;

    /// Returns all items `T` matching the query `Q` from the store. If no matching items are
    /// found an empty [`Vec`] is returned.
    async fn get<T, D, Q>(&self, descriptor: D, query: Q) -> Result<Vec<T>, Self::Error>
    where
        T: StoreData<Self::DataStore> + Send + Sync + 'static,
        D: DataDescriptor<T, Self::DataStore> + Send,
        Q: DataQuery<T, Self::DataStore> + Send;

    /// Returns all items `T` from the store. If no items are found an empty [`Vec`] is returned.
    async fn get_all<T, D>(&self, descriptor: D) -> Result<Vec<T>, Self::Error>
    where
        T: StoreData<Self::DataStore> + Send + Sync + 'static,
        D: DataDescriptor<T, Self::DataStore> + Send + Sync;

    /// Returns an item `T` matching the query `Q` from store. If no matching item is found `None`
    /// is returned.
    ///
    /// Note: There is no guarantee on the item order. Calling `get_one` multiple times with the
    /// same query might return different items on the same store.
    async fn get_one<T, D, Q>(&self, descriptor: D, query: Q) -> Result<Option<T>, Self::Error>
    where
        T: StoreData<Self::DataStore> + Send + Sync + 'static,
        D: DataDescriptor<T, Self::DataStore> + Send,
        Q: DataQuery<T, Self::DataStore> + Send;

    /// Inserts a new item `T` into the store.
    async fn insert<T, D>(&self, descriptor: D, data: T) -> Result<(), Self::Error>
    where
        T: StoreData<Self::DataStore> + Send + Sync + 'static,
        D: DataDescriptor<T, Self::DataStore> + Send;
}

pub trait StoreExt<S>
where
    S: Store,
{
    fn descriptor<T>(&self) -> T::Descriptor
    where
        T: StoreData<S::DataStore>,
        T::Descriptor: Default;
}

impl<S> StoreExt<S> for S
where
    S: Store,
{
    #[inline]
    fn descriptor<T>(&self) -> T::Descriptor
    where
        T: StoreData<S::DataStore>,
        T::Descriptor: Default,
    {
        T::Descriptor::default()
    }
}

pub trait StoreData<S>: Sized
where
    S: Store,
{
    type Descriptor: DataDescriptor<Self, S>;
    type Query: DataQuery<Self, S>;

    fn write<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: Writer<S>;

    fn read<R>(reader: &mut R) -> Result<Self, R::Error>
    where
        R: Reader<S>;
}

pub trait DataDescriptor<T, S>
where
    T: StoreData<S>,
    S: Store,
{
    fn ident(&self) -> &str;

    fn write<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: TypeWriter<S>;
}

pub trait DataQuery<T, S>
where
    T: StoreData<S>,
    S: Store,
{
    fn write<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: Writer<S>;
}

pub trait Writer<S>
where
    S: Store,
{
    type Error;

    /// Writes a `bool` value.
    fn write_bool(&mut self, v: bool) -> Result<(), Self::Error>;

    /// Writes a `i8` value.
    fn write_i8(&mut self, v: i8) -> Result<(), Self::Error>;

    /// Writes a `i16` value.
    fn write_i16(&mut self, v: i16) -> Result<(), Self::Error>;

    /// Writes a `i32` value.
    fn write_i32(&mut self, v: i32) -> Result<(), Self::Error>;

    /// Writes a `i64` value.
    fn write_i64(&mut self, v: i64) -> Result<(), Self::Error>;

    /// Writes a `u8` value.
    fn write_u8(&mut self, v: u8) -> Result<(), Self::Error>;

    /// Writes a `u16` value.
    fn write_u16(&mut self, v: u16) -> Result<(), Self::Error>;

    /// Writes a `u32` value.
    fn write_u32(&mut self, v: u32) -> Result<(), Self::Error>;

    /// Writes a `u64` value.
    fn write_u64(&mut self, v: u64) -> Result<(), Self::Error>;

    /// Writes a `f32` value.
    fn write_f32(&mut self, v: f32) -> Result<(), Self::Error>;

    /// Writes a `f64` value.
    fn write_f64(&mut self, v: f64) -> Result<(), Self::Error>;

    /// Writes a raw byte slice.
    fn write_bytes(&mut self, v: &[u8]) -> Result<(), Self::Error>;

    /// Writes a `&str`.
    fn write_str(&mut self, v: &str) -> Result<(), Self::Error>;

    /// Writes a field with the key `key` and the value `T`.
    fn write_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Write<S>;
}

pub trait Reader<S>
where
    S: Store,
{
    type Error;

    fn read_bool(&mut self) -> Result<bool, Self::Error>;

    fn read_i8(&mut self) -> Result<i8, Self::Error>;

    fn read_i16(&mut self) -> Result<i16, Self::Error>;

    fn read_i32(&mut self) -> Result<i32, Self::Error>;

    fn read_i64(&mut self) -> Result<i64, Self::Error>;

    fn read_u8(&mut self) -> Result<u8, Self::Error>;

    fn read_u16(&mut self) -> Result<u16, Self::Error>;

    fn read_u32(&mut self) -> Result<u32, Self::Error>;

    fn read_u64(&mut self) -> Result<u64, Self::Error>;

    fn read_f32(&mut self) -> Result<f32, Self::Error>;

    fn read_f64(&mut self) -> Result<f64, Self::Error>;

    fn read_byte_buf(&mut self) -> Result<Vec<u8>, Self::Error>;

    fn read_string(&mut self) -> Result<String, Self::Error>;

    fn read_field<T>(&mut self, key: &'static str) -> Result<T, Self::Error>
    where
        T: Sized + Read<S>;
}

pub trait TypeWriter<S>
where
    S: Store,
{
    type Error;

    fn write_bool(&mut self) -> Result<(), Self::Error>;

    fn write_i8(&mut self) -> Result<(), Self::Error>;
    fn write_i16(&mut self) -> Result<(), Self::Error>;
    fn write_i32(&mut self) -> Result<(), Self::Error>;
    fn write_i64(&mut self) -> Result<(), Self::Error>;

    fn write_u8(&mut self) -> Result<(), Self::Error>;
    fn write_u16(&mut self) -> Result<(), Self::Error>;
    fn write_u32(&mut self) -> Result<(), Self::Error>;
    fn write_u64(&mut self) -> Result<(), Self::Error>;

    fn write_f32(&mut self) -> Result<(), Self::Error>;
    fn write_f64(&mut self) -> Result<(), Self::Error>;

    fn write_bytes(&mut self) -> Result<(), Self::Error>;
    fn write_str(&mut self) -> Result<(), Self::Error>;

    fn write_field<T>(&mut self, key: &'static str) -> Result<(), Self::Error>
    where
        T: ?Sized + Write<S>;
}

pub trait Write<S>
where
    S: Store,
{
    fn write<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: Writer<S>;

    fn write_type<W>(writer: &mut W) -> Result<(), W::Error>
    where
        W: TypeWriter<S>;
}

pub trait Read<S>: Sized
where
    S: Store,
{
    fn read<R>(reader: &mut R) -> Result<Self, R::Error>
    where
        R: Reader<S>;
}
