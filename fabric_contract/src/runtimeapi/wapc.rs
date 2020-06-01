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

// Include the `items` module, which is generated from items.proto.
// TODO: Rename this from items
pub mod items {
    include!(concat!(env!("OUT_DIR"), "/datatypes.rs"));
}

use items::Arguments;
use items::Return;

/// General function to log messages
// #[link(wasm_import_module = "wapc")]
// extern "C" {
//     pub fn __log(ptr: *const u8, len: usize);
// }

// register the callback handler for the wapc calls
//wapc_handler!(handle_wapc);

pub fn handle_wapc(operation: &str, msg: &[u8]) -> CallResult {
    match operation {
        "contract" => handle_tx_invoke(msg),
       // could add other functions for administration
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
fn _runtime_host_call(cmd: String, data: Vec<u8>) -> String {
    log(&format!("Making host call{}::{}",cmd,str::from_utf8(&data).unwrap())[..]);
    let res = host_call("wapc", &cmd[..],&data).unwrap();
    format!("{:?}",res)
}

/// handle_tx_invoke called with the buffer that contains the request 
/// of what transaction function should be invoked
fn handle_tx_invoke(msg: &[u8]) -> CallResult {
    log("handler_tx_invoke");  

    let ctx = Context::new(log);

    // decode the message and arguments
    let args = Arguments::decode(msg).unwrap();
    log(&args.args.join(","));
    
    let operation = args.fnname;
    log(&operation[..]);

    // pass over to the contract manager to route
    log("making the routing calll");
    let _r = ContractManager::route(ctx,String::from("AssetContract"),operation,args.args);
    
    log("afterTransactions");
    let ret = Return {
        data : String::from("Hello"),
        code: 200,
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
