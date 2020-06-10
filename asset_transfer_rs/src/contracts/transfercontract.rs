/*
 * SPDX-License-Identifier: Apache-2.0
 */

//! Basic CRUD style asset contract
//!
//!

use fabric_contract::contract::*;
use fabric_contract::blockchain::*;
use log::info;

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
    pub fn issue_asset(&self, asset_id: String, properties: String) -> Result<(),ContractError> {

        let tx = fabric_contract::blockchain::Transaction::current_transaction();
        let org = match tx.get_peer_mspid() == tx.get_submitting_identity()?.get_mspid()  {
            true => tx.get_peer_mspid(),
            false => return Err(ContractError::from("error".to_string()))
        };


        
        Ok(())
    }

    #[Transaction()]
    pub fn agree_to_sell(&self, asset_id: String) -> Result<(),ContractError> {
        todo!("issue_asset")
    }

    #[Transaction()]
    pub fn agree_to_buy(&self, asset_id: String) -> Result<(),ContractError> {
        todo!("issue_asset")
    }

    #[Transaction()]
    pub fn verify_asset_properties(&self, asset_id: String) -> Result<(),ContractError> {
        todo!("issue_asset")
    }

    #[Transaction()]
    pub fn transfer_asset(&self, asset_id: String) -> Result<(),ContractError> {
        todo!("issue_asset")
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
