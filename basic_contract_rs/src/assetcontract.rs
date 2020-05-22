#![allow(dead_code)]
/*
 * SPDX-License-Identifier: Apache-2.0
 */

//! Basic CRUD style asset contract
//! 
//! 

use fabric_contract::contract::*;
use fabric_contract::error::*;

use crate::myasset::*;

// macros for marking up the contract
use contract_macros::contract_impl;
use contract_macros::transaction;


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
    
}



// transient

#[contract_impl]
impl AssetContract {

    
    pub fn new() -> AssetContract {
        AssetContract {           
        }
    }
    
    #[transaction]
    pub fn asset_exists(assset_id:  String) -> Result<bool,ContractError> {
        let ledger = Ledger::access_ledger();
        let world = ledger.get_collection(CollectionName::World);

        Ok(world.state_exists(assset_id))

    }

    /// creates an asset
    /// 
    #[transaction(transient = { value })]
    pub fn create_asset(&self, my_assset_id: String, value: String) -> Result<(),String> {
      
        // get the collection that is backed by the world state
        let ledger = Ledger::access_ledger();
        let world = ledger.get_collection(CollectionName::World);

        let new_asset = MyAsset::new(value);//Asset::builder().uuid(my_assset_id).build();

        let serialized = serde_json::to_string(&new_asset).unwrap();

        world.create_state(my_assset_id,serialized.into_bytes());

        Ok(())
        //
        // get the collection that is backed by the Organization's Implicity Private Data Collection
        //        let orgs_collection = ledger.get_collection(CollectionName::Organization(String::from("org1")));
        
        // get the collection that is backed by the named Private Data Collection
        //       let private_collection = ledger.get_collection(CollectionName::Private(String::from("my_private_details")));
    }

    /// reads an asset and returns the value
    #[transaction(evaluate)]
    pub fn read_asset(&self, my_assset_id: String) -> Result<String,String> {
        let ledger = Ledger::access_ledger();
        let world = ledger.get_collection(CollectionName::World);

        let result = world.get_state(String::from(my_assset_id));
        let serialized = String::from_utf8(result.value()).unwrap();

        let deserialized: MyAsset = serde_json::from_str(&serialized).unwrap();
        
        Ok(deserialized.get_value())
    }
  
}

