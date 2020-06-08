/*
 * SPDX-License-Identifier: Apache-2.0
 */
#![allow(unused_variables)]
#![allow(dead_code)]

use crate::ledgerapi::state::*;
use crate::ledgerapi::datatype::*;


use crate::runtimeapi::ledgerservice::*;

/// Collection Name
/// 
/// An enumeration that be one of 
/// 
/// - World for the ledger's world state
/// - Private(<string>) for a private data collection with the given name
/// - Organization<mspid> for an organizations implicit private data collection
/// 
/// Note for the ogranization the underlying prefix of _implict_org is automatically added
pub enum CollectionName {
    World, Private(String), Organization(String)
}

/// Key Queryhandler
/// 
/// Enumeration to define the Key Range queries that can take place. 
/// TODO: Check if these are inclusive or execlusice
/// TODO: Not included all as that could return just too much data
/// 
pub enum KeyQueryHandler {
   /// Range(string,string) The start and end keys  
   Range(String, String),

   /// RangeFrom(string)    From the given key to the end
   RangeFrom(String),

   /// RangeTo(string)      From the start to the given key 
   RangeTo(String)
}

/// Specify the Rich Query Handler
pub enum RichQueryHandler {
    /// The query string to pass to the state database (currently only supported for CouchDB)
    Query(String)
}

pub struct Collection {
    name: CollectionName
}

impl Collection {

    pub fn new() -> Collection {
        Collection {
            name: CollectionName::World,
        }
    }

    pub fn retrieve<T: DataType>(&self, key: &String) -> Result<T,String> {
         todo!("getstate")
    }

    pub fn create<T: DataType>(&self, value: T) -> Result<State,String> {
         
        let s = value.to_state();
        self.create_state(s.key(), s.value())

    }

    pub fn update<T: DataType>(&self, _value: T) -> Result<State,String> {
         todo!("getstate")
    }

    pub fn delete<T: DataType>(&self, _value: T) -> Result<(),String> {
         todo!("getstate")
    }


    /// Does this key exist
    pub fn state_exists(&self, key: &String) -> bool {
        LedgerService::exists_state(key).unwrap()
    }
    
    /// Return the state for this key
    /// 
    pub fn get_state(&self, key: String) -> Result<State,String> {
        LedgerService::read_state(key)
    }

    /// Creates the state
    /// 
    /// If it it already exists, this is an error
    pub fn create_state(&self, key: String, data: Vec<u8>) -> Result<State,String> {     
       LedgerService::create_state(key,data)
    }

    /// Update the states
    /// 
    /// If it doesn't exist, this is an error
    pub fn update_state(&self, key: String, data: Vec<u8>) -> State {
        todo!("update")
    }

    /// Deletes the key 
    pub fn delete_state(&self, key: String) -> State {
        todo!("update")
    }

    /// Performs a key range query
    /// 
    /// # Example
    /// 
    /// 
    /// use fabric_contract::contract::*;
    /// 
    /// let collection = Ledger::access_ledger().get_collection(CollectionName::World); 
    /// collection.get_states(KeyQueryHandler::Range("Car001","Car002"));
    /// 
    pub fn get_states(handler: KeyQueryHandler) -> String{
         todo!("getstates");
    //     // https://users.rust-lang.org/t/how-to-return-an-iterator/25133/3
    }

    pub fn query_states(handler: RichQueryHandler) -> String {
        todo!("getstates");
    }
}

/// Collection Iterator
/// 
/// Standard Rust iterator over the returned states
pub trait CollectionIterator : Iterator {

    /// sets the paging size
    fn set_paging_size(pagesize: u32);

    /// number of fetched states
    fn get_fetched_count() -> u32;

    /// set the bookmark to a previous returned value
    fn set_bookmark(bookmark: String);

    /// get currentmark
    fn get_bookmark() -> String;

    // close
    // hope this can be done automatiacally....
    // 
}