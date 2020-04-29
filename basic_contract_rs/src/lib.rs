/*
 * SPDX-License-Identifier: Apache-2.0
 */

//! Main library module 
//! 
//! The purpose of this is to do register the contracts.
//! 
//! Other setup can take place here but do so with
//! caution. Any failure will terminate the container
//! and there is limited scope for logging at stage in
//! the lifecyle
mod assetcontract;

use fabric_contract::contractapi::contractmanager::*;
use assetcontract::AssetContract;

// The macro to indicate the function that will be used to register the contracts.
use fabric_contract::launch_handler;


launch_handler!(register);

/// Register the contracts
/// Look to fold this into the macro (or the contract_impl macro)
pub fn register() -> i32 {
    ContractManager::register_contract(Box::new(AssetContract::new()));

    // return 0 to indicate success, other values imply failure and will terminate
    // the chaincode container
    0
}