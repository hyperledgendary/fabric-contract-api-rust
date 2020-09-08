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
<<<<<<< HEAD
    id: String, 
    owner_org: String,
    public_description: String
}

impl Asset {
    pub fn new(id: String,owner_org: String,public_description:String) -> Asset {
        Asset {
            id,owner_org,public_description
=======
<<<<<<< HEAD:asset_transfer_rs/src/types/asset.rs
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
=======
    id: String, 
    owner_org: String,
    public_description: String
}

impl Asset {
    pub fn new(id: String,owner_org: String,public_description:String) -> Asset {
        Asset {
<<<<<<< HEAD
            id,owner_org,public_description,on_the_market
>>>>>>> Examples update:asset_transfer_secure_private_rs/src/types/asset.rs
>>>>>>> Examples update
=======
            id,owner_org,public_description
>>>>>>> Update secure transfer
        }
    }

    pub fn get_owner(&self) -> String {
        self.owner_org.clone()
    }

    pub fn update_owner(&mut self, owner: String) -> () {
<<<<<<< HEAD
=======
<<<<<<< HEAD:asset_transfer_rs/src/types/asset.rs
        self.owner = owner;
    }

    pub fn get_color(&self) -> String {
        return self.color.clone();
    }

    pub fn get_id(&self) -> String {
        return self.id.clone();
=======
>>>>>>> Examples update
        self.owner_org = owner;
    }

    pub fn set_public_description(&mut self, desc: String) -> () {
        self.public_description = desc.clone();
    }

    pub fn get_public_description(&self) -> String {
        self.public_description.clone()
    }

<<<<<<< HEAD
<<<<<<< HEAD
    pub fn get_id(&self) -> String {
        self.id.clone()
=======

    pub fn is_on_market(&self) -> bool {
        self.on_the_market
    }

=======
>>>>>>> Update secure transfer
    pub fn get_id(&self) -> String {
        self.id.clone()
>>>>>>> Examples update:asset_transfer_secure_private_rs/src/types/asset.rs
>>>>>>> Examples update
    }
}

/// Very important to implement the DataType Trait for the Asset
///
/// This provides the ability to store the data in the ledger
impl DataType for Asset {
<<<<<<< HEAD
<<<<<<< HEAD

    /// Converts to a State
    fn to_state(&self) -> State {
        let json = serde_json::to_string(self).unwrap();       
=======
    fn to_state(&self) -> State {
        let json = serde_json::to_string(self).unwrap();
        debug!("ToState::{}",&json.as_str());
>>>>>>> Examples update
=======

    /// Converts to a State
    fn to_state(&self) -> State {
        let json = serde_json::to_string(self).unwrap();       
>>>>>>> Update secure transfer
        let buffer = json.into_bytes();
        State::from((self.id.clone(), buffer))
    }

<<<<<<< HEAD
<<<<<<< HEAD
    /// Returns the key for this specific object as a string
    fn get_key(&self) -> String {
        Asset::form_key(&self.id)
=======
    fn get_key(&self) -> String {
<<<<<<< HEAD:asset_transfer_rs/src/types/asset.rs
        Asset::form_key(&self.id.clone())
=======
        self.id.clone()
>>>>>>> Examples update:asset_transfer_secure_private_rs/src/types/asset.rs
>>>>>>> Examples update
=======
    /// Returns the key for this specific object as a string
    fn get_key(&self) -> String {
        Asset::form_key(&self.id)
>>>>>>> Update secure transfer
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
<<<<<<< HEAD
<<<<<<< HEAD
    
    fn form_key(k: &String) -> String {
        format!("Asset#{}",k)
=======

<<<<<<< HEAD:asset_transfer_rs/src/types/asset.rs
    fn form_key(k: &String) -> String {
       format!("Asset::{}",k)
=======
/// Implementing the Default trait
///
/// Consider using a 'builder' style api on the DataTYpe above
impl Default for Asset {
    fn default() -> Self {
        Asset {
            id: "".to_string(),
            public_description: "".to_string(),
            on_the_market: false,
            owner_org: "".to_string()
        }
>>>>>>> Examples update:asset_transfer_secure_private_rs/src/types/asset.rs
>>>>>>> Examples update
=======
    
    fn form_key(k: &String) -> String {
        format!("Asset#{}",k)
>>>>>>> Update secure transfer
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