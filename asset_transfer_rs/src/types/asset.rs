/*
 * SPDX-License-Identifier: Apache-2.0
 */
use fabric_contract::contract::*;
use fabric_contract::data::*;
use serde::{Deserialize, Serialize};
use std::str::from_utf8;

// Use the log crate to support logging
use log::{debug};

#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    ID: String,
    Color: String,
    Size: i32,
    Owner: String,
    AppraisedValue: i32,
}

impl Asset {
    pub fn new(ID: String, Color: String, Size: i32, Owner: String, AppraisedValue: i32) -> Asset {
        Asset {
            ID,
            Color,
            Size,
            Owner,
            AppraisedValue,
        }
    }

    pub fn update_owner(&mut self, owner: String) -> () {
        self.Owner = owner;
    }
}
/// Very important to implement the DataType Trait for the Asset
///
/// This provides the ability to store the data in the ledger
impl DataType for Asset {
    fn to_state(&self) -> State {
        let json = serde_json::to_string(self).unwrap();
        debug!("ToState::{}",&json.as_str());
        let buffer = json.into_bytes();


        State::from((self.ID.clone(), buffer))
    }
    fn get_key(&self) -> String {
        self.ID.clone()
    }
    fn from_state(&mut self, state: State) {
        let b = state.value();
        let str = match from_utf8(&b) {
            Ok(a) => a,
            Err(_) => panic!("Err"),
        };
        debug!("FromState:: {}",&str);
        match serde_json::from_str(str) {
            Ok(Asset {
                ID,
                Color,
                Size,
                Owner,
                AppraisedValue,
            }) => {
                self.ID = ID;
                self.Color = Color;
                self.Size = Size;
                self.Owner = Owner;
                self.AppraisedValue = AppraisedValue;
            }
            Err(_) => panic!("Err"),
        }
    }

    fn build_from_state(state: State) -> Self {
        let b = state.value();

        let str = match from_utf8(&b) {
            Ok(a) => a,
            Err(_) => panic!("Err"),
        };
        debug!("build_from_state:: {}",&str);
        serde_json::from_str(str).unwrap()
    }
}


/// Implementing the Default trait
///
/// Consider using a 'builder' style api on the DataTYpe above
impl Default for Asset {
    fn default() -> Self {
        Asset {
            ID: "".to_string(),
            Color: "".to_string(),
            Size: -1,
            Owner: "".to_string(),
            AppraisedValue: -1,
        }
    }
}
// impl From<DataType> for State {
//    fn from(a: MyAsset) -> Self {

//       let key = a.get_key();
//       let data = format!("{{\"value\":\"{}\"}}",a.get_value());

//       Self::new(key , data.into() )
//    }
// }
impl WireBufferFromReturnType<Asset> for WireBuffer {
    fn from_rt(self: &mut Self, s: Asset) {
        // we've got a wire buffer object and we need to set the bytes here from the string
        let json = serde_json::to_string(&s).unwrap();
        debug!("wire buffer returning the value {}",json.as_str());
        let buffer = json.into_bytes();
        self.buffer = Some(buffer);
    }
}