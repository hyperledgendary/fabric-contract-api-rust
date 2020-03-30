use std::collections::HashMap;
use crate::contractapi::contract::*;

pub struct ContractRuntime {
   contracts: HashMap<String,Box<dyn Contract>>,
}

impl ContractRuntime {
    pub fn new() -> ContractRuntime {
        ContractRuntime {
           contracts: HashMap::new(),
        }
    }

   pub fn register(&mut self, contract: Box<dyn Contract>) {
        println!("Got contract passed in {:?}",contract.name());
        self.contracts.insert(contract.name(), contract);
       
   }

   pub fn start(&mut self){
       for (key,value) in &self.contracts {
           println!("{}={}",key,value.name());
       }
   }
}


