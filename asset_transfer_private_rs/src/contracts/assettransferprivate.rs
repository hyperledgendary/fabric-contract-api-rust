/*
 * SPDX-License-Identifier: Apache-2.0
 */

// Use the Fabric Contract modules
use fabric_contract::contract::*;
use fabric_contract::data::*;

// Use the log crate to support logging
use log::{info,debug};

// Our own asset types
use crate::types::{Asset,AssetPrivate};


const ASSET_COLLECTION: CollectionName = CollectionName::Private("assetCollection".to_string());


/// Structure for the AssetContract, on which implemenation transaction functions will be added
pub struct AssetTransfer {}

/// Implementation of the contract trait for the AssetContract
/// There are default implementation methods, but can be modified if you wish
///
/// Recommended that the name() function is always modified
impl Contract for AssetTransfer {
    //! Name of the contract
    fn name(&self) -> String {
        format!("AssetTransferPrivate")
    }
}

/// The contract implementation
/// Should be marked with the macro `#[contrant_impl]`
#[Contract_Impl]
impl AssetTransfer {
    pub fn new() -> AssetTransfer {
        AssetTransfer {}
    }

    /// CreateAsset issues a new asset to the world state with given details.
    #[Transaction(submit,transient="asset,appraised")]
    pub fn create_asset(
        &self,
        asset: Asset,
        appraised: AssetPrivate
    ) -> Result<(), ContractError> {
        // get the collection that is backed by the world state
        let asset_collection = Ledger::access_ledger().get_collection(ASSET_COLLECTION);

        asset_collection.state_exists(asset.get_id().as_str()).or_then(/*new error*/);
        let tx = fabric_contract::blockchain::Transaction::current_transaction();

        let peers_msp = tx.get_peer_mspid();
        let client_msp = tx.get_submitting_identity()?.get_mspid();
        if peers_msp != client_msp {
            return Err(ContractError::from("Mismatch of Organization names".to_string()));
        }

        let collectionName = CollectionName::Private(format!("{}PrivateCollection",client_msp));
        let org_collection = Ledger::access_ledger().get_collection(collectionName);
        asset_collection.create(asset);        
        org_collection.create(appraised);

        Ok(())
    }

    pub fn agree_to_transfer(asset_value: AssetPrivate, transfer: TransferAgreement) -> Result<(),ContractError> {
        let tx = fabric_contract::blockchain::Transaction::current_transaction();

        let peers_msp = tx.get_peer_mspid();
        let client_msp = tx.get_submitting_identity()?.get_mspid();
        if peers_msp != client_msp {
            return Err(ContractError::from("Mismatch of Organization names".to_string()));
        }

        // get the collection that is backed by the world state
        let asset_collection = Ledger::access_ledger().get_collection(ASSET_COLLECTION);


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
