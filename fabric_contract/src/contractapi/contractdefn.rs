/*
 * SPDX-License-Identifier: Apache-2.0
 */
#![allow(dead_code)]

use std::collections::HashMap;

use crate::contractapi::transaction;
use crate::contractapi::contract::*;
use crate::contractapi::context::*;

use log::{debug, trace};

/// Internal definition of a contract
pub(super) struct ContractDefn {
    name: String,
    methods: HashMap<String, transaction::TransactionFn>,
    contract: Box<dyn Contract +Send>,
}


impl ContractDefn {
    pub fn new(c: Box<dyn Contract+Send>) -> ContractDefn {
        ContractDefn {
            name: String::from(c.name()),
            methods: HashMap::new(),
            contract:c,
        }
    }

    pub fn add_new_method(self: &mut ContractDefn, name: &str, func: fn(&str) -> bool) {

        let tx = transaction::TransactionFn::new(name,func);
        debug!("{:?}",tx);
        self.methods.insert(String::from(name), tx);
    }

    pub fn get_txfn(self: &ContractDefn, name: &str) -> Result<&transaction::TransactionFn,String> {        
        match self.methods.get(&String::from(name)) {
            Some(t) => Ok(t),
            None => Err(String::from("Unable to find tx")),
        }
        
    }

    pub fn invoke(self: &ContractDefn, ctx: &Context, name:String, args:Vec<String>) -> Result<String,String> {
        trace!(">> invoke {} {:#?}",name, args);
        let _r = self.contract.route2(ctx,name,args); 
        Ok(String::from("ok"))
    }
}


