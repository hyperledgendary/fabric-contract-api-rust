/*
 * SPDX-License-Identifier: Apache-2.0
 */

//! This is the main crate for providing support for writing Smart Contracts to work with 
//! Hyperledger Fabric. 
//! 
//! Any rust contract must be compiled to target Wasm. They can then be run in the Wasm chaincode
//! 
//! In addition to this crate, there is the `contract_macros` crate that contains the macros needed
//! 

mod contractapi;
mod ledgerapi;
mod blockchainapi;
mod runtimeapi;
pub mod error;
pub use error::*;

/// Module to provide 'runtime' services.
/// 
/// Services available are
/// 
/// - Registering the contracts available at start
/// - Logging
/// 
pub mod runtime {
    pub use crate::runtimeapi::ContractRuntime as ContractRuntime;
}

/// Module to provide APIs to write contracts and interact with the ledger
/// 
/// TODO: Split this into two modules?
///
pub mod contract {
    pub use crate::ledgerapi::ledger::Ledger as Ledger;
    pub use crate::contractapi::contract::Contract as Contract;
    pub use crate::contractapi::contractmanager::ContractManager as ContractManager;
    pub use crate::contractapi::contract::Routing as Routing;
    pub use crate::contractapi::context::Context as Context;
    pub use crate::ledgerapi::collection::Collection as Collection;
    pub use crate::ledgerapi::collection::CollectionName as CollectionName;
}

/// Module to provide APIs to get information about Fabric
/// 
/// It is not intended to be full chain access utility but the important
/// aspects required for 
/// 
/// - Current executing transactions
/// - Events to be added to the read/write set of the tranasction
/// - Invoking chaincode on other channels
pub mod blockchain {
    pub use crate::blockchainapi::transaction::Transaction as Transaction;
}