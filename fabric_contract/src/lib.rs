/*
 * SPDX-License-Identifier: Apache-2.0
 */

#![allow(unused_variables)]

//! This is the main crate for providing support for writing Smart Contracts to work with
//! Hyperledger Fabric's Wasm chaincode runtime.
//!
//! The `fabric_contract_macros` crate contains the macros to assist with development of the Smart
//! Contracts
//!
//! `basic_contract_rs` shows a simple Asset contract.
//!

mod blockchainapi;
mod contractapi;
mod dataapi;
mod error;
mod ledgerapi;
mod runtimeapi;

pub use fabric_contract_macros::*;

/// Macro to use in the lib.rs file of your contract's crate
///
/// Should be called with the functions that create new instances of
/// the contract structures.
///
/// # Example
///
/// ```ignore
/// use crate::fabric_contract::*;
/// struct AssetContract {};
///
/// impl AssetContract {
///    pub fn new() -> AssetContract {
///        AssetContract {}
///    }
/// }
///
/// fabric_contract::register!( AssetContract::new );
/// ```
///
#[macro_export]
macro_rules! register {

    (  $( $contract:path ),*   )=> {

        use log::{debug};

        use fabric_contract::contract::ContractManager;
        use fabric_contract::prelude::*;

        use std::sync::Once;

        static START: Once = Once::new();

        pub fn __launch() {
            fabric_contract::runtime::init_logger();
            debug!("Logger setup and launched");
            

            $( ContractManager::register_contract(Box::new($contract())); )*
            debug!("Contracts registered");
        }

        pub fn once_wapc(operation: &str, msg: &[u8]) -> wapc_guest::prelude::CallResult {
            START.call_once(|| {
                 __launch();
            });
            handle_wapc(operation,msg)
        }


        // register the callback handler for the wapc calls
        wapc_guest::wapc_handler!(once_wapc);
    };
}

pub mod prelude {
    pub use crate::runtimeapi::wapc::handle_wapc;
    pub use crate::runtimeapi::wapc::log as host_log;
    pub use wapc_guest::prelude::*;

    pub use crate::contractapi::contractdefn::ContractDefn;
    pub use crate::contractapi::transaction::TransactionFn;
    pub use crate::contractapi::transaction::TransactionFnBuilder;
}

/// Module to use to define the complex datatypes
pub mod data {
    pub use crate::dataapi::typeschema::TypeSchema;
    pub use crate::dataapi::wirebuffer::WireBuffer;
    pub use crate::dataapi::wirebuffer::WireBufferFromReturnType;
    pub use crate::ledgerapi::datatype::DataType;
    pub use fabric_contract_macros::property as Property;
}

/// Module to provide 'runtime' services.
///
/// Services available are
///
/// - Registering the contracts available at start
/// - Logging
///
pub mod runtime {
    pub use crate::runtimeapi::logger::*;
}

/// Module to provide APIs to write contracts and interact with the ledger
///
/// TODO: Split this into two modules?
///
pub mod contract {
    pub use crate::contractapi::context::Context;
    pub use crate::contractapi::contract::Contract;
    pub use crate::contractapi::contract::Metadata;
    pub use crate::contractapi::contract::Routing;
    pub use crate::contractapi::contractmanager::ContractManager;
    pub use crate::ledgerapi::collection::Collection;
    pub use crate::ledgerapi::collection::CollectionName;
    pub use crate::ledgerapi::collection::KeyQueryHandler;
    pub use crate::ledgerapi::ledger::Ledger;
    pub use crate::ledgerapi::state::*;
    pub use crate::ledgerapi::statequerylist::StateQueryList;
    pub use crate::ledgerapi::endorsement::*;

    pub use fabric_contract_macros::contract_impl as Contract_Impl;
    pub use fabric_contract_macros::transaction as Transaction;

    pub use crate::error::ContractError;
    pub use crate::error::LedgerError;
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
    pub use crate::blockchainapi::clientidentity::ClientIdentity;
    pub use crate::blockchainapi::transaction::Transaction;
}
