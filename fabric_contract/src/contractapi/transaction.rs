/*
 * SPDX-License-Identifier: Apache-2.0
 */
#![allow(dead_code)]
#![allow(unused_imports)]
use crate::dataapi::{typeschema::ContractType, typeschema::Format, TypeSchema};
use std::fmt;
use Format::Other;
use std::str::FromStr;

#[derive(Debug,Clone,Copy)]
/// Should this transaction be submitted or evaluated?
pub enum TxType {
    Submit,
    Evaluate,
}
#[derive(Debug,Clone)]
pub struct ParameterDefn {
    pub name: String,
    pub type_schema: TypeSchema,
    pub transient: bool,
}

impl std::fmt::Display for ParameterDefn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?}", self.name,self.type_schema)
    }
  }

impl std::convert::From<&str> for ParameterDefn {
    fn from(tx: &str) -> Self {
        // parse out the contract_name here
        let type_name: String;
        let arg_name: String;
        match tx.find(":") {
            Some(s) => {
                arg_name = tx[..s-1].to_string();
                type_name = tx[s + 2..].to_string();
            }
            None => {panic!("Code is not correct")}
        }
        
        Self {
            name : arg_name.clone(),
            transient : false,
            type_schema:TypeSchema {
                contract_type : ContractType::from_str(&type_name[..]).unwrap(),
                format: Option::None,
            }
        }
    }
}

#[derive(Default,Debug,Clone)]
pub struct TransactionFn {
    name: String,
    return_type: TypeSchema,
    parameters: Vec<ParameterDefn>,
}

impl fmt::Display for TransactionFn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TxFn <{}> {:?}", self.name, self.parameters)
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

    pub fn get_parameters(&self) -> Vec<ParameterDefn> {
        return self.parameters.clone();
    }
}

#[derive(Default,Debug,Clone)]
pub struct TransactionFnBuilder {
    name: String,
    return_type: TypeSchema,
    parameters: Vec<ParameterDefn>,
}

impl TransactionFnBuilder {
    pub fn new() -> TransactionFnBuilder {
        TransactionFnBuilder::default()
    }

    pub fn name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn return_type(&mut self, return_type: TypeSchema) {
        self.return_type = return_type;
    }

    pub fn add_arg(&mut self, arg: &str) {
        self.parameters.push(ParameterDefn::from(arg));
    }

    pub fn build(self) -> TransactionFn {
        TransactionFn {
            name: self.name,
            return_type: self.return_type,
            parameters: self.parameters,
        }
    }
}
