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
    pub fn make_composite_key(keytype: String, attributes: Vec<String>) -> String {
        todo!("make key");
    }


    /// Splits a composite key
    /// 
    /// Splits the composite key into the parts, the first is the 
    /// string that is passed in as the keytype above
    pub fn split_composite_key(key: String) -> Vec<String> {
        todo!("make key");
    }

    /// Creates a new state
    pub fn new(key: String, data: Vec<u8>) -> State {
        State { key, data }
    }

    /// Get the buffer that  this state 
    pub fn value(&self) -> Vec<u8> {
        self.data.clone()
    }

    /// Get the key that used for this state
    pub fn key(&self) -> String {
        self.key.clone()
    }

    /// Returns an iterator of the state history for this state
    pub fn get_history() /* -> TODO  Iterator of StateHistory */{

    }

    /// gets the private hash for this stae
    pub fn get_hash() -> Vec<u8> {
        todo!("get_hash")
    }

    /// Sets the State Based Endorsment for this state
    pub fn set_endorsement(/* TODO */) /* TODO */ {

    }

    pub fn get_endorsement() /* TODO */ {
         
    }
}

impl Default for State  {
    fn default()-> Self {
        State { key : "".to_string(), data : vec!() }
    }
}

impl ToState for State {
    #[inline]
    fn to_state(&self) -> &State {
        self.to_owned()
    }
}

impl From<()> for State {
    fn from(_:()) -> Self {
        Self::default()
    }
}

impl From<(String,Vec<u8>)> for State {
    fn from((a, b): (String, Vec<u8>)) -> Self {
        Self { key: a, data: b }
    }
}

pub trait ToState {
    fn to_state(&self) -> &State;
}

fn temp() {
    let s = State::new("Key".to_string(),"Value".to_string().into_bytes());
}
