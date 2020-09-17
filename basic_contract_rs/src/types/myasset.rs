/*
 * SPDX-License-Identifier: Apache-2.0
 */

//! Very simple custom Asset object

// Use the required crates
use fabric_contract::contract::*;
use fabric_contract::data::*;
use serde::{Deserialize, Serialize};
use std::str::from_utf8;
/// Basic definition of the asset object
/// 
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MyAsset {
    id: String,
    value: String
}

/// Standard Implementation
impl MyAsset {

    pub fn new(id: String, value: String) -> MyAsset {
        MyAsset {
            id,
            value,
        }
    }

    pub fn get_value(&self) -> String {
        self.value.clone()
    }
}

/// The DataType trait must be implemented for this struct to be handled
/// by the contract
impl DataType for MyAsset {
    fn to_state(&self) -> State {
        let json = serde_json::to_string(self).unwrap();
        let buffer = json.into_bytes();
        State::from((MyAsset::form_key(&self.id.clone()), buffer))
    }

    fn get_key(&self) -> String {
       MyAsset::form_key(&self.id)
    }

    fn build_from_state(state: State) -> Self {
        let b = state.value();

        let str = match from_utf8(&b) {
            Ok(a) => a,
            Err(_) => panic!("Err"),
        };
        serde_json::from_str(str).unwrap()
    }

    fn form_key(k: &String) -> String {
        format!("MyAsset::{}",k)
    }
}

impl WireBufferFromReturnType<MyAsset> for WireBuffer {
    fn from_rt(self: &mut Self, s: MyAsset) {
        // we've got a wire buffer object and we need to set the bytes here from the string
        let json = serde_json::to_string(&s).unwrap();
        
        let buffer = json.into_bytes();
        self.buffer = Some(buffer);
    }
}

impl From<&WireBuffer> for MyAsset {
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
