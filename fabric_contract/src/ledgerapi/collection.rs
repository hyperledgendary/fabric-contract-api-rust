/*
 * SPDX-License-Identifier: Apache-2.0
 */
#![allow(unused_variables)]
#![allow(dead_code)]

use super::DataType;
use crate::ledgerapi::{state::*, statequerylist::*};
use crate::{error::LedgerError, runtimeapi::ledgerservice::*};


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
    World,
    Private(String),
    Organization(String),
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
    RangeTo(String),

    /// RangeAll(),  All composite keys. use with caution
    RangeAll()
}

/// Specify the Rich Query Handler
pub enum RichQueryHandler {
    /// The query string to pass to the state database (currently only supported for CouchDB)
    Query(String),
}

pub struct Collection{
    name: CollectionName,
}

impl Collection {
    pub fn new(name: CollectionName) -> Self {
        Collection {
           name,
        }
    }

    pub fn create<T>(&self, asset: T) -> Result<State, LedgerError>
    where
        T: DataType,
    {
        let s = asset.to_state();
        LedgerService::create_state(s.key(), s.value())
    }

    pub fn retrieve<T>(&self, key: &String) -> Result<T, LedgerError>
    where
        T: Default + DataType,
    {
        let s = LedgerService::read_state(&T::form_key(key)).unwrap();
        // let mut asset = T::default();
        // asset.from_state(s);     
        Ok(T::build_from_state(s))
    }

    pub fn retrieve_hash<T>(&self, key: &String) -> Result<T, LedgerError>
    where
        T: Default + DataType,
    {
        let s = LedgerService::read_state( &T::form_key(key)).unwrap();
        // let mut asset = T::default();
        // asset.from_state(s);     
        Ok(T::build_from_state(s))
    }


    pub fn update<T>(&self, asset: T) -> Result<State, LedgerError> where T: DataType,
    {
        let s = asset.to_state();
        LedgerService::update_state(s.key(), s.value())
    }

    /// Does this key exist
    pub fn state_exists(&self, key: &str) -> Result<bool, LedgerError> {
        LedgerService::exists_state(&key.to_string())
    }

    /// Return the state for this key
    ///
    pub fn retrieve_state(&self, key: &String) -> Result<State, LedgerError> {
        LedgerService::read_state(&key)
    }

    /// Return the state has ONLY for this key
    ///
    pub fn retrieve_state_hash(&self, key: &String) -> Result<State, LedgerError> {
       todo!()
    }

    /// Creates the state
    ///
    /// If it it already exists, this is an error
    pub fn create_state(&self, key: String, data: Vec<u8>) -> Result<State, LedgerError> {
        LedgerService::create_state(key, data)
    }

    /// Update the states
    ///
    /// If it doesn't exist, this is an error
    pub fn update_state(&self, key: String, data: Vec<u8>) -> Result<State, LedgerError> {
        LedgerService::update_state(key, data)
    }

    /// Deletes the key
    pub fn delete_state(&self, key: &String) -> Result<(), LedgerError> {
        LedgerService::delete_state(&key)
    }

    /// Performs a key range query
    ///
    /// # Example
    ///
    /// ```ignore
    /// use fabric_contract::contract::*;
    ///
    /// let collection = Ledger::access_ledger().get_collection(CollectionName::World);
    /// collection.get_states(KeyQueryHandler::Range("Car001","Car002"));
    ///
    pub fn get_states(&self,handler: KeyQueryHandler) -> Result<StateQueryList, LedgerError> {

        let states = match handler {
            KeyQueryHandler::Range(start_key,end_key) => LedgerService::get_range_states(start_key.as_str(), end_key.as_str()),
            KeyQueryHandler::RangeTo(end_key) => LedgerService::get_range_states("", end_key.as_str()),
            KeyQueryHandler::RangeFrom(start_key) => LedgerService::get_range_states(start_key.as_str(), ""),
            KeyQueryHandler::RangeAll() => LedgerService::get_range_states("", ""),
        }?;
        Ok(StateQueryList::new(states))
    }

    pub fn query_states(&self,handler: RichQueryHandler) -> Result<(), LedgerError> {
        todo!("getstates");
    }
}

/// Collection Iterator
///
/// Standard Rust iterator over the returned states
pub trait CollectionIterator: Iterator {
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
