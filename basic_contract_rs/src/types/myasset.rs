
/*
 * SPDX-License-Identifier: Apache-2.0
 */

use fabric_contract::data::DataType;
use fabric_contract::contract::State;

pub struct MyAsset {
   
   value: String
}

impl MyAsset {
   pub fn new(value: String) -> MyAsset {
      MyAsset {
         value
      }
   }

   pub fn get_value(&self) -> String {
      self.value.clone()
   }
}


impl DataType for MyAsset {
   
   fn to_state(&self)-> &State{
      todo!("to_state")
   }
   fn get_key(&self) -> String{
      todo!("getkey")
   }
}