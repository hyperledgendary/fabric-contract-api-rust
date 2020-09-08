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
    id: String,
    appraised_value: u32,
    itemcode: String,
    description: String
}

impl AssetPrivate {
    pub fn new(id: String, appraised_value: u32,colour: String,itemcode:String, description: String) -> AssetPrivate {
        AssetPrivate {
            id,
            appraised_value,
            itemcode,
            description
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
        State::from((self.id.clone(), buffer))
    }

    fn get_key(&self) -> String {
        self.id.clone()
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
        todo!()
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

impl From<&WireBuffer> for AssetPrivate {
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