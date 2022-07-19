mod support;

use datastore::{DataDescriptor, StoreData};

use self::support::__Store;

#[test]
fn test_storedata_struct_0() {
    #[derive(StoreData)]
    struct SomeData {}

    assert_eq!(name!(SomeData), "SomeData");
    fields!(SomeData, {});
}

#[test]
fn test_storedata_struct_1() {
    #[derive(StoreData)]
    struct SomeData {
        x: u8,
    }

    fields!(SomeData, { "x" => U8 });
}

#[test]
fn test_storedata_struct_many() {
    #[derive(StoreData)]
    struct SomeData {
        f1: u8,
        f2: u16,
        f3: u32,
        f4: u64,
        f5: i8,
        f6: i16,
        f7: i32,
        f8: i64,
        f9: f32,
        f10: f64,
    }

    fields!(SomeData, {
        "f1" => U8,
        "f2" => U16,
        "f3" => U32,
        "f4" => U64,
        "f5" => I8,
        "f6" => I16,
        "f7" => I32,
        "f8" => I64,
        "f9" => F32,
        "f10" => F64,
    });
}

#[test]
fn test_storedata_name() {
    #[derive(StoreData)]
    struct SomeData {}

    assert_eq!(name!(SomeData), "SomeData");

    #[derive(StoreData)]
    #[datastore(name = "name")]
    struct SomeData2 {}

    assert_eq!(name!(SomeData2), "name");
}
