/*
 * SPDX-License-Identifier: Apache-2.0
 */

//! Basic CRUD style asset contract
//!
//!

// Use the Fabric Contract modules
use fabric_contract::contract::*;
use fabric_contract::data::*;

// Use the log crate to support logging
use log::info;

// Our own asset types
use crate::types::MyAsset;

/// Structure for the AssetContract, on which implemenation transaction functions will be added
pub struct AssetContract {}

/// Implementation of the contract trait for the AssetContract
/// There are default implementation methods, for after, before and unknownTransaction
///
/// Recommended that the name() function is always modified
impl Contract for AssetContract {
    
    /// Name of the contract
    fn name(&self) -> String {
        format!("AssetContract")
    }
}

/// The contract implementation
/// Should be marked with the macro `#[contrant_impl]`
/// 
#[Contract_Impl]
impl AssetContract {

    /// Some form of 'new' function is required.
    /// See the lib.rs of this example for how this is used. 
    pub fn new() -> AssetContract {
        AssetContract {}
    }

    /// Does the Asset with the supplied key exist
    ///
    /// Returns true or false.
    #[Transaction(evaluate)]
    pub fn asset_exists(&self, asset_id: String) -> Result<bool, ContractError> {
        info!("# asset_exists");

        let world = Ledger::access_ledger().get_collection(CollectionName::World);
        Ok(world.state_exists(&asset_id)?)
    }
    
    #[Transaction(submit)]
    pub fn create_asset(&self, my_assset_id: String, value: String) -> Result<(), ContractError> {

        info!("#> create_asset");
        // get the collection that is backed by the world state
        let world = Ledger::access_ledger().get_collection(CollectionName::World);
        
        let new_asset = MyAsset::new(my_assset_id, value);
        world.create(new_asset)?;

        info!("#< create_asset");
        Ok(())
    }
   
    #[Transaction(evaluate)]
    pub fn read_asset_value(&self, my_assset_id: String) -> Result<String, ContractError> {
        // get the collection that is backed by the world state
        let world = Ledger::access_ledger().get_collection(CollectionName::World);

        match self.asset_exists(my_assset_id.clone()) {
            Ok(true) => {
                info!("#confirmed that asset exists");
                let v = world.retrieve::<MyAsset>(my_assset_id).unwrap().get_value();
                info!("{}",v);
                Ok(v)
            },
            _ => return Err(ContractError::from(String::from("Unable to find asset"))),
        }
    }
}
