#![allow(dead_code)]
#![allow(unused_variables)]
/*
 * SPDX-License-Identifier: Apache-2.0
 */

//! Basic CRUD style asset contract
//!
//!

use fabric_contract::contract::*;
use fabric_contract::data::*;

use crate::types::MyAsset;

/// Structure for the AssetContract, on which implemenation transaction functions will be added
pub struct AssetContract {}

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

/// The contract implementation
/// Should be marked with the macro `#[contrant_impl]`
#[Contract_Impl]
impl AssetContract {
    /// TODO: Is this required? Can it be enforced
    pub fn new() -> AssetContract {
        AssetContract {}
    }
    /// Does the Asset with the supplied key exist
    ///
    /// Returns true or false.
    #[Transaction(evaluate)]
    pub fn asset_exists(&self, assset_id: String) -> Result<bool, ContractError> {
        let world = Ledger::access_ledger().get_collection(CollectionName::World);
        Ok(world.state_exists(&assset_id))
    }

    /// Creates an asset
    ///
    /// As an example the value is passed as private data
    ///
    #[Transaction(submit)]
    pub fn create_asset(&self, my_assset_id: String, value: String) -> Result<(), ContractError> {
        // get the collection that is backed by the world state
        let world = Ledger::access_ledger().get_collection(CollectionName::World);
        let new_asset = MyAsset::new(my_assset_id,value);

        let _r = world.create(new_asset);
        Ok(())

    }

    /// reads an asset and returns the value
    #[Transaction(evaluate)]
    pub fn read_asset_value(&self, my_assset_id: String) -> Result<String, ContractError> {
        // get the collection that is backed by the world state
        let world = Ledger::access_ledger().get_collection(CollectionName::World);

        self.asset_exists(my_assset_id.clone());

        let value = world.retrieve::<MyAsset>(&my_assset_id).unwrap().get_value();

        Ok(value)

    }
}
