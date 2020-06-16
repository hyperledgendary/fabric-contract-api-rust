/*
 * SPDX-License-Identifier: Apache-2.0
 */
#![allow(unused_imports)]
use crate::{dataapi::WireBuffer, contractapi::context::*, prelude::TransactionFn};
use super::contractdefn::ContractDefn;

// trait that is implemented by macro for each struct that does the final step in the routing to
// the transaction functions
pub trait Routing  {
    fn route2(&self, ctx: &Context, tx_fn: String, args: Vec<String>) -> Result<String,String>;
    fn route3(&self, tx_fn: String, args: Vec<WireBuffer>) -> Result<String,String>;
}

pub trait Metadata { 
    /// Gets the detail of the functions, which is vector of strings
    fn get_fn_metadata(&self) -> Vec<TransactionFn>;
}

/// Trait that is implemented for each contract
/// Default implementations here
pub trait Contract : Routing  + Metadata {

    fn name(&self) -> String;
    // fn before_transaction(&self, ctx: Context);
    // fn after_transaction(&self, _ctx: Context) {
    //         println!("Default After Tranasction");
    // }
    
}

