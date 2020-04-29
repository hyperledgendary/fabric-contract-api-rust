# Fabric Rust Contract-API

The Fabric Contract API in rust; this is written with Wasm as the target runtime not native.

A SmartContract is single crate, containing one or more contract sructs, compiled to a Wasm library. In this repo the `basic_contract_rs`
is an example. This is a simple asset based contract based on the standard vs code extension examples.

`fabric_contract` is the main crate that provides the apis for writing a contract and accessing the ledger. `contract_macros` contain the
macros required (as rust currently requires procedural macros to be in their own crate).


**NOTE: All very early in development; subject to change! No suggestion is too daft (within limits)**

## Build

To build a Wasm binary

```
cargo build --target wasm32-unknown-unknown
```
Joh
File will be in `target/wasm32-unknown-unknown/debug/basic_contract_rs.wasm`


To build documentation

```
cargo docs --no-deps --open
```

There is also a `justfile` for quick command running.

