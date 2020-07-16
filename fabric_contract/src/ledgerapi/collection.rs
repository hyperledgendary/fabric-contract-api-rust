/*
 * SPDX-License-Identifier: Apache-2.0
 */
#![allow(unused_variables)]
#![allow(dead_code)]

use crate::ledgerapi::state::*;
use crate::{error::LedgerError, runtimeapi::ledgerservice::*};
use super::DataType;

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

    pub fn create<T>(&self, asset:T ) -> Result<State,LedgerError> where T:  DataType {
        let s = asset.to_state();
        LedgerService::create_state(s.key(), s.value())
    }

    pub fn retrieve<T>(&self, key: String) -> Result<T,LedgerError> where T: Default + DataType  {        
        let s = LedgerService::read_state(key).unwrap();
        let mut asset = T::default();
        asset.from_state(s);
        Ok(asset)
    }

    /// Does this key exist
    pub fn state_exists(&self, key: &String) -> Result<bool,LedgerError>  {
        LedgerService::exists_state(key)
    }
    
    /// Return the state for this key
    /// 
    pub fn retrieve_state(&self, key: String) -> Result<State,LedgerError> {
        LedgerService::read_state(key)
    }

    /// Creates the state
    /// 
    /// If it it already exists, this is an error
    pub fn create_state(&self, key: String, data: Vec<u8>) -> Result<State,LedgerError> {     
       LedgerService::create_state(key,data)
    }

    /// Update the states
    /// 
    /// If it doesn't exist, this is an error
    pub fn update_state(&self, key: String, data: Vec<u8>) -> Result<State,LedgerError> {
       LedgerService::update_state(key, data)
    }

    /// Deletes the key 
    pub fn delete_state(&self, key: String) -> Result<(),LedgerError>  {
        LedgerService::delete_state(key)
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
    pub fn get_states(handler: KeyQueryHandler) -> Result<(),LedgerError> {
         todo!("getstates");
    //     // https://users.rust-lang.org/t/how-to-return-an-iterator/25133/3
    }

    pub fn query_states(handler: RichQueryHandler) -> Result<(),LedgerError>  {
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