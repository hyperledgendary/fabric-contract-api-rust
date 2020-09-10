/*
 * SPDX-License-Identifier: Apache-2.0
 */
use fabric_contract::contract::*;
use fabric_contract::data::*;
use serde::{Deserialize, Serialize};
use std::str::from_utf8;

// Use the log crate to support logging
use log::{debug};

/// 

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Asset {
    id: String,
    color: String,
    size: i32,
    owner: String,
    appraised_value: i32,
}

impl Asset {
    pub fn new(id: String, color: String, size: i32, owner: String, appraised_value: i32) -> Asset {
        Asset {
            id,
            color,
            size,
            owner,
            appraised_value,
        }
    }

    pub fn get_owner(&self) -> String {
        self.owner.clone()
    }

    pub fn update_owner(&mut self, owner: String) -> () {
        self.owner = owner;
    }

    pub fn get_color(&self) -> String {
        return self.color.clone();
    }

    pub fn get_id(&self) -> String {
        return self.id.clone();
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
        State::from((self.id.clone(), buffer))
    }

    fn get_key(&self) -> String {
        Asset::form_key(&self.id.clone())
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


    fn form_key(k: &String) -> String {
       format!("Asset::{}",k)
    }
}

impl WireBufferFromReturnType<Asset> for WireBuffer {
    fn from_rt(self: &mut Self, s: Asset) {
        // we've got a wire buffer object and we need to set the bytes here from the string
        let json = serde_json::to_string(&s).unwrap();
        debug!("wire buffer returning the value {}",json.as_str());
        let buffer = json.into_bytes();
        self.buffer = Some(buffer);
    }
}

impl From<&WireBuffer> for Asset {
    fn from(wb: &WireBuffer) -> Self {
        match &wb.buffer {
            Some(buffer) => {
                match std::str::from_utf8(&buffer) {
                    Ok(a) => serde_json::from_str(a).unwrap(),
                    _ => unreachable!(),
                }
            }
            None => panic!(),
        }
    }
}