/*
 * SPDX-License-Identifier: Apache-2.0
 */

// Use the Fabric Contract modules
use fabric_contract::contract::*;
use fabric_contract::data::*;
use Expression::Principal;

// Our own asset types
use crate::types::{Asset, AssetPrivate, PriceAgreement, TransferReceipt};

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

    /// CreateAsset issues a new asset.
    /// 
    /// - Public information is put to the world state
    /// - Private data put to the organization's collection
    /// - Endorsement added to public state to permit only owning organization
    ///   to modify
    /// - Must be submitted to the peer of client's organization
    /// 
    #[Transaction(submit, transient = "asset_properties")]
    pub fn create_asset(
        &self,
        id: String,
        public_description: String,
        asset_properties: AssetPrivate,
    ) -> Result<(), ContractError> {
        
        let owner_org = self.get_verified_client_org()?;
        
        // get the collection that is backed by the world state
        let asset_collection = Ledger::access_ledger().get_collection(CollectionName::World);

        match asset_collection.state_exists(id.as_str()) {
            Ok(true) => return Err(ContractError::from("Asset id already present".to_string())),
            Ok(false) => (),
            Err(e) => return Err(ContractError::from(e)),
        }
        
        let asset = Asset::new(id, owner_org.clone(), public_description);
        let state = asset_collection.create(asset)?;

        // Set the endorsement policy such that an owner org peer is required to endorse future updates
        state.set_endorsment(StateBasedEndorsement::build(&[&Principal(
            owner_org.clone(),
            ROLE::PEER,
        )]));

        // add to the organizations implicity colletion
        let private_asset_collection =
           Ledger::access_ledger().get_collection(CollectionName::Organization(owner_org.clone()));
        private_asset_collection.create(asset_properties)?;

        Ok(())
    }

    /// Change the public description of an already created asset
    /// 
    #[Transaction]
    pub fn change_public_description(
        &self,
        asset_id: String,
        public_desc: String,
    ) -> Result<(), ContractError> {

        self.get_verified_client_org()?;

        let world = Ledger::access_ledger().get_collection(CollectionName::World);
        let mut asset = world.retrieve::<Asset>(&asset_id)?;

        asset.set_public_description(public_desc);
        world.update(asset)?;

        Ok(())
    }

    /// Seller submits there agreed selling price
    #[Transaction(transient = "price")]
    pub fn agree_to_sell(&self, asset_id: String, price: u32) -> Result<(), ContractError> {
        let client_msp = self.get_verified_client_org()?;

        let private_collection =
            Ledger::access_ledger().get_collection(CollectionName::Organization(client_msp));
        let asset = private_collection.retrieve::<Asset>(&asset_id)?;

        let client_msp = self.get_verified_client_org()?;
        if asset.get_owner() != client_msp {
            return Err(ContractError::from(
                "Submitting organization does not own asset".to_string(),
            ));
        };

        let sell_price = PriceAgreement::new(asset_id, price);
        let _state = private_collection.create(sell_price);
        Ok(())
    }

    #[Transaction(transient = "price")]
    pub fn agree_to_buy(&self, asset_id: String, price: u32) -> Result<(), ContractError> {
        let client_msp = self.get_verified_client_org()?;
        
        let private_collection =
            Ledger::access_ledger().get_collection(CollectionName::Organization(client_msp));

        let buy_price = PriceAgreement::new(asset_id, price);
        match private_collection.create(buy_price) {
            Ok(_) => Ok(()),
            Err(e) => Err(ContractError::from(e)),
        }
    }

    /// Used by a buyer to vlidate that the asset they are planning on buying 
    /// really is as specified
    #[Transaction(transient = "asset_properties")]
    pub fn verify_asset_properties(
        &self,
        asset_id: String,
        asset_properties: AssetPrivate,
    ) -> Result<(), ContractError> {
        // get the collection that is backed by the world state
        let world = Ledger::access_ledger().get_collection(CollectionName::World);
        let asset = world.retrieve::<Asset>(&asset_id)?;

        let private_collection =
            Ledger::access_ledger().get_collection(CollectionName::Organization(asset.get_owner()));

        let state_hash = private_collection.retrieve_state_hash(&asset_id)?;
        match state_hash.verify_consistent(asset_properties) {
            Ok(true) => Ok(()),
            Ok(false) => Err(ContractError::from("Unable to verify asset properties".to_string())),
            Err(e) => Err(ContractError::from(e)),
        }
    }

    /// Issue the actual transfer asset
    #[Transaction(transient = "asset_properties")]
    pub fn transfer_asset(
        &self,
        asset_id: String,
        buyer_orgid: String,
        asset_properties: AssetPrivate
    ) -> Result<(), ContractError> {
        let tx = fabric_contract::blockchain::Transaction::current_transaction();
        let client_msp = tx.get_submitting_identity()?.get_mspid();
        
        let collection_seller = Ledger::access_ledger().get_collection(CollectionName::Organization(client_msp.clone()));
        let collection_buyer = Ledger::access_ledger().get_collection(CollectionName::Organization(buyer_orgid.clone()));
        let mut asset = collection_seller.retrieve::<Asset>(&asset_id)?;

        // Verify Transfer pre-conditions
        // 1) Confirm that only the owner can submit the transfer transaction
        if asset.get_owner() != client_msp {
            return Err(ContractError::from(
                "Submitting organization does not own asset".to_string(),
            ));
        };

        // 2) Confirm that the private details do indeed match what is recorded
        let state = collection_seller.retrieve_state_hash(&asset_id)?;
        match state.verify_consistent(asset_properties) {
            Ok(true) => (),
            Ok(false) => return Err(ContractError::from("Unable to verify asset properties".to_string())),
            Err(e) => return Err(ContractError::from(e)),
        };

        // 3) Confirm that the prices recorded for buyer and seller match
        let s_hash = collection_seller.retrieve_state_hash(&PriceAgreement::form_key(&asset_id))?;
        let b_hash = collection_buyer.retrieve_state_hash(&PriceAgreement::form_key(&asset_id))?;
        match s_hash.verify_consistent(b_hash){ 
            Ok(true) => (), 
            Ok(false) => return Err(ContractError::from("Unable to verify matching price agreements".to_string())),
            Err(e) => return Err(ContractError::from(e)),
        };

        // remove these records no longer required
        collection_seller.delete_state(&PriceAgreement::form_key(&asset_id))?;
        collection_buyer.delete_state(&PriceAgreement::form_key(&asset_id))?;       
 
        // Transfer the Asset State

        // 1) Update the actual owner
        asset.update_owner(buyer_orgid.clone());

        // 2) Set the endorsement policy such that new owner org peer is required to endorse future updates
        state.set_endorsment(StateBasedEndorsement::build(&[&Principal(
            buyer_orgid.clone(),
            ROLE::PEER,
        )]));

        // 3) add receipts to the private collections of buyer and seller
        collection_seller.create(TransferReceipt::new(asset_id.clone()))?;
        collection_buyer.create(TransferReceipt::new(asset_id.clone()))?;

        Ok(())
    }
}
