/*
 * SPDX-License-Identifier: Apache-2.0
 */

///
/// A State is the combination of key and value that are contained within a collection.
///
/// State-based endorsement should be set on this object as well.
///
///  Represents the things that are contained within the Collections.
///
pub struct State {
    key: String,
    data: Vec<u8>,
}

impl State {
    /// Make a composite key.
    ///
    /// # Remarks
    /// 
    /// Takes a set of strings and arranges them in a ':' separate single string
    /// 
    /// # Arguments
    ///
    /// - `keytype`     The first part of the key. Referred to as 'type' but is just the 1st string
    ///                 part of the key.
    /// - `attributes`  Vector of Strings to make the rest of the ky
    pub fn make_composite_key(keytype: String, attritbutes: Vec<String>) -> String {
        todo!("make key");
    }

    pub fn split_composite_key(_type: String) -> Vec<String> {
        todo!("make key");
    }

    pub fn new(key: String, data: Vec<u8>) -> State {
        State { key, data }
    }

    pub fn value(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn key(&self) -> String {
        self.key.clone()
    }

    pub fn get_history() {}

    pub fn get_hash() -> Vec<u8> {
        todo!("get_hash")
    }

    pub fn set_endorsement() {}

    pub fn get_endorsement() {}
}

pub trait DataType {
    fn hello_macro();
}
