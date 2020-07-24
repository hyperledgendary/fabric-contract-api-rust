# Rust Chaincode running in Wasm for Hyperledger Fabric

Here are the steps to run the `basic_contract_rs`. 
- Rust Smart Contract hosted within a Wasm engine
- Hyperledger Fabric v2

This guide will start up a 1 Organization, 1 Peer Fabric Network and start the contract.

Creating a working directory to hold the github repos we'll clone. (examples assume that this is ~/github.com)
A lot of the steps need only to be done once.

## Getting setup

- Install the preqres for Rust and Wasm Development
  - Stable Rust is sufficient, nightly is not required. Instructions at the [rust-lang.org](https://www.rust-lang.org/tools/install)
  - To build a Wasm binary you will need to have the wasm target. Note that wasm-pack is not required here as there is no JavaScript host.
    - `rustup target add wasm32-unknown-unknown` 

- VSCode is our preferred editor, with the Rust Extension and the Rust Analyser

### Hyperledger Fabric Docker images

We need Fabric Version 2, which you may already have so can skip this.  

```bash
curl -sSL https://raw.githubusercontent.com/hyperledger/fabric/master/scripts/bootstrap.sh | bash -s -- 2.0.0-beta 1.4.4 0.4.18 -s -b
```

### Get ready for the external builders

Fabric knows about the following languages implicitly. Java, Node, Go. However the peer is not really supposed to be the orchestrator of creating docker images. So this is where the external builder comes in. 

To setup the external builder, you need to configure the peer and the easiest way to do that is by using the `fabric-builders` project.

```bash
git clone https://github.com/hyperledgendary/fabric-builders
```

Next step is to build the updated peer docker image - we will need to use this to create a local Fabric network; the configuration of the external builder needs to modify the peers configuration file. This is easiest to do by creating a new docker image with the updated config file. 
This only need to be done the once. 

Change into the `fabric-builder-peer` repo and build the docker image

```bash
docker build -t hyperledgendary/fabric-builder-peer .
```

### Get Fabric Network setup

It's perfectly possible to use the `fabric-samples` test-network for this, that's 2 Organization network. For these instructions, we want to use a smaller network purely for development purposes.

In your working directory clone the `fabric-dev-networks` repo

```
git clone https://github.com/hyperledgendary/fabric-dev-networks.git
```

Then build the docker tools image that can be used to control it

```
docker build -t hyperledgendary/fabric-tools .
```

## Create Fabric network

Generate the small network config files

```
docker run --rm -v small_config:/etc/hyperledger/fabric -w /etc/hyperledger/fabric --entrypoint=/bin/bash hyperledgendary/fabric-tools -c "setnet.sh"
```

Replace `core.yaml` file with one from [fabric-builders](https://github.com/hyperledgendary/fabric-builders)

```
docker run --rm -v ${PWD}:/local:ro -v small_config:/etc/hyperledger/fabric -w /etc/hyperledger/fabric --entrypoint=/bin/bash hyperledgendary/fabric-tools -c "cp /local/core.yaml ."
```

Update the small network `docker-compose.yaml` file to use `hyperledgendary/fabric-builder-peer` images

```
sed -i.bak 's|hyperledger/fabric-peer|hyperledgendary/fabric-builder-peer|g' docker-compose.yaml
```

Bring up the docker compose network

```
docker-compose -f docker-compose.yaml up -d orderer.example.com peer0.humboldt.example.com couchdb cli
```

Create the Fabric network

```
docker exec humboldt.cli mknet.sh
```


## Package and install Wasm chaincode

Create a `connection.json` file with details of how Fabric will connect to the external service chaincode

```
{
  "address": "wasmcc.example.com:9999",
  "dial_timeout": "10s",
  "tls_required": false
}
```

Package the `connection.json` file using the [pkgcc.sh](https://github.com/hyperledgendary/fabric-builders/blob/master/tools/pkgcc.sh) script

```
pkgcc.sh -l wasmftw -t external connection.json
```

Copy the chaincode package to a docker volume for installing

```
docker run --rm -v ${PWD}:/local:ro -v small_cli:/var/hyperledgendary/fdn -w /var/hyperledgendary/fdn --entrypoint=/bin/bash hyperledgendary/fabric-tools -c "cp /local/wasmftw.tgz ."
```

Install the new chaincode package

```
docker exec humboldt.cli peer lifecycle chaincode install /var/hyperledgendary/fdn/wasmftw.tgz
```

## Run Wasm chaincode

Create a `chaincode.env` file, making sure the CHAINCODE_ID matches the chaincode code package identifier from the install command

```
CHAINCODE_SERVER_ADDRESS=wasmcc.example.com:9999
CHAINCODE_ID=wasmftw:a5ceee0db53cdb4b50975c2379cc346075697873341af71a7440b5d4d7f1ca0c
CHAINCODE_WASM_FILE=/local/fabric_contract.wasm
```

Run the chaincode

```
docker run -it --rm -v ${PWD}/contracts:/local:ro --name wasmcc.example.com --hostname wasmcc.example.com --env-file chaincode.env --network=small_fabricdev hyperledgendary/fabric-chaincode-wasm
```

## Approve and commit the Wasm chaincode

Approve the chaincode, making sure the CHAINCODE_ID matches the chaincode code package identifier from the install command

```
docker exec humboldt.cli peer lifecycle chaincode approveformyorg -o orderer.example.com:7050 --ordererTLSHostnameOverride orderer.example.com --tls true --cafile /etc/hyperledger/fabric/crypto-config/ordererOrganizations/example.com/orderers/orderer.example.com/msp/tlscacerts/tlsca.example.com-cert.pem --channelID smallchannel --name wasmftw --version 1 --sequence 1 --waitForEvent --package-id wasmftw:a5ceee0db53cdb4b50975c2379cc346075697873341af71a7440b5d4d7f1ca0c
```

Commit the chaincode

```
docker exec humboldt.cli peer lifecycle chaincode commit -o orderer.example.com:7050 --tls true --cafile /etc/hyperledger/fabric/crypto-config/ordererOrganizations/example.com/orderers/orderer.example.com/msp/tlscacerts/tlsca.example.com-cert.pem --channelID smallchannel --name wasmftw --version 1 --sequence 1
```

## Run a transaction!

Good luck!

```
docker exec humboldt.cli peer chaincode invoke -o orderer.example.com:7050 --tls true --cafile /etc/hyperledger/fabric/crypto-config/ordererOrganizations/example.com/orderers/orderer.example.com/msp/tlscacerts/tlsca.example.com-cert.pem -C smallchannel -n wasmftw -c '{"function":"create_asset","Args":["bond007"]}'
```



- In your working directory, clone this repo
  - `git clone https://github.com/hyperledgendary/fabric-contract-api-rust.git`

- Ensure it can be built correctly, cd into the `fabric-contract-api-rust`
  - Using make: `make -f justfile wasm`
  - Using [just](https://github.com/casey/just): `just wasm`   
  - Using cargo: `cargo build --target wasm32-unknown-unknown`

### Get your rust contract ready

Yes, first step is clone a repo!

```bash
git clone  git@github.com:hyperledgendary/fabric-contract-api-rust.git
```

This contains the Rust Contract API, and a sample contract.

```bash
cargo build --target wasm32-unknown-unknown
```
This will have built a Wasm binary to `fabric-contract-api-rust/target/wasm32-unknown-unknown/debug/basic_contract_rs.wasm`

## Build the external chaincode container

One final git repo to clone

```bash
git clone git@github.com:hyperledgendary/fabric-chaincode-wasm.git
```

Some of the intructions in this repo, we've already done as it makes sense to do it earlier. But we need to build the final docker image

```bash
docker build -t hyperledgendary/fabric-chaincode-wasm .
```

Copy the `chaincode.env.sample` to `chaincode.env`