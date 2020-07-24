---
layout: home
title: Rust Smart Contract API
---

This **Technology Preview** provides an updated Contract and Ledger API in Rust. It is targetted to be compiled to WebAssembly and run in the `fabric-chaincode-wasm` engine.

The API presented here is the evolution of the APIs available in the other SDKs to support developing smart contracts (chaincode). There are three other smart contract SDKs available for Go, Node.js, and Java SDK:

  * [Go SDK documentation](https://godoc.org/github.com/hyperledger/fabric/core/chaincode/shim)
  * [Node.js SDK documentation](https://hyperledger.github.io/fabric-chaincode-node/)
  * [Java documentation](https://hyperledger.github.io/fabric-chaincode-java/)

## Getting setup

You'll need some tools installed, some you might already have

- [docker](https://docs.docker.com/get-docker/) & [docker-compose](https://docs.docker.com/compose/install/)
- rust and the Wasm toolchain
  - Stable Rust is sufficient, nightly is not required. Instructions at the [rust-lang.org](https://www.rust-lang.org/tools/install)
  - To build a Wasm binary you will need to have the wasm target. Note that wasm-pack is not required here as there is no JavaScript host.
    - `rustup target add wasm32-unknown-unknown` 
- git
- [just](https://github.com/casey/just) for use as a script running

VSCode is our preferred editor, with the Rust Extension and the Rust Analyser. You may, of course, have your own favourite editor.

## Guides

- [API Documentation](./apidoc/fabric_contract/index.html)
{% for doc in site.pages %} {% if doc.category =='guides' %}
- [{{ doc.title }}]({{site.baseurl}}{{ doc.url }})
{% endif %}
{% endfor %}

## Samples

Two examples are available in this repo.

- [Basic Asset Transfer](https://github.com/hyperledgendary/fabric-contract-api-rust/tree/master/basic_contract_rs)
- [Secure Asset Transfer](https://github.com/hyperledgendary/fabric-contract-api-rust/tree/master/asset_transfer_rs)
