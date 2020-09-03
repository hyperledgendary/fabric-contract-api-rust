---
title: Getting Rust Contract in Wasm Working with the Fabric test-network
layout: page
category: guides
---

## Introduction

In the main 'fabric-samples' the test-network is a two organization network configured to run the various samples and to support the Fabric documentation tutorials. 

Here are the steps to run the `basic_contract_rs`
- Rust Smart Contract hosted within a Wasm engine
- Hyperledger Fabric v2

Creating a working directory to hold the github repos we'll clone. (examples assume that this is ~/github.com). 

Firstly, we'll get setup with the required tools, then create two new docker images. 
Secondly, we'll create a Rust-compiled-wasm chaincode 
Finally, we'll install and startup the contract

## Getting setup

- Install the preqres for Rust and Wasm Development
  - Stable Rust is sufficient, nightly is not required. Instructions at the [rust-lang.org](https://www.rust-lang.org/tools/install)
  - To build a Wasm binary you will need to have the wasm target. Note that wasm-pack is not required here as there is no JavaScript host.
    - `rustup target add wasm32-unknown-unknown` 

- VSCode is our preferred editor, with the Rust Extension and the Rust Analyser
- docker and git as well

### Hyperledger Fabric Binaries, Docker and Samples 

We need Fabric Version 2, which you may already have so can skip this.  
There's an install script in Fabric that will do this for us.

This will get all the Docker Images, command line binaries, and the samples repo.

```bash
curl -sSL https://raw.githubusercontent.com/hyperledger/fabric/master/scripts/bootstrap.sh | bash -s -- 2.2.0 1.4.4 0.4.18 
```

### Building the two new docker images

There are 2 docker images we need to build. One is an updated Peer with an updated configuration. The second is the container that will host the running chaincode.

#### Configuring the Fabric Peer

Fabric knows about the following languages implicitly. Java, Node, Go. However the peer is not really supposed to be the orchestrator of creating docker images. So this is where the external builder comes in. 

To setup the external builder, you need to configure the peer and the easiest way to do that is by using the `fabric-builders` project.

```bash
git clone https://github.com/hyperledgendary/fabric-builders
```

Next step is to build the updated peer docker image - we will need to use this to create a local Fabric network; the configuration of the external builder needs to modify the peers configuration file. This is easiest to do by creating a new docker image with the updated config file. 

Change into the `fabric-builder-peer` repo and build the docker image

```bash
docker build -t hyperledgendary/fabric-builder-peer .
```

Using your editor of choice, locate the `fabric-samples/test-network/docker/docker-compose.yaml` file.  There are two references in this to the peer - update each of them to 


```
sed -i.bak 's|hyperledger/fabric-peer|hyperledgendary/fabric-builder-peer|g' docker-compose.yaml
<yaml here>
```

#### Confgiuring the Chaincode Docker Container
```bash
git clone git@github.com:hyperledgendary/fabric-chaincode-wasm.git
```

```bash
docker build -t hyperledgendary/fabric-chaincode-wasm .
```

## Creating the Smart Contract

This is in two parts, the actual code, and the 'proxy' chaincode that is installed to tell the peer the contract is running externally.

### Smart Contract

- In your working directory, clone this repo
  - `git clone https://github.com/hyperledgendary/fabric-contract-api-rust.git`

- Ensure it can be built correctly, cd into the `fabric-contract-api-rust`
  - Using make: `make -f justfile wasm`
  - Using [just](https://github.com/casey/just): `just wasm`   
  - Using cargo: `cargo build --target wasm32-unknown-unknown`

This will have built a Wasm binary to `fabric-contract-api-rust/target/wasm32-unknown-unknown/debug/basic_contract_rs.wasm`

### Proxy Chaincode
Create a 'connection.json' file with the following contents.

```
{
  "address": "wasmcc.example.com:9999",
  "dial_timeout": "10s",
  "tls_required": false
}
```
We need to package this up as a regular Chaincode Package. 

```
curl -s https://raw.githubusercontent.com/hyperledgendary/fabric-builders/master/tools/pkgcc.sh > ./pkgcc.sh && chmod u+x ./pkgcc.sh
```

A `wasmftw.tgz` file will be created. This is the chaincode package that will be installed on the peer - as the proxy for the real chaincode.  Feel free to unpack and investigate it's contents.

## Contract Deploy

We can now deploy the contract; these steps are mostly the same as for any Fabric Chaincode.

### Install Wasm chaincode

```
peer chaincode install..


get packageid...
```


### Start the Wasm Chaincode

For Java, Node, Go this bit is normally done for you.


### Run Wasm chaincode

Create a `chaincode.env` file, making sure the CHAINCODE_ID matches the chaincode code package identifier from the install command. (the one in the tests/assets directory should be sufficient if you've also copied the wasmftw.tgz file.  Just double check the package id matches)

```
CHAINCODE_SERVER_ADDRESS=wasmcc.example.com:9999
CHAINCODE_ID=wasmftw:eeae07c6e9455f329e28f9a0eed977ae3549be68e68247018f71dc5a5f511c0d
CHAINCODE_WASM_FILE=/local/basic_contract_rs.wasm
```

It's also worth copying the wasm binary file created previously, for example if you're following the same directory structure run this command from your working directory

```bash
mkdir -p contracts
cp ./fabric-contract-api-rust/target/wasm32-unknown-unknown/debug/basic_contract_rs.wasm ./contracts
```

Run the chaincode: note that this docker command runs it in foreground so you can watch what happens.  Worth opening another terminal at this point to run this in docker command in.

If you've built you wown fabric-chaincode-wasm container adjust the image name.

```
docker run -it --rm -v ${PWD}/contracts:/local:ro --name wasmcc.example.com --hostname wasmcc.example.com --env-file chaincode.env --network=wasm_network hyperledgendary/fabric-chaincode-wasm
```

### Approve and commit the Wasm chaincode

Approve the chaincode, making sure the `package-id` matches the chaincode code package identifier from the install command

```
peer lifecycle chaincode approveformyorg -o orderer-api.127-0-0-1.nip.io:8080 --channelID minifignet --name wasmftw --version 1 --sequence 1 --waitForEvent --package-id wasmftw:eeae07c6e9455f329e28f9a0eed977ae3549be68e68247018f71dc5a5f511c0d
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

### Run Wasm chaincode

Create a `chaincode.env` file, making sure the CHAINCODE_ID matches the chaincode code package identifier from the install command

```
CHAINCODE_SERVER_ADDRESS=wasmcc.example.com:9999
CHAINCODE_ID=wasmftw:a5ceee0db53cdb4b50975c2379cc346075697873341af71a7440b5d4d7f1ca0c
CHAINCODE_WASM_FILE=/local/basic_contract_rs.wasm
```

Run the chaincode: note that this docker command runs it in foreground so you can watch what happens.  Worth opening another terminal at this point.

```
docker run -it --rm -v ${PWD}/contracts:/local:ro --name wasmcc.example.com --hostname wasmcc.example.com --env-file chaincode.env --network=small_fabricdev hyperledgendary/fabric-chaincode-wasm
```

### Approve and commit the Wasm chaincode

Approve the chaincode, making sure the `package-id` matches the chaincode code package identifier from the install command

```
docker exec humboldt.cli peer lifecycle chaincode approveformyorg -o orderer.example.com:7050 --ordererTLSHostnameOverride orderer.example.com --tls true --cafile /etc/hyperledger/fabric/crypto-config/ordererOrganizations/example.com/orderers/orderer.example.com/msp/tlscacerts/tlsca.example.com-cert.pem --channelID smallchannel --name wasmftw --version 1 --sequence 1 --waitForEvent --package-id wasmftw:eeae07c6e9455f329e28f9a0eed977ae3549be68e68247018f71dc5a5f511c0d
```

Commit the chaincode

```
docker exec humboldt.cli peer lifecycle chaincode commit -o orderer.example.com:7050 --tls true --cafile /etc/hyperledger/fabric/crypto-config/ordererOrganizations/example.com/orderers/orderer.example.com/msp/tlscacerts/tlsca.example.com-cert.pem --channelID smallchannel --name wasmftw --version 1 --sequence 1
```

## Run a transaction!

Create an asset....

```bash
docker exec humboldt.cli peer chaincode invoke -o orderer.example.com:7050 --tls true --cafile /etc/hyperledger/fabric/crypto-config/ordererOrganizations/example.com/orderers/orderer.example.com/msp/tlscacerts/tlsca.example.com-cert.pem -C smallchannel -n wasmftw -c '{"function":"AssetContract:create_asset","Args":["007","Bond"]}'
```

```bash
docker exec humboldt.cli peer chaincode query -o orderer.example.com:7050 --tls true --cafile /etc/hyperledger/fabric/crypto-config/ordererOrganizations/example.com/orderers/orderer.example.com/msp/tlscacerts/tlsca.example.com-cert.pem -C smallchannel -n wasmftw -c '{"function":"AssetContract:read_asset_value","Args":["007"]}'
```

