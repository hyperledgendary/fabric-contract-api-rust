/*
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::blockchainapi::clientidentity::ClientIdentity;
use fabric_ledger_protos::common_messages::TransactionContext as TxCtx;

/// Represents a transaction
///
/// This can be used to obtain information about the current transaction
/// that trigger a contract function to be executed.
///
/// Timestamp and MSPID are two important fields
///
/// # Example
///
/// ```
/// pub fn myTransactionFn(){
///   let tx = fabric_contract::blockchain::TransactionContext::current_transaction();
///
///   let id = tx.get_id();
///   let timestamp = tx.get_timestamp();
///   let mspid = tx.get_peer_mspid();
/// }
/// ```
///  
#[derive(Default,Clone)]
pub struct TransactionContext {
    tx_id: std::string::String,
    channel_id: std::string::String,
    // timestamp: String,

}

impl TransactionContext {
    pub fn new(ctx: &TxCtx) -> TransactionContext {
        TransactionContext {
            channel_id: ctx.channel_id.clone(),
            tx_id: ctx.transaction_id.clone(),
            // timestamp: 
        }
    }

    /// Gets the transaction id
    ///
    /// Full transaction id
    pub fn get_id(&self) -> String {
        self.tx_id.clone()
    }

    /// Gets the timestamp of this transaction
    ///
    /// Format is ISO - need better type here
    pub fn get_timestamp(&self) -> String {
        todo!("get_id")
    }

    pub fn get_channelid(&self) -> std::string::String {
        self.channel_id.clone()
    }

    /// The MSP Identifier of the originating organization
    ///
    pub fn get_peer_mspid(&self) -> String {
        todo!("get_id")
    }

    /// Get
    pub fn get_submitting_identity(&self) -> Result<ClientIdentity, String> {
        todo!("get_submitting_identity");
    }

    /// Get the current transaction
    pub fn current_transaction() -> TransactionContext {
        todo!("get_id")
    }
}
