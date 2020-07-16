/*
 * SPDX-License-Identifier: Apache-2.0
 */

//! A basic Smart Contract
//! 
//! Implements Creation, and Retrieval of a simple asset
//! 

mod contracts;
pub use crate::contracts::AssetContract;

mod types;
pub use crate::types::MyAsset;

// The macro to indicate the function that will be used to register the contracts.
// if there is more than one, use a comma separated list
//
// Provide the function that will create a new instance of the contract strcut
// More than one contract struct can be provided. 
fabric_contract::register!( AssetContract::new );