/*
 * SPDX-License-Identifier: Apache-2.0
 */

//! Basic CRUD style asset contract
//!
//! The business logic in these is very simple.
//!
//! Note: the transaction APIs should maybe take &String instead of String?

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
        "AssetContract".to_string()
    }
}

/// The contract implementation
/// Should be marked with the macro `#[contrant_impl]`
///
/// All transactions functions should have the `#[Transaction(evaluate)]` or `#[Transaction(submit)]`
/// This indicates that the function should be submitted (sent for endorsing to all peers) or is a query
/// operation, that does not need to be written to the ledger.
///
#[Contract_Impl]
impl AssetContract {
    /// Some form of 'new' function is required.
    /// See the lib.rs of this example for how this is used.
    pub fn new() -> AssetContract {
        AssetContract {}
    }

    /// Does the Asset with the supplied key exist?
    ///
    /// Returns true or false or ContractError
    #[Transaction(evaluate)]
    pub fn asset_exists(&self, asset_id: String) -> Result<bool, ContractError> {
        info!("# asset_exists");

        let world = Ledger::access_ledger().get_collection(CollectionName::World);
        Ok(world.state_exists(MyAsset::form_key(&asset_id).as_str()  )?)
    }

    /// Creates a new asset, with supplied id, and value
    /// Marked as submit as this updates the ledger
    ///
    ///
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

    /// Reads the value with the supplied asset id
    ///
    /// Returns the string value
    #[Transaction(evaluate)]
    pub fn read_asset_value(&self, my_assset_id: String) -> Result<String, ContractError> {
        // get the collection that is backed by the world state
        let world = Ledger::access_ledger().get_collection(CollectionName::World);

        // Doing a check to see if the asset exists explicitly.
        // so we can return a business specific result
        match self.asset_exists(my_assset_id.clone()) {
            Ok(true) => {
                let v = world.retrieve::<MyAsset>(&my_assset_id)?.get_value();
                Ok(v)
            }
            _ => Err(ContractError::from(String::from("Unable to find asset"))),
        }
    }

    // Counts the number of assets
    //
    //An example of how to use iterators
    #[Transaction(evaluate)]
    pub fn count_assets(&self) -> Result<String, ContractError> {
        // get the collection that is backed by the world state
        let world = Ledger::access_ledger().get_collection(CollectionName::World);

        // Create a KeyQueryHandler, to start at the query 000
        let key_handler = KeyQueryHandler::RangeFrom("000".to_string());

        // execute the query
        let all_000 = world.get_states(key_handler)?;

        // Return the count
        let count = all_000.into_iter().count();
        Ok(format!("Number of 00 agents is {}", count).to_string())
    }

    #[Transaction(evaluate)]
    pub fn the_answer(&self) -> Result<i32,ContractError>{
        Ok(self.what_is_the_answer())
    }

    fn what_is_the_answer(&self) -> i32{
        42
    }

   
}
