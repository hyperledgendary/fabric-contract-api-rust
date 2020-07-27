# Fabric Rust Contract-API

"The Fabric Contract API in rust, with Wasm as the compilation target."

A SmartContract is single crate, containing one or more contract sructs, compiled to a Wasm library. In this repo the `basic_contract_rs` is an example. This is a simple asset based contract based on the standard vs code extension examples.

[End-to-End Getting Started Guide](https://hyperledgendary.github.io/fabric-contract-api-rust/guides/getting-started.html)

[Check the API Documentation for fabric_contract](https://hyperledgendary.github.io/fabric-contract-api-rust/apidoc/fabric_contract/index.html)

[Full Documentation](https://hyperledgendary.github.io/fabric-contract-api-rust/)

## Technology Preview

NOTE this is a technology preview, rather than production ready. Released for feedback and community experimentation.

## Tl;Dr; docs

### To write and deploy a Smart Contract

At a very highlevel the steps are

0. As the crates are not published yet, clone this entire cargo workspace to build both the fabric_contract and your own crates
1. Write the Smart Contract in rust, see `basic_contract_rs` for an example
2. Compile this targetting wasm. `cargo build --target wasm32-unknown-unknown`
3. Take the resulting Wasm binary and use the `https://github.com/hyperledgendary/fabric-chaincode-wasm` project to encapsualte this Wasm file in the 'Wasm Chaincode Runtime'
4. Once deployed this you interact with this like another contract


For all the steps please follow the [Getting Started Guide](https://hyperledgendary.github.io/fabric-contract-api-rust/guides/getting-started.html)

### Summary of how to build for Wasm

To build a Wasm binary you will need to have rust installed (stable branch is good, nightly is not required). You will need to add the wasm target.
(`rustup target add wasm32-unknown-unknown` if you don't have the Wasm toolchain. Note that wasm-pack is not required here as there is no JavaScript host)

```
cargo build --target wasm32-unknown-unknown
```

File will be in `target/wasm32-unknown-unknown/debug/basic_contract_rs.wasm`


To build documentation

```
just docs
```

There is also a `justfile` for quick command running.

