#!/bin/bash

set -ex

# Grab the current directory
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )"/.. && pwd )"

# docker network to connect the two containers
# docker network create wasm_microfab 
# npm install -g @hyperledgendary/weftility

# Need to start the microfab server
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
docker network create wasm_microfab
docker run --name microfab --rm -d -p 8080:8080 -e MICROFAB_CONFIG="${MICROFAB_CONFIG}"  --network=wasm_microfab sstone1/microfab

WORKING_DIR="${DIR}/_bldtmp"
mkdir -p "${DIR}/_bldtmp"

cd ${WORKING_DIR}

curl -sSL https://raw.githubusercontent.com/hyperledger/fabric/master/scripts/bootstrap.sh | bash -s -- 2.2.0 1.4.4 0.4.18 -s -d

export PATH=$PATH:"${WORKING_DIR}/bin"
export FABRIC_CFG_PATH="${WORKING_DIR}/config"

# move over the chaincode-env an the proxy contract
cp ${DIR}/tests/assets/* ${WORKING_DIR}

# install helper
npm install -g @hyperledgendary/weftility


# get the credentials out of the microfab server
curl -s http://console.127-0-0-1.nip.io:8080/ak/api/v1/components | weft microfab -w ${WORKING_DIR}/_wallets -p ${WORKING_DIR}/_gateways -m ${WORKING_DIR}/_msp -f

# en vars for the peer commands
export CORE_PEER_LOCALMSPID=AmpretiaMSP                                       
export CORE_PEER_ADDRESS=ampretiapeer-api.127-0-0-1.nip.io:8080
export CORE_PEER_MSPCONFIGPATH="${WORKING_DIR}/_msp/Ampretia/ampretiaadmin/msp"

# install proxy chaincode
peer lifecycle chaincode install ${WORKING_DIR}/wasmftw.tgz

BUILD_TYPE=debug
# start the chaincode as a separate docker image
cp "${DIR}/target/wasm32-unknown-unknown/${BUILD_TYPE}/basic_contract_rs.wasm" ${WORKING_DIR}
docker run -d -t --rm -p 9999:9999 -v ${WORKING_DIR}:/local:ro --name wasmcc.example.com --hostname wasmcc.example.com --env-file ${WORKING_DIR}/chaincode.env --network=wasm_microfab calanais/fabric-chaincode-wasm:tp1
sleep 5
# approve
peer lifecycle chaincode approveformyorg -o orderer-api.127-0-0-1.nip.io:8080 --channelID minifignet --name wasmftw --version 1 --sequence 1 --waitForEvent --package-id wasmftw:eeae07c6e9455f329e28f9a0eed977ae3549be68e68247018f71dc5a5f511c0d

# commit
peer lifecycle chaincode commit -o orderer-api.127-0-0-1.nip.io:8080 --channelID minifignet --name wasmftw --version 1 --sequence 1
sleep 5
# runtests
# temporarily use the 'peer invoke' command ahead of full cucumber
peer chaincode invoke -o orderer-api.127-0-0-1.nip.io:8080 --channelID minifignet -n wasmftw -c '{"function":"AssetContract:create_asset","Args":["007","Bond"]}' 
sleep 5
peer chaincode query  -o orderer-api.127-0-0-1.nip.io:8080 --channelID minifignet -n wasmftw -c '{"function":"AssetContract:read_asset_value","Args":["007"]}' 
