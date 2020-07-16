
/*
 * SPDX-License-Identifier: Apache-2.0
 */

use fabric_contract::data::*;
use fabric_contract::contract::*;


pub struct MyAsset {
   uid: Option<String>,
   value: Option<String>
}

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

impl Default for MyAsset {
    fn default() -> Self {
        MyAsset { uid : Option::None, value: Option::None }
    }   
}

impl From<State> for MyAsset {
    fn from(s: State) -> Self {
      let key = s.key();
      let value: String = String::from_utf8(s.value()).unwrap();
      MyAsset::new(key,value)
    }
}

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

