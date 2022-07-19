# datastore

[![Crates.io](https://img.shields.io/crates/v/datastore)](https://crates.io/crates/datastore)
[![Docs.rs](https://img.shields.io/docsrs/datastore/latest)](https://docs.rs/datastore)

A framework for generically storing data inside stores.

## Usage

Add `datastore` to your `Cargo.toml`:

```
datastore = { version = "0.1.5", features = ["derive"] }
```

Define some data using the `StoreData` macro:

```
use datastore::StoreData;

#[derive(StoreData)]
struct Person {
    id: i64,
    name: String,
}
```

datastore only defines a format describing how to read/write some data from/to a store. To use the defined format you need a crate with a [`Store`](https://docs.rs/datastore/latest/datastore/trait.Store.html) driver. See [datastore-mysql](https://github.com/MrGunflame/datastore-mysql) for an example store implementation.

## License

Licensed under either The [Apache License, Version 2.0](https://github.com/MrGunflame/datastore/blob/master/LICENSE-APACHE) or [MIT license](https://github.com/MrGunflame/datastore/blob/master/LICENSE-MIT) at your option.
