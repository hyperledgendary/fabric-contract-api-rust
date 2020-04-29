/*
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::contractapi::context::*;

// trait that is implemented by macro for each struct that does the final step in the routing to
// the transaction functions
pub trait Routing  {
    fn route2(&self, ctx: Context, tx_fn: String, args: Vec<String>) -> Result<String,String>;
}

/// Trait that is implemented for each contract
/// Default implementations here
pub trait Contract : Routing {

    fn name(&self) -> String;
    fn before_transaction(&self, ctx: Context);
    fn after_transaction(&self, _ctx: Context) {
            println!("Default After Tranasction");
    }
    
}
