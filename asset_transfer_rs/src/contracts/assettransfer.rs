/*
 * SPDX-License-Identifier: Apache-2.0
 */

// Use the Fabric Contract modules
use fabric_contract::contract::*;
use fabric_contract::data::*;

// Use the log crate to support logging
use log::{info};

// Our own asset types
use crate::types::Asset;



/// Structure for the AssetContract, on which implemenation transaction functions will be added
pub struct AssetTransfer {}

/// Implementation of the contract trait for the AssetContract
/// There are default implementation methods, but can be modified if you wish
///
/// Recommended that the name() function is always modified
impl Contract for AssetTransfer {
    //! Name of the contract
    fn name(&self) -> String {
        format!("AssetTransfer")
    }
}

/// The contract implementation
/// Should be marked with the macro `#[contrant_impl]`
#[Contract_Impl]
impl AssetTransfer {
    pub fn new() -> AssetTransfer {
        AssetTransfer {}
    }

    #[Transaction]
    pub fn init_ledger(&self) -> Result<(), ContractError> {
        // get the collection that is backed by the world state
        let world = Ledger::access_ledger().get_collection(CollectionName::World);

        world.create(Asset::new(
            "asset1".to_string(),
            "blue".to_string(),
            5,
            "Tomoko".to_string(),
            300,
        ))?;
        world.create(Asset::new(
            "asset2".to_string(),
            "red".to_string(),
            5,
            "Brad".to_string(),
            400,
        ))?;
        world.create(Asset::new(
            "asset3".to_string(),
            "green".to_string(),
            10,
            "Jin Soo".to_string(),
            500,
        ))?;
        world.create(Asset::new(
            "asset4".to_string(),
            "yellow".to_string(),
            10,
            "Max".to_string(),
            600,
        ))?;
        world.create(Asset::new(
            "asset5".to_string(),
            "black".to_string(),
            15,
            "Adriana".to_string(),
            700,
        ))?;
        world.create(Asset::new(
            "asset6".to_string(),
            "white".to_string(),
            15,
            "Michel".to_string(),
            800,
        ))?;

        Ok(())
    }

    /// CreateAsset issues a new asset to the world state with given details.
    #[Transaction(submit)]
    pub fn create_asset(
        &self,
        id: String,
        color: String,
        size: i32,
        owner: String,
        appraised_value: i32,
    ) -> Result<(), ContractError> {
        // get the collection that is backed by the world state
        info!("create_asset");
        let world = Ledger::access_ledger().get_collection(CollectionName::World);
        
        // create the new asset
        let new_asset = Asset::new(id, color, size, owner, appraised_value);        
        world.create(new_asset)?;

        Ok(())
    }

    #[Transaction(evaluate)]
    pub fn read_asset(&self, id: String) -> Result<Asset, ContractError> {
        let world = Ledger::access_ledger().get_collection(CollectionName::World);
        let asset = world.retrieve::<Asset>(&id)?;
        Ok(asset)
    }

    #[Transaction(submit)]
    pub fn update_asset(
        &self,
        id: String,
        color: String,
        size: i32,
        owner: String,
        appraised_value: i32,
    ) -> Result<(), ContractError> {
        let world = Ledger::access_ledger().get_collection(CollectionName::World);

        match world.update::<Asset>(Asset::new(id, color, size, owner, appraised_value)) {
            Ok(_) => Ok(()),
            Err(e) => {
                return Err(ContractError::from((
                    "That asset is not found".to_string(),
                    e,
                )))
            }
        }
    }

    #[Transaction(submit)]
    pub fn delete_asset(&self, id: String) -> Result<(), ContractError> {
        let world = Ledger::access_ledger().get_collection(CollectionName::World);

        match world.delete_state(&id) {
            Err(e) => Err(ContractError::from((
                format!("Unable to delete asset {}", id),
                e,
            ))),
            Ok(_) => Ok(()),
        }
    }

    #[Transaction(evaluate)]
    pub fn asset_exists(&self, id: String) -> Result<bool, ContractError> {
        let world = Ledger::access_ledger().get_collection(CollectionName::World);

        match world.state_exists(id.as_str()) {
            Err(e) => Err(ContractError::from((
                format!("Unable to check asset {}", id),
                e,
            ))),
            Ok(r) => Ok(r),
        }
    }

    #[Transaction(submit)]
    pub fn transfer_asset(&self, id: String, new_owner: String) -> Result<(), ContractError> {
        let world = Ledger::access_ledger().get_collection(CollectionName::World);
        let asset = world.retrieve::<Asset>(&id);

        match asset {
            Ok(mut a) => {
                a.update_owner(new_owner);
                world.update::<Asset>(a)?;
                Ok(())
            }
            Err(e) => Err(ContractError::from((
                format!("Unable to check asset {}", id),
                e,
            ))),
        }
    }
}
