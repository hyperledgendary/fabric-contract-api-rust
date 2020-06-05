/*
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::contractapi::context::*;
use crate::contractapi::contract::*;
use crate::contractapi::contractdefn;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use log::{error, info, warn,debug};
// Static reference to the ContractManager
lazy_static! {
    static ref CONTRACT_MGR: Mutex<ContractManager> = Mutex::new(ContractManager::new());
}

/// Contract Manager structure that holds the list contract objects
pub struct ContractManager {
    contracts: HashMap<String, contractdefn::ContractDefn>,
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

        self.contracts.insert(name, contract_defn);
    }

    fn evaluate(
        self: &mut ContractManager,
        ctx: &Context,
        contract_name: String,
        tx: String,
        args: Vec<u8>,
    ) -> Result<String, String> {
        ctx.log(format!("contractmanager::evaluate {} {}", contract_name,tx));
       
        for (key, _value) in  &self.contracts {
            ctx.log(format!("{} ", key));
        }

        match self.contracts.get(&contract_name) {
            Some(defn) => {
                ctx.log(String::from("Found defn"));

                // need to move this to the contract
                // and apply the correct wire deserializtion protocol
                let parsed_args = String::from_utf8(args)
                    .unwrap_or_default()
                    .split(",")
                    .map(|s| s.to_string())
                    .collect();
                debug!("{:#?}",parsed_args);
                defn.invoke(ctx, tx, parsed_args)
            }
            None => {
                ctx.log(format!("Unable to find contract Failed {}.{},{:?}", contract_name, tx, args));
                Err(String::from("Unable to find contract"))
            }
        }
    }

    /// register the contract
    pub fn register_contract(contract: Box<dyn Contract + Send>) {
        CONTRACT_MGR
            .lock()
            .unwrap()
            .register_contract_impl(contract);
    }

    /// Route the call to the correct contract
    pub fn route(ctx: &Context, tx: String, args: Vec<u8>) -> Result<String, String> {
        ctx.log(String::from("contractmanager::route>>"));

        // parse out the contract_name here
        let namespace: String;
        let fn_name: String;
        match tx.find(":") {
            None => {
                namespace = "default".to_string();
                fn_name = tx.clone();
            }
            Some(s) => {
                namespace = tx[..s].to_string();
                fn_name = tx[s + 1..].to_string();
            }
        }

        let r = CONTRACT_MGR
            .lock()
            .unwrap()
            .evaluate(ctx, namespace, fn_name, args);
        
        ctx.log(String::from("contractmanager::route<<"));
        r
    }
}
