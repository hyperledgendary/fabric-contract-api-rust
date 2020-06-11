
/*
 * SPDX-License-Identifier: Apache-2.0
 */

use fabric_contract::data::*;
use fabric_contract::contract::*;

pub struct MyAsset {
   uid: String,
   value: String
}

impl MyAsset {
   pub fn new(uid: String, value: String) -> MyAsset {
      MyAsset {
         uid,
         value
      }
   }

   pub fn get_value(&self) -> String {
      self.value.clone()
   }

}


impl DataType for MyAsset {
   
   fn to_state(&self)-> State{      
      State::from((self.uid.clone(),self.value.clone().into_bytes()))
   }
   fn get_key(&self) -> String{
      self.uid.clone()
   }
}

// impl From<DataType> for State {
//    fn from(a: MyAsset) -> Self {

//       let key = a.get_key();
//       let data = format!("{{\"value\":\"{}\"}}",a.get_value());

//       Self::new(key , data.into() )
//    }
// }