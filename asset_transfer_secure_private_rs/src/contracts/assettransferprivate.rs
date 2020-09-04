/*
 * SPDX-License-Identifier: Apache-2.0
 */

// Use the Fabric Contract modules
use fabric_contract::contract::*;
use fabric_contract::data::*;
use sha2::{Sha256};
// Use the log crate to support logging
use log::{debug, info};

// Our own asset types
use crate::types::{Asset, AssetPrivate,PriceAgreement};


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

    /// Verify the client MSPID and the Peers MSPID are the same
    fn get_verified_client_org(&self) -> Result<String, ContractError> {
        let tx = fabric_contract::blockchain::Transaction::current_transaction();

        let peers_msp = tx.get_peer_mspid();
        let client_msp = tx.get_submitting_identity()?.get_mspid();
        if peers_msp != client_msp {
            Err(ContractError::from(
                "Mismatch of Organization names".to_string(),
            ))
        } else {
            Ok(client_msp)
        }
    }

    /// CreateAsset issues a new asset to the world state with given details.
    #[Transaction(submit, transient = "asset_properties")]
    pub fn create_asset(
        &self,
        id: String,
        public_description: String,
        asset_properties: AssetPrivate,
    ) -> Result<(), ContractError> {
        // get the collection that is backed by the world state
        let asset_collection = Ledger::access_ledger().get_collection(CollectionName::World);

        match asset_collection.state_exists(id.as_str()) {
            Ok(true) => return Err(ContractError::from("Asset id already present".to_string())),
            Ok(false) => (),
            Err(e) => return Err(ContractError::from(e)),
        }
        let owner_org = self.get_verified_client_org()?;
        let asset = Asset::new(id,owner_org.clone(),public_description,false);

        // persist in the world state
        let state  = asset_collection.create(asset)?;
        
        // Set the endorsement policy such that an owner org peer is required to endorse future updates
        state.set_endorsment();


        // add to the organizations implicity colletion 
        let private_asset_collection = Ledger::access_ledger().get_collection(CollectionName::Organization(owner_org));
        private_asset_collection.create(asset_properties);
        Ok(())
    }

    pub fn agree_to_transfer(&self,
        asset_value: AssetPrivate,
        transfer: PriceAgreement,
    ) -> Result<(), ContractError> {
        let tx = fabric_contract::blockchain::Transaction::current_transaction();

        let peers_msp = tx.get_peer_mspid();
        let client_msp = tx.get_submitting_identity()?.get_mspid();
        if peers_msp != client_msp {
            return Err(ContractError::from(
                "Mismatch of Organization names".to_string(),
            ));
        }
        let ASSET_COLLECTION: CollectionName = CollectionName::Private("assetCollection".to_string());

        // get the collection that is backed by the world state
        let asset_collection = Ledger::access_ledger().get_collection(ASSET_COLLECTION);
        Ok(())
    }


    #[Transaction]
    pub fn change_public_description(&self,asset_id: String, public_desc: String) -> Result<(),ContractError> {
        let tx = fabric_contract::blockchain::Transaction::current_transaction();
        let client_msp = tx.get_submitting_identity()?.get_mspid();

        // get the collection that is backed by the world state
        let world = Ledger::access_ledger().get_collection(CollectionName::World);;        
        let mut asset = world.retrieve::<Asset>(&asset_id)?;

        if asset.get_owner() != client_msp {
            return Err(ContractError::from("Submitting organization does not own asset".to_string()));
        };

        asset.set_public_description(public_desc);
        world.update(asset)?;

        Ok(())
    }

    #[Transaction(transient = "price")]
    pub fn agree_to_sell(&self, asset_id: String, price: u32) -> Result<(),ContractError> {
        let client_msp = self.get_verified_client_org()?;

        // get the collection that is backed by the world state
        let privateOrgCollection = Ledger::access_ledger().get_collection(CollectionName::Organization(client_msp));;        
        let asset = privateOrgCollection.retrieve::<Asset>(&asset_id)?;
       
        let client_msp = self.get_verified_client_org()?;
        if asset.get_owner() != client_msp {
            return Err(ContractError::from("Submitting organization does not own asset".to_string()));
        };

        let sell_price = PriceAgreement::new(asset_id,price);
        let _state = privateOrgCollection.create(sell_price);
        Ok(())
    }

    #[Transaction(transient= "price")]
    pub fn agree_to_buy(&self, asset_id: String,price: u32) -> Result<(),ContractError> {
        let client_msp = self.get_verified_client_org()?;
        let privateOrgCollection = Ledger::access_ledger().get_collection(CollectionName::Organization(client_msp));

        let buy_price = PriceAgreement::new(asset_id,price);
        match privateOrgCollection.create(buy_price) {
            Ok(_)=> Ok(()),
            Err(e) => Err(ContractError::from(e))
        }
    }

    #[Transaction(transient = "properties")]
    pub fn verify_asset_properties(&self, asset_id: String) -> Result<(),ContractError> {
              // get the collection that is backed by the world state
        let world = Ledger::access_ledger().get_collection(CollectionName::World);;        
        let asset = world.retrieve::<Asset>(&asset_id)?;

        let privateOrgCollection = Ledger::access_ledger().get_collection(CollectionName::Organization(asset.get_owner()));
        let state = privateOrgCollection.retrieve_state(&asset_id)?;        
        let state_hash = state.get_hash();

        // create a Sha256 object
        // let mut hasher = Sha256::new();

        // write input message
        // hasher.update(properties.into_bytes());

        // read hash digest and consume hasher
        // let result = hasher.finalize();

        // if result!=state_hash {
        //     return Err(ContractError::from("Verification of properties failure"))
        // };

        Ok(())
    }
    #[Transaction(transient = "properties,price" )]
    pub fn transfer_asset(&self, asset_id: String,asset_properties: AssetPrivate) -> Result<(),ContractError> {
        let tx = fabric_contract::blockchain::Transaction::current_transaction();
        let client_msp = tx.get_submitting_identity()?.get_mspid();

        // get the collection that is backed by the world state
        let world = Ledger::access_ledger().get_collection(CollectionName::World);;        
        let asset = world.retrieve::<Asset>(&asset_id)?;

        if asset.get_owner() != client_msp {
            return Err(ContractError::from("Submitting organization does not own asset".to_string()));
        };

        let privateOrgCollection = Ledger::access_ledger().get_collection(CollectionName::Organization(client_msp));
        let state = privateOrgCollection.retrieve_state(&asset_id,/* price record */)?;        
        let state_hash = state.get_hash();     
        
        // asset_properties.getHash()

        // validate transfer agreements
        // these should all be the same

        Ok(())
    }


}
