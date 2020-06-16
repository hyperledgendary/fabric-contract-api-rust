/*
 * SPDX-License-Identifier: Apache-2.0
 */
#![allow(dead_code)]
#![allow(unused_imports)]
use crate::contractapi::context::*;
use crate::contractapi::contract::*;
use crate::{dataapi::WireBuffer, contractapi::contractdefn};

use lazy_static::lazy_static;

use log::{debug, trace, warn, info};

use std::collections::HashMap;
use std::sync::Mutex;

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
        args: &[Vec<u8>],
        transient: &[Vec<u8>]
    ) -> Result<String, String> {
        debug!("contractmanager::evaluate {} {}", contract_name, tx);

        match self.contracts.get(&contract_name) {
            Some(defn) => { 
                let r = defn.invoke(ctx,tx,args/*,transient*/);
                r
            }
            None => {
                warn!(
                    "Unable to find contract Failed {}.{},{:?}",
                    contract_name, tx, args
                );
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
    pub fn route(ctx: &Context, tx: String, args: &[Vec<u8>], transient: &[Vec<u8>]) -> Result<String, String> {
        trace!("contractmanager::route>>");

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
            .evaluate(ctx, namespace, fn_name, args,transient);

        trace!("contractmanager::route<<");
        r
    }
}
