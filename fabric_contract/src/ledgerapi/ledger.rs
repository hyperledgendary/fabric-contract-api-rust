/*
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::ledgerapi::collection::*;


///
/// Ledger representing high level concept of the Fabric Blockchain network
/// 
/// Provides the access points for 
///  - getting access to data in both world state and peer's private collections
///  - invoking chaincode on this and other networks
/// 
/// When a transaction is invoked, it takes place under a 'transactional context'
/// This context can be queried to provide the current transaction id, and information 
/// about the identity requesting the transaction.
/// 
/// All operations need to be done under this transactional context, therefore
/// the context needs to be passed to obtain the Ledger instance
/// 
/// # Example
/// 
/// ```
/// pub fn create_asset() -> Result<(),String> {
/// 
/// pub fn asset_exists(my_assset_id: String) -> Result<bool,String> {
///    let ledger = Ledger::access_ledger();
///    
///    let world = ledger.get_collection(CollectionName::World);
///
///    Ok(world.state_exists(my_assset_id))
///
/// }
///  
pub struct Ledger {

}

impl Ledger {

    /// Get the Ledger based on the current transactional context
    /// 
    /// The Tranasctional Context is available via the [Transaction API](../../transaction/struct.Transaction.html)
    /// 
    pub fn access_ledger() -> Ledger {
       Ledger::new()
    }
    
    fn new() -> Ledger {
        Ledger {
           
        }
    }
    
    /// Return the collection based on the name
    /// 
    /// The collection provides access to the put & get Fabric
    /// semantics of the underlying world state, or private data
    /// that is specified by the Collection name. 
    /// 
    /// # Example
    /// ```
    ///    // get the collectin that is backed by the World State
    ///    let world = ledger.get_collection(CollectionName::World);  
    /// 
    ///    // get the collection that is backed by the Organization's Implicity Private Data Collection
    ///    let orgs_collection = ledger.get_collection(CollectionName::Organization(String::from("org1")));
    ///    
    ///    // get the collection that is backed by the named Private Data Collection
    ///    let private_collection = ledger.get_collection(CollectionName::Private(String::from("my_private_details")));
    /// ```
    pub fn get_collection(&self, name: CollectionName) -> Collection {
         todo!("get_collection")
    }   

}