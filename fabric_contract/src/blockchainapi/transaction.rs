/*
 * SPDX-License-Identifier: Apache-2.0
 */

/// Represents a transaction 
///  
pub struct Transaction {

}

impl Transaction {

    fn new() -> Transaction {
        Transaction{

        }
    }


    /// Gets the transaction id
    pub fn get_id(&self) -> String {
        todo!("get_id")
    }

    pub fn get_timestamp(&self) -> String {
        todo!("get_id")
    }

    pub fn get_mspid(&self) -> String {
        todo!("get_id")
    }

    pub fn current_transaction() -> Transaction {
        todo!("get_id")
    }
}