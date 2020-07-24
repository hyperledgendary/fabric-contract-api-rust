---
title: Writing a Rust Contract
layout: page
category: guides
---

This guide will detail some of the finer points in writing a Rust Smart Contract. It's worth reading this alongside the [basic_contract_rs](https://github.com/hyperledgendary/fabric-contract-api-rust/tree/master/basic_contract_rs)

## Project Structure

A single *library* crate is the deployable chaincode unit, within which there can be one or more Contracts defined. Each Contract is defined as a single struct on which there are 'transaction functions' that can be invoked from the client application.

If you are familar with the Node, Java or Go Smart Contracts thess should be familiar patterns.

### Wasm note
For compilation to Wasm the `crate-type` should be set to `["cdylib"]`. 

### Dependencies
- `fabric_contract` crate is required for the contract features.
- `log` is usually included to let the contract implementation log it's own messages
- `wapc-guest` is required by the `fabric_contract` library (note we'd like to remove this from being required directly by the contract implementation, but have been unable to do so)

### Rust versions
Stable Rust is fine for compiling, there are no nightly features required.
The Wasm toolchain will be required. This can be installed with `rustup target add wasm32-unknown-unknown` 

## lib.rs 

You must 'register' the contracts, it is recommended that this is down in the `lib.rs` of the library crate. 

For example

```rust
// The macro to indicate the function that will be used to register the contracts.
// if there is more than one, use a comma separated list
//
// Provide the function that will create a new instance of the contract strcut
// More than one contract struct can be provided. 
fabric_contract::register!( AssetContract::new );
```

The first contract is the *default* contract - the default concept is relavent when it comes to calling transaction functions from the client. 

## Contract

- Within each crate, you must have at least one contract. 
- Each contract is a `struct` that must implement the `Contract` trait
- The impl of the contract structure must be decratored wtih the `#[Contract_Impl]` macro
- There must be a `new()` function or equivalent, that must be passed to the `register!` macro
- The `contract` trait includes some methods to assist - it is strongly recommended that the `name()` function is implemented. The String returned from this is what is used to 'namespace' different contracts.

## Transaction Functions

- Each contract must have one or more transaction functions
- They may any other functions or associate functions
- A function is classed as a transaction function if it is decorated with `#[Transaction( )]` macro
- The must have public visibility
- This macro takes a parameter of `evaluate` or `submit` to indicate how the invoke of this transaction is meant to be handled. This is, as with the other chaincodes, currently advisory and not enforced
- Transaction functions are NOT associate functions so must accept `&self` as the first parameter
- Each Transaction function must return a `Result<?,ContractError>` where the ? can be of any type
- Arguments, currently, can be Strings or other primitives.

## Other code
There are no restrictions on other code, either functions in the contract structs or any other rust code. 

Keep in mind any crates that may not compile to Wasm.