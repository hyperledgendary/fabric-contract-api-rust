/*
 * SPDX-License-Identifier: Apache-2.0
 */
#![allow(dead_code)]
#![allow(unused_imports)]
use std::collections::HashMap;

use crate::contractapi::contract::*;
use crate::contractapi::transaction;
use crate::{contract::ContractError, dataapi::WireBufferFromReturnType};
use crate::{contractapi::context::*, dataapi::serializer::*, dataapi::WireBuffer};

use log::{debug, trace};

/// Internal definition of a contract

pub struct ContractDefn {
    name: String,
    methods: HashMap<String, transaction::TransactionFn>,
    contract: Box<dyn Contract + Send>,
    converter: Box<dyn Converter + Send>,
}

impl ContractDefn {
    pub fn new(c: Box<dyn Contract + Send>) -> ContractDefn {
        let mut cd = ContractDefn {
            name: c.name(),
            methods: HashMap::new(),
            contract: c,
            converter: Box::new(JSONConverter {}),
        };

        let fns = cd.contract.get_fn_metadata();
        for t in fns {
            debug!("Function {:?}", t);
            cd.add_tx_fn(t);
        }

        // last thing that we need to do is setup the data converter that
        // is required for this contract

        cd
    }

    pub fn add_tx_fn(self: &mut ContractDefn, tx: transaction::TransactionFn) {
        self.methods.insert(tx.get_name(), tx);
    }

    pub fn add_new_method(self: &mut ContractDefn, name: &str, func: fn(&str) -> bool) {
        let tx = transaction::TransactionFn::new(name);
        debug!("{:?}", tx);
        self.methods.insert(String::from(name), tx);
    }

    pub fn get_txfn(
        self: &ContractDefn,
        name: &str,
    ) -> Result<&transaction::TransactionFn, String> {
        match self.methods.get(&String::from(name)) {
            Some(t) => Ok(t),
            None => Err(String::from("Unable to find tx")),
        }
    }

    pub fn invoke(
        self: &ContractDefn,
        ctx: &Context,
        name: String,
        args: &[Vec<u8>],
    ) -> Result<WireBuffer, ContractError> {
        // trace!(">> invoke {} {:#?}",name, args);
        debug!("Invoking tx fn");

        let txfn = self.get_txfn(&name[..])?;
        let mut updated_args = Vec::<WireBuffer>::new();
        // got the tx fn, now to loop over the supplied args
        for (pos, p) in txfn.get_parameters().iter().enumerate() {
            updated_args.push(WireBuffer::new(
                args[pos].clone(),
                p.type_schema, /*,Box::new(JSONConverter {})*/
            ));
        }

        self.contract.route3(name, updated_args, txfn.get_return())
    }
}
