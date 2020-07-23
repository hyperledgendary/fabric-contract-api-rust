---
layout: home
---

This **Technology Preview** provides an updated Contract and Ledger API in Rust. It is targetted to be compiled to WebAssembly and run in the `fabric-chaincode-wasm` engine.

The API presented here is the evolution of the APIs available in the other SDKs to support developing smart contracts (chaincode). There are three other smart contract SDKs available for Go, Node.js, and Java SDK:

  * [Go SDK documentation](https://godoc.org/github.com/hyperledger/fabric/core/chaincode/shim)
  * [Node.js SDK documentation](https://hyperledger.github.io/fabric-chaincode-node/)
  * [Java documentation](https://hyperledger.github.io/fabric-chaincode-java/)

## Getting setup

- Install the preqres for Rust and Wasm Development
  - Stable Rust is sufficient, nightly is not required. Instructions at the [rust-lang.org](https://www.rust-lang.org/tools/install)
  - To build a Wasm binary you will need to have the wasm target. Note that wasm-pack is not required here as there is no JavaScript host.
    - `rustup target add wasm32-unknown-unknown` 

VSCode is our preferred editor, with the Rust Extension and the Rust Analyser

- Clone this repo
  - `git clone https://github.com/hyperledgendary/fabric-contract-api-rust.git`

- Ensure it can be built correctly
  - Using make: `make -f justfile wasm`
  - Using [just](https://github.com/casey/just): `just wasm`   
  - Using cargo: `cargo build --target wasm32-unknown-unknown`

- Follow the [Wasm Contract Cheat Sheet](....) to get the basic contract up and working. 


## Documentation

[API Documentation](./apidoc/fabric_contract/index.html)

## Download


## Compatibility


## Samples

Two examples are available in this repo.

- [Basic Asset Transfer](https://github.com/hyperledgendary/fabric-contract-api-rust/tree/master/basic_contract_rs)
- [Secure Asset Transfer](https://github.com/hyperledgendary/fabric-contract-api-rust/tree/master/asset_transfer_rs)
