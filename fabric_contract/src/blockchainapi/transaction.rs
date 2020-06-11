/*
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::blockchainapi::clientidentity::ClientIdentity;


/// Represents a transaction 
/// 
/// This can be used to obtain information about the current transaction
/// that trigger a contract function to be executed.
/// 
/// Timestamp and MSPID are two important fields
/// 
/// TODO: Check for others to be added
/// 
/// # Example
/// 
/// ```
/// pub fn myTransactionFn(){
///   let tx = fabric_contract::blockchain::Transaction::current_transaction();
/// 
///   let id = tx.get_id();
///   let timestamp = tx.get_timestamp();
///   let mspid = tx.get_peer_mspid();
/// }
/// ```
///  
pub struct Transaction {

}

impl Transaction {

    fn new() -> Transaction {
        Transaction{

        }
    }

    /// Gets the transaction id
    /// 
    /// Full transaction id
    pub fn get_id(&self) -> String {
        todo!("get_id")
    }

    /// Gets the timestamp of this transaction
    /// 
    /// Format is ISO - need better type here
    pub fn get_timestamp(&self) -> String {
        todo!("get_id")
    }

    /// The MSP Identifier of the originating organization
    /// 
    pub fn get_peer_mspid(&self) -> String {
        todo!("get_id")
    }

    /// Get 
    pub fn get_submitting_identity(&self) -> Result<ClientIdentity,String> {
        todo!("get_submitting_identity");
    }

    /// Get the current transaction
    pub fn current_transaction() -> Transaction {
        todo!("get_id")
    }
}