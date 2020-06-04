/*
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::runtimeapi::wapc::runtime_host_call;
use crate::ledgerapi::*;

use protobuf::{parse_from_bytes,Message};

use fabric_ledger_protos::{ledger_messages};

/// This module contains the APIs that the peer is logically
/// exposing. i.e. the fabric-protos-ledger
///
///

pub struct LedgerService {}

impl LedgerService {
    pub fn create_state(key: String, data: Vec<u8>) -> Result<State,String> {
        // create the protobuf message and pass to waPC
        let buffer = vec![];

        let mut state = ledger_messages::State::new();
        state.set_key(key);
        state.set_value(data);
        let mut csr = ledger_messages::CreateStateRequest::new();

        // TODO: need to get the txid from LTS or similar
        csr.set_transaction_id("1234".to_string());
        csr.set_state(state);
        // need to get the ownership back again
        let state = csr.get_state();
        runtime_host_call("ledger_service".to_string(), buffer);
        Ok(State::from(state))
    }

    pub fn read_state(key: String) -> Result<State,String> {

        // create the protobuf message and pass to waPC
        let buffer = vec![];
        let mut rsr = ledger_messages::ReadStateRequest::new();

        // TODO: need to get the txid from LTS or similar
        rsr.set_transaction_id("1234".to_string());
        rsr.set_state_key(key);
        
        let response_buffer: Vec<u8> = runtime_host_call("ledger_service".to_string(), buffer);
        let response = parse_from_bytes::<ledger_messages::ReadStateResponse>(&response_buffer).unwrap();
        let state = response.get_state();
        Ok(State::from(state))
    }

    pub fn update_state() -> String {
        todo!("update state")
    }

    pub fn delete_state() -> String {
        todo!("update state")
    }

    pub fn exists_state(key: &String) -> Result<bool,String> {
        // create the protobuf message and pass to waPC
        let buffer = vec![];
        let mut rsr = ledger_messages::ExistsStateRequest::new();

        // TODO: need to get the txid from LTS or similar
        rsr.set_transaction_id("1234".to_string());
        rsr.set_state_key(key.clone());
        
        let response_buffer: Vec<u8> = runtime_host_call("ledger_service".to_string(), buffer);
        let response = parse_from_bytes::<ledger_messages::ExistsStateResponse>(&response_buffer).unwrap();
        
        Ok(response.get_exists())
    }
}
