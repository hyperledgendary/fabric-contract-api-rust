/*
 * SPDX-License-Identifier: Apache-2.0
 */

//! Main library module 
//! 
//! The purpose of this is to do register the contracts.
//! 
//! Other setup can take place here but do so with
//! caution. Any failure will terminate the container
//! and there is limited scope for logging at stage in
//! the lifecyle
mod contracts;
use crate::contracts::AssetContract;

mod types;
use crate::types::MyAsset;

// The macro to indicate the function that will be used to register the contracts.
// if there is more than one, use a comma separated list
//
// Provide the function that will create a new instance of the contract strcut
// More than one contract struct can be provided. 
fabric_contract::register!( AssetContract::new );