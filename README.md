# Fabric Rust Contract-API

"The Fabric Contract API in rust, with Wasm as the compilation target."

A SmartContract is single crate, containing one or more contract sructs, compiled to a Wasm library. In this repo the `basic_contract_rs` is an example. This is a simple asset based contract based on the standard vs code extension examples.

`fabric_contract` is the main crate that provides the apis for writing a contract and accessing the ledger. 

[Check the API Documentation for fabric_contract](https://hyperledgendary.github.io/fabric-contract-api-rust/apidoc/fabric_contract/index.html)


## To write and deploy a Smart Contract

0. As the crates are not published yet, clone this entire cargo workspace to build both the fabric_contract and your own crates
1. Write the Smart Contract in rust, see `basic_contract_rs` for an example
2. Compile this targetting wasm. `cargo build --target wasm32-unknown-unknown`
3. Take the resulting Wasm binary and use the `https://github.com/hyperledgendary/fabric-chaincode-wasm` project to encapsualte this Wasm file in the 'Wasm Chaincode Runtime'
4. Once deployed this you interact with this like another contract


## Build

To build a Wasm binary you will need to have rust installed (stable branch is good, nightly is not required). You will need to add the wasm target.

```
cargo build --target wasm32-unknown-unknown
```

File will be in `target/wasm32-unknown-unknown/debug/basic_contract_rs.wasm`


To build documentation

```
just docs
```

There is also a `justfile` for quick command running.

## Information

For information on Rust and Wasm see [the book](https://rustwasm.github.io/docs/book/).
Please note that section on [what crates may and may not work](https://rustwasm.github.io/docs/book/reference/which-crates-work-with-wasm.html) with Wasm for any dependencies that you use)

For informtion on how to write the contract, and how to do various fabric operations, please look at the `basic_contract_rs` first. Aim is try an add examples of the various fabric functions here.
