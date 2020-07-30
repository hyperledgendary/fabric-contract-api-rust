/*
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::runtimeapi::wapc::runtime_host_call;
use crate::{contract::LedgerError, ledgerapi::*};

use protobuf::{parse_from_bytes, Message};

use fabric_ledger_protos::{common_messages, ledger_messages};

/// This module contains the APIs that the peer is logically
/// exposing. i.e. the fabric-protos-ledger
///
///
pub struct LedgerService {}

impl LedgerService {
    pub fn create_state(key: String, data: Vec<u8>) -> Result<State, LedgerError> {
        // Create the protobuf request for 'create_state'
        // and populate
        let mut state = ledger_messages::State::new();
        state.set_key(key);
        state.set_value(data);
        let mut csr = ledger_messages::CreateStateRequest::new();

        csr.set_context(LedgerService::get_context()?);
        csr.set_state(state);

        // need to get the ownership back again
        let state = csr.get_state();

        // create the buffer to send
        let buffer = csr.write_to_bytes().unwrap();

        // make the host call
        // note the response here is empty, so no requirement to handle it
        runtime_host_call(
            "LedgerService".to_string(),
            "CreateState".to_string(),
            buffer,
        );

        // Return the Ledger state object
        Ok(State::from(state))
    }

    pub fn read_state(key: String) -> Result<State, LedgerError> {
        // create the protobuf message and pass to waPC
        let mut rsr = ledger_messages::ReadStateRequest::new();

        rsr.set_context(LedgerService::get_context()?);
        rsr.set_state_key(key);

        let buffer = rsr.write_to_bytes().unwrap();

        // need to handle the response to the request
        let response_buffer: Vec<u8> =
            runtime_host_call("LedgerService".to_string(), "ReadState".to_string(), buffer);

        let response =
            parse_from_bytes::<ledger_messages::ReadStateResponse>(&response_buffer).unwrap();
        let state = response.get_state();
        Ok(State::from(state))
    }

    pub fn update_state(key: String, data: Vec<u8>) -> Result<State, LedgerError> {
        // Create the protobuf request for 'create_state'
        // and populate
        let mut state = ledger_messages::State::new();
        state.set_key(key);
        state.set_value(data);
        let mut usr = ledger_messages::UpdateStateRequest::new();

        usr.set_context(LedgerService::get_context()?);
        usr.set_state(state);

        // need to get the ownership back again
        let state = usr.get_state();

        // create the buffer to send
        let buffer = usr.write_to_bytes().unwrap();

        // make the host call
        // note the response here is empty, so no requirement to handle it
        runtime_host_call(
            "LedgerService".to_string(),
            "UpdateState".to_string(),
            buffer,
        );

        // Return the Ledger state object
        Ok(State::from(state))
    }

    pub fn delete_state(key: String) -> Result<(), LedgerError> {
        let mut dsr = ledger_messages::DeleteStateRequest::new();
        dsr.set_context(LedgerService::get_context()?);
        dsr.set_state_key(key);

        let buffer = dsr.write_to_bytes().unwrap();
        // make the host call
        // note the response here is empty, so no requirement to handle it
        runtime_host_call(
            "LedgerService".to_string(),
            "DeleteState".to_string(),
            buffer,
        );

        // Return the Ledger state object
        Ok(())
    }

    pub fn exists_state(key: &str) -> Result<bool, LedgerError> {
        // create the protobuf message and pass to waPC
        let mut esr = ledger_messages::ExistsStateRequest::new();
        esr.set_context(LedgerService::get_context()?);
        esr.set_state_key(key.to_string());

        let buffer = esr.write_to_bytes().unwrap();

        let response_buffer: Vec<u8> = runtime_host_call(
            "LedgerService".to_string(),
            "ExistsState".to_string(),
            buffer,
        );
        let response =
            parse_from_bytes::<ledger_messages::ExistsStateResponse>(&response_buffer).unwrap();

        Ok(response.get_exists())
    }

    // Gets the thread-local transaction context and creates the protobuf from it.
    fn get_context() -> Result<common_messages::TransactionContext, LedgerError> {
        let ctx = crate::runtimeapi::wapc::get_context();

        let mut tx_context = common_messages::TransactionContext::new();
        tx_context.set_transaction_id(ctx.get_txid());
        tx_context.set_channel_id(ctx.get_channelid());
        Ok(tx_context)
    }
}
