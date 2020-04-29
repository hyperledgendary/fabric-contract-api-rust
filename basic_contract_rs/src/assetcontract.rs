#![allow(dead_code)]
/*
 * SPDX-License-Identifier: Apache-2.0
 */

//! Basic CRUD style asset contract
//! 
//! 

use fabric_contract::contractapi::contract::*;
use fabric_contract::contractapi::context::*;
use fabric_contract::contractapi::contract::Routing;

// macros for marking up the contract
use contract_macros::contract_impl;

/// Structure for the AssetContract, on which implemenation transaction functions will be added
pub struct AssetContract {

}

/// Implementation of the contract trait for the AssetContract
/// There are default implementation methods, but can be modified if you wish
/// 
/// Recommended that the name() function is always modified
impl Contract for AssetContract {

    //! Name of the contract
    fn name(&self) -> String {
        format!("AssetContract")
    }

    /// Implementing a customer before transaction
    fn before_transaction(&self,ctx: Context)  {
        ctx.log(String::from("AssetContract customer Before_Transaction"));
    }
    
}

#[contract_impl]
impl AssetContract {

    pub fn new() -> AssetContract {
        AssetContract {           
        }
    }

    /// creates an asset
    /// 
    pub fn create_asset(&self,mut ctx: Context, my_assset_id: String, value: String) -> Result<(),String> {
        ctx.log(format!("within the create_asset transactions {}:{}",my_assset_id,value));
        ctx.create_state(my_assset_id.clone(),b"json data here".to_vec());

        Ok(())
    }

    /// reads an asset and returns the value
    pub fn read_asset(&self,mut ctx: Context, my_assset_id: String) -> Result<String,String> {
        ctx.log(format!("within the read_asset transactions {}",my_assset_id));
        let result = ctx.retrieve_state(String::from(my_assset_id));
        let s = format!("retrieved asset data is {}",result);
        Ok(s)
    }


}

// the following (left for reference) is what the [contract_impl] macro would logically add
// to this code

// impl Routing for AssetContract {    
   
//     fn route2(&self, ctx: Context, tx_fn: String, args: Vec<String>) -> Result<String,String>{
//         ctx.log(format!("Inside the contract {} {:?}",tx_fn,args));
//         let _r = match &tx_fn[..] {
//             "create_asset" =>  {
                
//                 let a0 = match args.get(0) {
//                     Some(a) => Ok(a),
//                     None => Err(String::from("Missing argument 0")),
//                 };
                
//                 let a1 = match args.get(1) {
//                     Some(a) => Ok(a),
//                     None => Err(String::from("Missing argument 1")),
//                 };
               
//                 let _r=self.create_asset(ctx, a0.unwrap().to_string(), a1.unwrap().to_string());
//                 Ok(String::from(""))
//             },
//             "read_asset" =>  {

//                 let a0 = match args.get(0) {
//                     Some(a) => Ok(a),
//                     None => Err(String::from("Missing argument 0")),
//                 };

//                 let _r=self.read_asset(ctx, a0.unwrap().to_string());
//                 Ok(String::from(""))
//             },
//             _ => Err(String::from("Unknown transaction fn "))
//         };

//         Ok(String::from("200"))
//     }
// }