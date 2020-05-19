
use serde::{Serialize, Deserialize};

// will add the to and from serialized form based on the requested
#[derive(Serialize,Deserialize)]  
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
