/*
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::{
    blockchain::TransactionContext,
    contract::ContractError,
    dataapi::{TypeSchema, WireBuffer},
    prelude::TransactionFn,
};

// trait that is implemented by macro for each struct that does the final step in the routing to
// the transaction functions
pub trait Routing {
    fn route3(
        &self,
        tx_fn: String,
        args: Vec<WireBuffer>,
        return_wb: TypeSchema,
    ) -> Result<WireBuffer, ContractError>;
}

pub trait Metadata {
    /// Gets the detail of the functions, which is vector of strings
    fn get_fn_metadata(&self) -> Vec<TransactionFn>;
}

/// Trait that is implemented for each contract
/// Default implementations here
pub trait Contract: Routing + Metadata {
    fn name(&self) -> String;
    // fn before_transaction(&self, ctx: Context);
    // fn after_transaction(&self, _ctx: Context) {
    //         println!("Default After Tranasction");
    // }

    /// Verify the client MSPID and the Peers MSPID are the same
    fn get_verified_client_org(&self) -> Result<String, ContractError> {
        let tx = TransactionContext::current_transaction();

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
}
