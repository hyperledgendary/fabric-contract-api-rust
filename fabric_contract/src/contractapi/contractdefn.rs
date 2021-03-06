/*
 * SPDX-License-Identifier: Apache-2.0
 */
#![allow(dead_code)]

use std::collections::HashMap;

use crate::contractapi::contract::*;
use crate::contractapi::transaction;
use crate::{blockchain::TransactionContext, dataapi::serializer::*, dataapi::WireBuffer};
use crate::{contract::ContractError};

use log::{debug};

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
        ctx: &TransactionContext,
        name: String,
        args: &[Vec<u8>],
        transient: &HashMap<String, Vec<u8>>,
    ) -> Result<WireBuffer, ContractError> {
       
        debug!("Invoking tx fn {} {:#?} {}", name, args, args.len());

        let txfn = self.get_txfn(&name[..])?;
        let transient_ids = txfn.get_transient_ids();
        let mut updated_args = Vec::<WireBuffer>::new();
        // got the tx fn, now to loop over the supplied args
        for (pos, p) in txfn.get_parameters().iter().enumerate() {
            debug!("{} {:?}", pos, p);
            // if the parameter is in the transient list, then get it from the hashmap instead
            if transient_ids.contains(&p.name){
                let temp_buffer = transient.get::<_>(&p.name).unwrap().clone();
                updated_args.push(WireBuffer::new(
                    temp_buffer,
                    p.type_schema, /*,Box::new(JSONConverter {})*/
                ));

            } else {
                updated_args.push(WireBuffer::new(
                    args[pos].clone(),
                    p.type_schema, /*,Box::new(JSONConverter {})*/
                ));
            }
        }

        let buffer = self
            .contract
            .route3(name, updated_args, txfn.get_return())?;
        debug!("Returned buffer {:?}", &buffer);
        Ok(buffer)
    }
}

// Test section
#[cfg(test)]
mod tests {
    use super::*;
    use crate::contractapi::transaction::*;
    use crate::data::TypeSchema;
    use claim::assert_ok;
    use mockall::{mock, predicate::*};

    mock! {
        TestContract {}
        // First trait to implement on C
        trait Metadata {
            fn get_fn_metadata(&self) -> Vec<TransactionFn>;
        }
        // Second trait to implement on C
        trait Routing {
            fn route3(
                &self,
                tx_fn: String,
                args: Vec<WireBuffer>,
                return_wb: TypeSchema,
            ) -> Result<WireBuffer, ContractError>;
        }
    }

    impl Contract for MockTestContract {
        fn name(&self) -> String {
            "TestContract".to_string()
        }
    }

    #[test]
    fn new_defn() {
        let contract = MockTestContract::new();
        let mut b = Box::new(contract);

        let mut tx_fns = Vec::<TransactionFn>::new();
        let mut txfn1 = TransactionFnBuilder::new();
        txfn1.name("testfn");
        let t = txfn1.build();
        tx_fns.push(t);

        b.expect_get_fn_metadata().return_const(tx_fns);
        let contract_defn = ContractDefn::new(b);
        assert_ok!(contract_defn.get_txfn("testfn"));
    }
}
