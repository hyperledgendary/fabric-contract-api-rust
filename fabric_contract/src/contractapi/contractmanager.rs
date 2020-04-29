/*
 * SPDX-License-Identifier: Apache-2.0
 */

#![allow(dead_code)]
use std::collections::HashMap;
use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;
use crate::contractapi::contractdefn;
use crate::contractapi::contract::*;
use crate::contractapi::context::*;


// Static reference to the ContractManager
lazy_static! {
    static ref CONTRACT_MGR: Mutex<ContractManager> = Mutex::new(ContractManager::new());
}

/// Contract Manager structure that holds the list contract objects
pub struct ContractManager {
    contracts: HashMap<String,contractdefn::ContractDefn>,
}

impl ContractManager {

    pub fn new() -> ContractManager {
        ContractManager {
           contracts: HashMap::new(),
        }
    }

    fn register_contract_impl(self: &mut ContractManager, contract: Box<dyn Contract + Send>) {
        let name = contract.name();
        let contract_defn = contractdefn::ContractDefn::new(contract);

        self.contracts.insert(name,contract_defn);
    }

    fn evaluate2(self: &mut ContractManager, ctx: Context, contract_name: String, tx: String, args: Vec<String>) -> Result<String,String> {
        ctx.log(format!("{}",self.contracts.len()));        
        match self.contracts.get(&contract_name) {
            Some(defn) => {
                ctx.log(String::from("Found defn"));
                let _r = defn.invoke(ctx, tx,args);
                Ok(String::from("called"))
            },
            None => {
                ctx.log(format!("Failed {}.{},{:?}",contract_name,tx,args));
                Err(String::from("Unable to find contract"))
            },
    
        }

    }

    pub fn register_contract(contract: Box<dyn Contract + Send>){
        CONTRACT_MGR.lock().unwrap().register_contract_impl(contract);
    }

    pub fn route(ctx: Context, contract_name: String, tx: String, args: Vec<String>) -> Result<String,String> {
        CONTRACT_MGR.lock().unwrap().evaluate2(ctx,contract_name,tx,args)
    }
}


#[macro_export]
macro_rules! launch_handler {
    ($user_handler:ident) => {
        #[no_mangle]
        pub extern "C" fn __launch() -> i32 {           
            $user_handler()
        }
    };
}
