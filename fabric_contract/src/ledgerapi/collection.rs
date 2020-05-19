/*
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::ledgerapi::state::*;

/// CollectionName
/// Can be either the 
pub enum CollectionName {
    World, Private(String), Organization(String)
}

pub enum KeyQueryHandler {
    Range(String, String), RangeFrom(String), RangeTo(String)
}

pub struct Collection {
    name: CollectionName
}

impl Collection {

    // pub fn retrieve(&self, _key: String) -> Result<T,String> {
    //     todo!("getstate")
    // }

    // pub fn create(&self, _value: T) -> Result<State<T>,String> {
    //     todo!("getstate")
    // }

    // pub fn update(&self, _value: T) -> Result<State<T>,String> {
    //     todo!("getstate")
    // }

    // pub fn delete(&self, _value: T) -> Result<(),String> {
    //     todo!("getstate")
    // }

    pub fn new(name: CollectionName) -> Collection {
        Collection {
            name
        }
    }

    pub fn state_exists(&self, key: String) -> bool {
        true
    }
    
    pub fn get_state(&self, key: String) -> State {
        todo!("getstate")
    }

    pub fn create_state(&self, key: String, data: Vec<u8>) -> State { 
       State::new(key,data)
    }

    pub fn update_state(&self, key: String, data: Vec<u8>) -> State {
        todo!("update")
    }

    pub fn delete_state(&self, key: String) -> State{
        todo!("update")
    }

    // pub fn get_states(handler: KeyQueryHandler) -> CollectionIterator {
    //     todo!("getstates");
    //     // https://users.rust-lang.org/t/how-to-return-an-iterator/25133/3
    // }
}

pub trait CollectionIterator : Iterator {

    fn set_paging_size(pagesize: u32);
    fn get_fetched_count() -> u32;

    fn set_bookmark(bookmark: String);
    fn get_bookmnark() -> String;

    // close.... hope this can be done automatiacally....
}