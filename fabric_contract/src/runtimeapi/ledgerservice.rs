/*
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::runtimeapi::wapc::runtime_host_call;
use crate::ledgerapi::*;
/// This module contains the APIs that the peer is logically
/// exposing. i.e. the fabric-protos-ledger
///
///

pub struct LedgerService {}

impl LedgerService {
    pub fn create_state(key: String, data: Vec<u8>) -> String {
        // create the protobuf message and pass to waPC
        // let buffer = vec![];
        // runtime_host_call("ledger_service".to_string(), buffer)
        String::from("todo")
    }

    pub fn read_state() -> State {
        todo!("update state")
        // create the protobuf message and pass to waPC
        // let buffer = vec![];
        // let response = runtime_host_call("ledger_service".to_string(), buffer);

        // // convert response into the 
        // State::new()

    }

    pub fn update_state() -> String {
        todo!("update state")
    }

    pub fn delete_state() -> String {
        todo!("update state")
    }

    pub fn exists_state() -> String {
        todo!("update state")
    }
}
