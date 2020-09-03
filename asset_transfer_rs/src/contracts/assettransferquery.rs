/*
 * SPDX-License-Identifier: Apache-2.0
 */

// Use the Fabric Contract modules
use fabric_contract::contract::*;
use fabric_contract::data::*;

// Our own asset types
use crate::types::Asset;


const INDEX_NAME: &str = "index~name";
/// Structure for the AssetContract, on which implemenation transaction functions will be added
pub struct AssetTransferQuery {}

/// Implementation of the contract trait for the AssetContract
/// There are default implementation methods, but can be modified if you wish
///
/// Recommended that the name() function is always modified
impl Contract for AssetTransferQuery {
    //! Name of the contract
    fn name(&self) -> String {
        format!("AssetTransferQuery")
    }
}

/// The contract implementation
/// Should be marked with the macro `#[contrant_impl]`
#[Contract_Impl]
impl AssetTransferQuery {
    pub fn new() -> AssetTransferQuery {
        AssetTransferQuery {}
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
        let world = Ledger::access_ledger().get_collection(CollectionName::World);

        // create the new asset
        let new_asset = Asset::new(id.clone(), color.clone(), size, owner, appraised_value);        
        world.create(new_asset)?;

        let index_key = State::make_composite_key(INDEX_NAME.into(), vec![color,id]);
        world.create_state(index_key, vec![0])?;

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

        let asset = world.retrieve::<Asset>(&id)?;

        match world.delete_state(&id) {
            Err(e) => Err(ContractError::from((
                format!("Unable to delete asset {}", id),
                e,
            ))),
            Ok(_) => Ok(()),
        }?;
        let index_key = State::make_composite_key(INDEX_NAME.into(), vec![asset.get_color(),asset.get_id()]);

        match world.delete_state(&index_key) {
            Err(e) => Err(ContractError::from((
                format!("Unable to delete asset index {}", id),
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

    #[Transaction(submit)]
    pub fn transfer_asset_by_color(&self,color: String, new_owner: String)-> Result<(),ContractError>{

        let world = Ledger::access_ledger().get_collection(CollectionName::World);     
        let index_key = State::make_composite_key(INDEX_NAME.into(), vec![color]);
        
        let states = world.get_states(KeyQueryHandler::RangeFrom(index_key))?;
        for s in states {
            let key = State::split_composite_key(s.key()).remove(1);            
            self.transfer_asset(key, new_owner.clone())?;
        }
        Ok(())

    }
}
