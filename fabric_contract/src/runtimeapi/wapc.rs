/*
 * SPDX-License-Identifier: Apache-2.0
 */
#![allow(dead_code)]

//! This is the runtime componet that marshalls the WAPC calls
use crate::{blockchain::TransactionContext, contractapi::contractmanager::*};
use protobuf::{parse_from_bytes, Message};
use std::cell::RefCell;
use std::str;

extern crate wapc_guest as guest;
use guest::prelude::*;
use wapc_guest::console_log;

use fabric_ledger_protos::contract_messages::*;

use log::{debug, trace};

///
/// Map to the ContractService
///
/// ```ignore
/// service ContractService {
///  rpc GetMetadata (GetMetadataRequest) returns (GetMetadataResponse);
///  rpc InvokeTransaction (InvokeTransactionRequest) returns (InvokeTransactionResponse);
///  rpc RegisterPeer (RegisterPeerRequest) returns (RegisterPeerResponse);
/// }
/// ```
///
pub fn handle_wapc(operation: &str, msg: &[u8]) -> CallResult {
    log(">> handle_wapc");
    match operation {
        "InvokeTransaction" => handle_tx_invoke(msg),
        "GetMetadata" => todo!("GetMetadata"),
        "RegisterPeer" => todo!("RegisterPeer"),
        _ => Err("Unknown operation being called".into()),
    }
}

#[inline(never)] // not sure why this is not inlined?
pub fn log(s: &str) {
    console_log(s);
}

/// The API calls made by the users contract implementation via the Collection, State interfaces etc..
/// get rounted to here. They are then passed over the Wasm boundary to be sent off (somehow) to the peer
///
pub fn runtime_host_call(service: String, cmd: String, data: Vec<u8>) -> Result<Vec<u8>> {
    trace!(
        "Making host call {}::{}::len={}::",
        service,
        cmd,
        data.len()
    );
    match host_call("wapc", &service[..], &cmd[..], &data) {
        Ok(v) => Ok(v),
        Err(e) => {
            debug!("{:?}", e);
            Err(e)
        }
    }
}

/// handle_tx_invoke called with the buffer that contains the request
/// of what transaction function should be invoked
fn handle_tx_invoke(msg: &[u8]) -> CallResult {
    trace!("handler_tx_invoke>>");

    // decode the message and arguments
    let invoke_request = parse_from_bytes::<InvokeTransactionRequest>(&msg).unwrap();
    let fn_name = invoke_request.get_transaction_name();
    let args = invoke_request.get_args();
    let transient_args = invoke_request.get_transient_args();
    let request_ctx = invoke_request.get_context();
    set_context(TransactionContext::new(request_ctx));
    let ctx = get_context();

    // pass over to the contract manager to route
    trace!(
        "making the routing call tx::{}  fn::{}",
        request_ctx.get_transaction_id(),
        fn_name
    );

    let mut response_msg = InvokeTransactionResponse::new();

    let ret = match ContractManager::route(&ctx, fn_name.to_string(), args, transient_args) {
        Ok(r) => {
            let buffer = match r.buffer {
                Some(r) => r,
                None => Vec::new(),
            };
            response_msg.set_payload(buffer)
        }
        Err(e) => response_msg.set_payload(e.to_string().into_bytes()),
    };

    let buffer: Vec<u8> = response_msg.write_to_bytes()?;
    trace!("handler_tx_invoke<<");
    Ok(buffer)
}

thread_local! {
    pub static CONTEXT: RefCell<TransactionContext>
    = RefCell::new( Default::default() );
}

fn set_context(name: TransactionContext) {
    CONTEXT.with(|ctx| *ctx.borrow_mut() = name);
}

pub fn get_context() -> TransactionContext {
    CONTEXT.with(|ctx| ctx.borrow().clone())
}
