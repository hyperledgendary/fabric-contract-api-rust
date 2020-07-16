
/*
 * SPDX-License-Identifier: Apache-2.0
 */

//! Very simple custom Asset object

// Use the required crates
use fabric_contract::data::*;
use fabric_contract::contract::*;

/// Basic definition of the asset object
/// 
/// Using Options here as this makes is possible to use the 
/// Default trait to create a blank object. We may in future consider
/// adding another method to the DataType trait instead; we may be able
/// to remove the use of Option
pub struct MyAsset {
   uid: Option<String>,
   value: Option<String>
}

/// Standard Implementation
impl MyAsset {
   pub fn new(uid: String, value: String) -> MyAsset {
      MyAsset {
         uid: Some(uid),
         value: Some(value)
      }
   }

   pub fn get_value(&self) -> String {
      self.value.clone().unwrap()
   }

}

/// TODO: Do we need both this and the from_state below?
impl From<State> for MyAsset {
    fn from(s: State) -> Self {
      let key = s.key();
      let value: String = String::from_utf8(s.value()).unwrap();
      MyAsset::new(key,value)
    }
}

/// The DataType trait must be implemented for this struct to be handled
/// by the contract
impl DataType for MyAsset {
   
   fn to_state(&self)-> State{      
      State::from((self.uid.clone().unwrap(),self.value.clone().unwrap().into_bytes()))
   }

   fn get_key(&self) -> String{
      self.uid.clone().unwrap()
   }

   fn from_state(&mut self, state: State) {
      self.uid = Some(state.key());
      self.value = Some(String::from_utf8(state.value()).unwrap());
    }

}

/// Implementing the Default trait
/// 
/// Consider using a 'builder' style api on the DataTYpe above
impl Default for MyAsset {
   fn default() -> Self {
       MyAsset { uid : Option::None, value: Option::None }
   }   
}
