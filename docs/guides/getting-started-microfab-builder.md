---
title: Getting Rust Contract in Wasm Working with MicroFab and Wasm builder
layout: page
category: guides
---

Here are the steps to run the `basic_contract_rs` using.
- Rust Smart Contract hosted within a Wasm engine
- Hyperledger Fabric v2

This guide will start up a 1 Organization, 1 Peer Fabric Network and start the contract. This is done using a [fork of MicroFab](https://github.com/hyperledgendary/microfab) - a single containerized fabric network that is perfect for fast prototyping and developement. It's uses the Fabric binaries directly so it is really Fabric.

This is what we're going to be starting up and running,

‹Image TBC›

- MicroFab will provide the Fabric v2 runtime, into which we are going to deploy Wasm chaincode. We'll use the 'peer' cli commands to do this. 
- The chaincode here consists of a Wasm binary.
- The Wasm binary will have been built from the Rust `basic_contract_rs`

## Getting setup

- Install the preqres for Rust and Wasm Development
  - Stable Rust is sufficient, nightly is not required. Instructions at the [rust-lang.org](https://www.rust-lang.org/tools/install)
  - To build a Wasm binary you will need to have the wasm target. Note that wasm-pack is not required here as there is no JavaScript host.
    - `rustup target add wasm32-unknown-unknown` 
- VSCode is our preferred editor, with the Rust Extension and the Rust Analyser
- Create a working directory to hold the github repos we'll clone. (examples assume that this is ~/github.com). 


### Hyperledger Fabric Command Line Tools

We need Fabric Version 2 cli binaries, you may already have these so can skip this.  

If not, to get the `peer` commands (rather than the docker images or samples directory).

```bash
curl -sSL https://raw.githubusercontent.com/hyperledger/fabric/master/scripts/bootstrap.sh | bash -s -- 2.2.0 1.4.4 0.4.18 -s -d
```

Ensure that the commands and configuration are setup correctly.

```
export PATH=$PATH:~/github.com/bin
export FABRIC_CFG_PATH=~/github.com/config
```

## Creation 

Now we're going to create the various assets etc that we need

### Smart Contract

- In your working directory, clone this repo
  - `git clone https://github.com/hyperledgendary/fabric-contract-api-rust.git`

- Ensure it can be built correctly, cd into the `fabric-contract-api-rust`
  - Using make: `make -f justfile wasm`
  - Using [just](https://github.com/casey/just): `just wasm`   
  - Using cargo: `cargo build --target wasm32-unknown-unknown`

This will have built a Wasm binary to `fabric-contract-api-rust/target/wasm32-unknown-unknown/debug/basic_contract_rs.wasm`

As we've cloned the whole repo, we've also got the Rust Contract crates source as well. Normally you would just add the dependencies from `crates.io`

### Start Microfab

First setup an environment variable:

```bash
export MICROFAB_CONFIG='{
    "endorsing_organizations":[
        {
            "name": "Ampretia"
        }
    ],
    "channels":[
        {
            "name": "minifignet",
            "endorsing_organizations":[
                "Ampretia"
            ]
        }
    ],
    "capability_level":"V2_0"
}'
```

Then issue this docker command to run MicroFab

```bash
docker run --name microfab --rm -d -p 8080:8080 -e MICROFAB_CONFIG="${MICROFAB_CONFIG}" hyperledgendary/microfab
```

### Get the MicroFab configuration

When applications (including the Peer commands ) run they need a local identity in a wallet and a gateway connection profile. In this case there's a helpful script that can pull out all the information needed. 

Run this in your working directory - some sub-directories will be created. 

```bash
npm install -g @hyperledgendary/weftility
curl -s http://console.127-0-0-1.nip.io:8080/ak/api/v1/components | weft microfab -w ./_wallets -p ./_gateways -m ./_msp -f
```

Then setup some environment variables for the peer commands; these are

```bash
export CORE_PEER_LOCALMSPID=AmpretiaMSP                                       
export CORE_PEER_ADDRESS=ampretiapeer-api.127-0-0-1.nip.io:8080
export CORE_PEER_MSPCONFIGPATH="${WORKING_DIR}/_msp/Ampretia/ampretiaadmin/msp"
```

## Contract Deploy

Fabric knows about the following languages implicitly. Java, Node, Go. The forked version of Microfab started above is configured to handle Wasm in addition to the usual languages. 

You will need to package the Wasm chaincode in a tgz archive file before being able to install it in the same way as regular chaincode. Although you can create the chaincode package manually it is easier to use a copy of the [pkgcc.sh](https://github.com/hyperledgendary/fabric-builders/blob/master/tools/pkgcc.sh) script.

```bash
curl -s https://raw.githubusercontent.com/hyperledgendary/fabric-builders/master/tools/pkgcc.sh > ./pkgcc.sh && chmod u+x ./pkgcc.sh
```

### Package Wasm Chaincode

Package the `basic_contract_rs.wasm` file you built earlier using the [pkgcc.sh](https://github.com/hyperledgendary/fabric-builders/blob/master/tools/pkgcc.sh) script

Again if you have the same directory structure, use this command

```bash
./pkgcc.sh -l wasmftw -t wasm ./fabric-contract-api-rust/target/wasm32-unknown-unknown/debug/basic_contract_rs.wasm
```

A `wasmftw.tgz` file will be created. This is the chaincode package that will be installed on the peer.  Feel free to unpack and investigate it's contents.  Note that this is the equiavlent of doing a `peer lifecycle chaincode package`

### Install the Wasm Chaincode

Install this Wasm chaincode package.

```
peer lifecycle chaincode install wasmftw.tgz
```

It's important to keep the output from this command as it will be needed in the next step

### Approve and commit the Wasm chaincode

Approve the chaincode, making sure the `package-id` matches the chaincode code package identifier from the install command

```
peer lifecycle chaincode approveformyorg -o orderer-api.127-0-0-1.nip.io:8080 --channelID minifignet --name wasmftw --version 1 --sequence 1 --waitForEvent --package-id wasmftw:88fcf69b9ee0f9bb74b931fd0b526415cb14e111d98910e9c2d421f711cbdc46
```

Commit the chaincode

```
peer lifecycle chaincode commit -o orderer-api.127-0-0-1.nip.io:8080 --channelID minifignet --name wasmftw --version 1 --sequence 1
```

## Run a transaction!

Create an asset....

```bash
peer chaincode invoke -o orderer-api.127-0-0-1.nip.io:8080 --channelID minifignet -n wasmftw -c '{"function":"AssetContract:create_asset","Args":["007","Bond"]}'
```

Get back the value

```bash
peer chaincode query  -o orderer-api.127-0-0-1.nip.io:8080 --channelID minifignet -n wasmftw -c '{"function":"AssetContract:read_asset_value","Args":["007"]}'
```

## Using a node.js client application

Running the peer commands is ok, but really one of the SDKs will be used to run client applications.

As an example client application we can use [runhfsc](https://github.com/hyperledgendary/runhfsc). This uses the latest Fabric Node SDKs, and runs in node 12.

```bash
npm install -g @hyperledgendary/runhfsc
```

From your working directory, run this command to point the application at the client wallets and connection profile
```bash
runhfsc --gateway ./_gateways/ampretiagateway.json --wallet ./_wallets/Ampretia --user ampretiaadmin --channel minifignet
```

This will connect and you can see the prompt with the id and channel, type in `contract wasmftw`

```bash
[default] ampretiaadmin@minifignet:<contractid> - $ contract wasmftw
Contract set to wasmftw
[default] ampretiaadmin@minifignet:wasmftw 
```

We can now repeat the final peer command but driving the Node.js SDK

```bash
evaluate 'AssetContract:read_asset_value' '["007"]'
Submitted AssetContract:read_asset_value  007
[ '007' ]
> 
Bond
```
