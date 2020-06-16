/*
 * SPDX-License-Identifier: Apache-2.0
 */
#![allow(dead_code)]

use crate::dataapi::{typeschema::ContractType, typeschema::Format, TypeSchema};
use std::fmt;
use Format::Other;

/// Should this transaction be submitted or evaluated?
pub enum TxType {
    Submit,
    Evaluate,
}
#[derive(Default)]
pub struct ParameterDefn {
    name: String,
    type_schema: TypeSchema,
    transient: bool,
}

#[derive(Default)]
pub struct TransactionFn {
    name: String,
    return_type: TypeSchema,
    parameters: Vec<ParameterDefn>,
}

impl fmt::Debug for TransactionFn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TxFn <{}> ", self.name)
    }
}

impl TransactionFn {
    pub fn new(name: &str) -> TransactionFn {
        TransactionFn {
            name: String::from(name),
            return_type: TypeSchema {
                contract_type: ContractType::CTString,
                format: Option::None,
            },
            parameters: vec![],
        }
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }
}

#[derive(Default)]
pub struct TransactionFnBuilder {
    name: String,
    return_type: TypeSchema,
    parameters: Vec<ParameterDefn>,
}

impl TransactionFnBuilder {
    pub fn new() -> TransactionFnBuilder {
        TransactionFnBuilder::default()
    }

    pub fn name(&mut self, name: &str)  {
        self.name = name.to_string();
        
    }

    pub fn return_type(&mut self, return_type: TypeSchema)  {
        self.return_type = return_type;
     
    }

    pub fn add_arg(&mut self, arg: &str) {
        // self.parameters.push(arg);
     
    }

    pub fn build(self) -> TransactionFn {
        TransactionFn {
            name: self.name,
            return_type: self.return_type,
            parameters: Vec::new()//self.parameters,
        }
    }
}
