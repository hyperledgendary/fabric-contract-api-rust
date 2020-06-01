/*
 * SPDX-License-Identifier: Apache-2.0
 */

#![allow(dead_code)]
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
///   let tx = Transaction::current_transaction()
/// 
///   let id = tx.get_id();
///   let timestamp = tx.get_timestamp();
///   let mspid = tx.get_mspid();
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
    pub fn get_mspid(&self) -> String {
        todo!("get_id")
    }

    /// Get the current transaction
    pub fn current_transaction() -> Transaction {
        todo!("get_id")
    }
}