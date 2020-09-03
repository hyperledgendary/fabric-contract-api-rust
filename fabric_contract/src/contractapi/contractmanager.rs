/*
 * SPDX-License-Identifier: Apache-2.0
 */
#![allow(dead_code)]
#![allow(unused_imports)]
use crate::contractapi::context::*;
use crate::contractapi::contract::*;
use crate::{contract::ContractError, contractapi::contractdefn, dataapi::WireBuffer};

use lazy_static::lazy_static;

use log::{debug, info, trace, warn};

use std::collections::HashMap;
use std::sync::Mutex;

// Static reference to the ContractManager
lazy_static! {
    static ref CONTRACT_MGR: Mutex<ContractManager> = Mutex::new(ContractManager::new());
    //static ref CONTRACT_MGR: ContractManager = ContractManager::new();

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
        transient: &[Vec<u8>],
    ) -> Result<WireBuffer, ContractError> {
        debug!("contractmanager::evaluate {} {}", contract_name, tx);

        match self.contracts.get(&contract_name) {
            Some(defn) => {
                defn.invoke(ctx, tx, args /*,transient*/)
            }
            None => {
                warn!(
                    "Unable to find contract Failed {}.{},{:?}",
                    contract_name, tx, args
                );
                Err(ContractError::from(format!("Unable to find contract Failed {}:{}",contract_name, tx)))
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
    pub fn route(
        ctx: &Context,
        tx: String,
        args: &[Vec<u8>],
        transient: &[Vec<u8>],
    ) -> Result<WireBuffer, ContractError> {
        trace!("contractmanager::route>>");

        // parse out the contract_name here
        let namespace: String;
        let fn_name: String;
        match tx.find(':') {
            None => {
                namespace = "default".to_string();
                fn_name = tx.clone();
            }
            Some(s) => {
                namespace = tx[..s].to_string();
                fn_name = tx[s + 1..].to_string();
            }
        }

        // two early checks on the validity of the requested function
        if namespace.is_empty() {
            return Err(ContractError::from(String::from("Emptry namespace, for default just give fn name")));
        };

        if fn_name.is_empty() {
            return Err(ContractError::from(String::from("Empty function name")));
        };

        // let r = CONTRACT_MGR
        //     .lock()
        //     .unwrap()
        //     .evaluate(ctx, namespace, fn_name, args, transient);


        let r = match CONTRACT_MGR
            .lock() {
                Ok(mut ccm) => {            
                    ccm.evaluate(ctx, namespace, fn_name, args, transient)
                }
                Err(poisoned) => {panic!("I've been poisoned")}
            };

        trace!("contractmanager::route<<");
        r
    }
}
