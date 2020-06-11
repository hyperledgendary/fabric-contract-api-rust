/*
 * SPDX-License-Identifier: Apache-2.0
 */

//! Basic CRUD style asset contract
//!
//!

use fabric_contract::contract::*;
use fabric_contract::blockchain::*;
use log::info;

use sha2::{Sha256};

use crate::types::MyAsset;

/// Structure for the AssetContract, on which implemenation transaction functions will be added
pub struct TransferContract {}

/// Implementation of the contract trait for the AssetContract
/// There are default implementation methods, but can be modified if you wish
///
/// Recommended that the name() function is always modified
impl Contract for TransferContract {
    //! Name of the contract
    fn name(&self) -> String {
        format!("AssetContract")
    }
}

/// The contract implementation
/// Should be marked with the macro `#[contrant_impl]`
#[Contract_Impl]
impl TransferContract {
    
    pub fn new() -> TransferContract {
        TransferContract {}
    }
    
    #[Transaction(transient = [properties])]
    pub fn issue_asset(&self, asset_id: String, public_desc: String, properties: String) -> Result<(),ContractError> {

        let tx = fabric_contract::blockchain::Transaction::current_transaction();

        let peers_msp = tx.get_peer_mspid();
        let client_msp = tx.get_submitting_identity()?.get_mspid();

        if peers_msp != client_msp {
            return Err(ContractError::from("Mismatch of Organization names".to_string()));
        }

        let asset = MyAsset::new(asset_id,public_desc);
        
        // get the collection that is backed by the world state
        let world = Ledger::access_ledger().get_collection(CollectionName::World);
        let state = world.create(asset)?;
        
        state.set_endorsement(/* Peerm clientMSP */);


        let privateOrgCollection = Ledger::access_ledger().get_collection(CollectionName::Organization(client_msp));
        privateOrgCollection.create_state(asset_id,properties.into_bytes());
        
        Ok(())
    }

    #[Transaction]
    pub fn change_public_description(&self,asset_id: String, public_desc: String) -> Result<(),ContractError> {
        let tx = fabric_contract::blockchain::Transaction::current_transaction();
        let client_msp = tx.get_submitting_identity()?.get_mspid();

        // get the collection that is backed by the world state
        let world = Ledger::access_ledger().get_collection(CollectionName::World);;        
        let asset = world.retrieve(&asset_id)?;

        if asset.get_owner() != client_msp {
            return Err(ContractError::from("Submitting organization does not own asset".to_string()));
        };

        asset.set_public_description(public_desc);
        world.update(asset)?;

        Ok(())
    }

    #[Transaction(transient = [price])]
    pub fn agree_to_sell(&self, asset_id: String, price: String) -> Result<(),ContractError> {
        let tx = fabric_contract::blockchain::Transaction::current_transaction();

        let peers_msp = tx.get_peer_mspid();
        let client_msp = tx.get_submitting_identity()?.get_mspid();

        if peers_msp != client_msp {
            return Err(ContractError::from("Mismatch of Organization names".to_string()));
        }
        // get the collection that is backed by the world state
        let world = Ledger::access_ledger().get_collection(CollectionName::World);;        
        let asset = world.retrieve(&asset_id)?;

        if asset.get_owner() != client_msp {
            return Err(ContractError::from("Submitting organization does not own asset".to_string()));
        };

        let privateOrgCollection = Ledger::access_ledger().get_collection(CollectionName::Organization(client_msp));
        privateOrgCollection.create_state(asset_id,/* price record */);

    }

    #[Transaction()]
    pub fn agree_to_buy(&self, asset_id: String) -> Result<(),ContractError> {
        let tx = fabric_contract::blockchain::Transaction::current_transaction();

        let peers_msp = tx.get_peer_mspid();
        let client_msp = tx.get_submitting_identity()?.get_mspid();

        if peers_msp != client_msp {
            return Err(ContractError::from("Mismatch of Organization names".to_string()));
        }

        let privateOrgCollection = Ledger::access_ledger().get_collection(CollectionName::Organization(client_msp));


    }


    #[Transaction(transient = [properties])]
    pub fn verify_asset_properties(&self, asset_id: String, properties: String) -> Result<(),ContractError> {
              // get the collection that is backed by the world state
        let world = Ledger::access_ledger().get_collection(CollectionName::World);;        
        let asset = world.retrieve(&asset_id)?;

        let privateOrgCollection = Ledger::access_ledger().get_collection(CollectionName::Organization(asset.get_owner()));
        let state = privateOrgCollection.retrieve_state(asset_id,/* price record */)?;        
        let state_hash = state.get_hash();

        // create a Sha256 object
        let mut hasher = Sha256::new();

        // write input message
        hasher.update(properties.into_bytes());

        // read hash digest and consume hasher
        let result = hasher.finalize();

        if result!=state_hash {
            return Err(ContractError::from("Verification of properties failure"))
        };

        Ok(())
    }

    #[Transaction(transient = [properties,price])]
    pub fn transfer_asset(&self, asset_id: String,properties: String, price: String) -> Result<(),ContractError> {
        let tx = fabric_contract::blockchain::Transaction::current_transaction();
        let client_msp = tx.get_submitting_identity()?.get_mspid();


    }

    pub fn verify_transfer_conditions(){

        let tx = fabric_contract::blockchain::Transaction::current_transaction();

        let peers_msp = tx.get_peer_mspid();
        let client_msp = tx.get_submitting_identity()?.get_mspid();

        // CHECK1: auth check to ensure that client's org actually owns the asset
        if client_org != asset.get_owner() {

        }

        let privateOrgCollection = Ledger::access_ledger().get_collection(CollectionName::Organization(client_msp));
        let state = privateOrgCollection.retrieve_state(asset_id,/* price record */)?;        
        let state_hash = state.get_hash();
        // verify that the hash of the passed immutable properties matches the on-chain hash

        // CHECK3: verify that seller and buyer agreed on the same price

    }



    /// Does the Asset with the supplied key exist
    ///
    /// Returns true or false.
    #[Transaction(evaluate)]
    pub fn asset_exists(&self, asset_id: String) -> Result<bool, ContractError> {
        let world = Ledger::access_ledger().get_collection(CollectionName::World);
        Ok(world.state_exists(&asset_id))
    }

    /// Creates an asset
    ///
    /// As an example the value is passed as private data
    ///
    #[Transaction(submit)]
    pub fn create_asset(&self, my_assset_id: String, value: String) -> Result<(), ContractError> {
        // get the collection that is backed by the world state
        let world = Ledger::access_ledger().get_collection(CollectionName::World);
        info!("Accessed collection");
        let new_asset = MyAsset::new(my_assset_id, value);
        info!("Created asset");

        world.create(new_asset)?;

        info!("created in world");
        Ok(())
    }

    /// reads an asset and returns the value
    #[Transaction(evaluate)]
    pub fn read_asset_value(&self, my_assset_id: String) -> Result<String, ContractError> {
        // get the collection that is backed by the world state
        let world = Ledger::access_ledger().get_collection(CollectionName::World);

        match self.asset_exists(my_assset_id.clone()) {
            Ok(true) => Ok(world.retrieve::<MyAsset>(&my_assset_id)?.get_value()),
            _ => Err(ContractError::from(String::from("Unable to find asset"))),
        }
    }
}
