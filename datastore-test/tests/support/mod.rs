mod types;

use std::collections::HashMap;
use std::convert::Infallible;
use std::error;
use std::fmt::{self, Display, Formatter};

use async_trait::async_trait;
use datastore::{DataDescriptor, DataQuery, Error, Store, StoreData, TypeWriter, Write};

#[macro_export]
macro_rules! __descriptor {
    ($data:ident) => {{
        fn __descriptor() -> impl DataDescriptor<$data, __Store> {
            <$data as StoreData<__Store>>::Descriptor::default()
        }

        __descriptor()
    }};
}

#[macro_export]
macro_rules! name {
    ($data:ident) => {
        __descriptor!($data).ident()
    };
}

#[macro_export]
macro_rules! fields {
    ($data:ident, { $($key:expr => $val:tt),* $(,)? }) => {{
        let mut writer = $crate::support::__TypeWriter::new();
        __descriptor!($data).write(&mut writer).unwrap();

        $(
            match writer.values.remove($key) {
                Some(val) => {
                    if val != $crate::support::Type::$val {
                        panic!("unexpected type {:?} at {:?} (expected {:?})", val, $key, $crate::support::Type::$val);
                    }
                },
                None => panic!("missing field {:?} (expected {:?})", $key, $crate::support::Type::$val),
            }
        )*

        if !writer.values.is_empty() {
            for (key, val) in writer.values.into_iter() {
                println!("unexpected field {:?} with type {:?} remaining", key, val);
            }

            panic!("not all fields consumed");
        }
    }};
}

pub struct __Store;

#[async_trait]
impl Store for __Store {
    type DataStore = Self;
    type Error = __Error;

    async fn connect(_uri: &str) -> Result<Self, Self::Error> {
        Ok(Self)
    }

    async fn create<T, D>(&self, _descriptor: D) -> Result<(), Self::Error>
    where
        T: StoreData<Self::DataStore> + Send + Sync + 'static,
        D: DataDescriptor<T, Self::DataStore> + Send + Sync,
    {
        Ok(())
    }

    async fn delete<T, D, Q>(&self, _descriptor: D, _query: Q) -> Result<(), Self::Error>
    where
        T: StoreData<Self::DataStore> + Send + Sync + 'static,
        D: DataDescriptor<T, Self::DataStore> + Send,
        Q: DataQuery<T, Self::DataStore> + Send,
    {
        Ok(())
    }

    async fn get<T, D, Q>(&self, _descriptor: D, _query: Q) -> Result<Vec<T>, Self::Error>
    where
        T: StoreData<Self::DataStore> + Send + Sync + 'static,
        D: DataDescriptor<T, Self::DataStore> + Send,
        Q: DataQuery<T, Self::DataStore> + Send,
    {
        Ok(Vec::new())
    }

    async fn get_one<T, D, Q>(&self, _descriptor: D, _query: Q) -> Result<Option<T>, Self::Error>
    where
        T: StoreData<Self::DataStore> + Send + Sync + 'static,
        D: DataDescriptor<T, Self::DataStore> + Send,
        Q: DataQuery<T, Self::DataStore> + Send,
    {
        Ok(None)
    }

    async fn get_all<T, D>(&self, _descriptor: D) -> Result<Vec<T>, Self::Error>
    where
        T: StoreData<Self::DataStore> + Send + Sync + 'static,
        D: DataDescriptor<T, Self::DataStore> + Send + Sync,
    {
        Ok(Vec::new())
    }

    async fn insert<T, D>(&self, _descriptor: D, _data: T) -> Result<(), Self::Error>
    where
        T: StoreData<Self::DataStore> + Send + Sync + 'static,
        D: DataDescriptor<T, Self::DataStore> + Send,
    {
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct __Error;

impl Display for __Error {
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl error::Error for __Error {}

impl Error for __Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: Display,
    {
        Self
    }
}

#[derive(Debug)]
pub struct __TypeWriter {
    typ: Type,
    pub values: HashMap<String, Type>,
}

impl __TypeWriter {
    pub fn new() -> Self {
        Self {
            typ: Type::Bool,
            values: HashMap::new(),
        }
    }
}

impl TypeWriter<__Store> for __TypeWriter {
    type Error = Infallible;

    fn write_bool(&mut self) -> Result<(), Self::Error> {
        self.typ = Type::Bool;
        Ok(())
    }

    fn write_i8(&mut self) -> Result<(), Self::Error> {
        self.typ = Type::I8;
        Ok(())
    }

    fn write_i16(&mut self) -> Result<(), Self::Error> {
        self.typ = Type::I16;
        Ok(())
    }

    fn write_i32(&mut self) -> Result<(), Self::Error> {
        self.typ = Type::I32;
        Ok(())
    }

    fn write_i64(&mut self) -> Result<(), Self::Error> {
        self.typ = Type::I64;
        Ok(())
    }

    fn write_u8(&mut self) -> Result<(), Self::Error> {
        self.typ = Type::U8;
        Ok(())
    }

    fn write_u16(&mut self) -> Result<(), Self::Error> {
        self.typ = Type::U16;
        Ok(())
    }

    fn write_u32(&mut self) -> Result<(), Self::Error> {
        self.typ = Type::U32;
        Ok(())
    }

    fn write_u64(&mut self) -> Result<(), Self::Error> {
        self.typ = Type::U64;
        Ok(())
    }

    fn write_f32(&mut self) -> Result<(), Self::Error> {
        self.typ = Type::F32;
        Ok(())
    }

    fn write_f64(&mut self) -> Result<(), Self::Error> {
        self.typ = Type::F64;
        Ok(())
    }

    fn write_bytes(&mut self) -> Result<(), Self::Error> {
        self.typ = Type::Bytes;
        Ok(())
    }

    fn write_str(&mut self) -> Result<(), Self::Error> {
        self.typ = Type::Str;
        Ok(())
    }

    fn write_field<T>(&mut self, key: &'static str) -> Result<(), Self::Error>
    where
        T: ?Sized + Write<__Store>,
    {
        T::write_type(self)?;
        self.values.insert(key.to_owned(), self.typ);
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Bool,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Bytes,
    Str,
}
