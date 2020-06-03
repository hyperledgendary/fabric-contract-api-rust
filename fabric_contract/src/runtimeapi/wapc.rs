/*
 * SPDX-License-Identifier: Apache-2.0
 */

//! This is the runtime componet that marshalls the WAPC calls 
use prost::Message;
use std::str;
use crate::contractapi::context::Context;
use crate::contractapi::contractmanager::*;

extern crate wapc_guest as guest;
use guest::prelude::*;

use fabric_ledger_protos_rust::*;

/// General function to log messages
// #[link(wasm_import_module = "wapc")]
// extern "C" {
//     pub fn __log(ptr: *const u8, len: usize);
// }


///
/// Map to the ContractService
/// 
/// ```
/// service ContractService {
///  rpc GetMetadata (GetMetadataRequest) returns (GetMetadataResponse);
///  rpc InvokeTransaction (InvokeTransactionRequest) returns (InvokeTransactionResponse);
///  rpc RegisterPeer (RegisterPeerRequest) returns (RegisterPeerResponse);
/// }
/// ```
/// 
pub fn handle_wapc(operation: &str, msg: &[u8]) -> CallResult {
    match operation {
        "InvokeTransaction" => handle_tx_invoke(msg),
        "GetMetadata" => todo!("GetMetadata"),
        "RegisterPeer" => todo!("RegisterPeer"),
        _ => Err("Unknown function being called".into()),
    }
}

#[inline(never)]   // not sure why this is not inlined?
pub fn log(s: &str) {
    unsafe {
        guest::__console_log(s.as_ptr(), s.len());
    }
}

/// The API calls made by the users contract implementation via the Collection, State interfaces etc..
/// get rounted to here. They are then passed over the Wasm boundary to be sent off (somehow) to the peer
/// 
pub fn runtime_host_call(cmd: String, data: Vec<u8>) -> Vec<u8> {
    log(&format!("Making host call{}::{}",cmd,str::from_utf8(&data).unwrap())[..]);
    let res = host_call("wapc", &cmd[..],&data).unwrap();
    res
}

/// handle_tx_invoke called with the buffer that contains the request 
/// of what transaction function should be invoked
fn handle_tx_invoke(msg: &[u8]) -> CallResult {
    log("handler_tx_invoke");  

    // decode the message and arguments
    let invokeRequest = contract::InvokeTransactionRequest::decode(msg).unwrap();
    let fn_name = invokeRequest.transaction_name;
    let args = invokeRequest.payload;
    let ctx = Context::new(invokeRequest.channel_id, invokeRequest.transaction_id, log);

    // pass over to the contract manager to route
    log("making the routing calll");
    let ret = match ContractManager::route(ctx,fn_name,args) {
        Ok(r) =>  contract::InvokeTransactionResponse { payload : r.into_bytes() } ,
        Err(e) => contract::InvokeTransactionResponse { payload : e.into_bytes() } ,
    };
   
    let mut buffer = vec![];

    // encoding response from the transaction
    log("encoding response");
    if let Err(encoding_error) = ret.encode(&mut buffer) {
        panic!("Failed to encode {:?}",encoding_error);
    }
 
    log("done invoke");
    Ok(buffer)

}
