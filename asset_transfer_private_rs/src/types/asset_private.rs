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
pub struct AssetPrivate {
    ID: String,
    appraised_value: u32,
}

impl AssetPrivate {
    pub fn new(ID: String, appraised_value: u32) -> AssetPrivate {
        AssetPrivate {
            ID,
            appraised_value
        }
    }
}

/// Very important to implement the DataType Trait for the Asset
///
/// This provides the ability to store the data in the ledger
impl DataType for AssetPrivate {
    fn to_state(&self) -> State {
        let json = serde_json::to_string(self).unwrap();
        debug!("ToState::{}",&json.as_str());
        let buffer = json.into_bytes();
        State::from((self.ID.clone(), buffer))
    }

    fn get_key(&self) -> String {
        self.ID.clone()
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

impl WireBufferFromReturnType<AssetPrivate> for WireBuffer {
    fn from_rt(self: &mut Self, s: AssetPrivate) {
        // we've got a wire buffer object and we need to set the bytes here from the string
        let json = serde_json::to_string(&s).unwrap();
        debug!("wire buffer returning the value {}",json.as_str());
        let buffer = json.into_bytes();
        self.buffer = Some(buffer);
    }
}