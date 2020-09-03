/*
 * SPDX-License-Identifier: Apache-2.0
 */
use fabric_contract::contract::*;
use fabric_contract::data::*;
use serde::{Deserialize, Serialize};
use std::str::from_utf8;

// Use the log crate to support logging
use log::{debug};

/// 

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferAgreement {
    ID: String,
    buyer_id: String,
}

impl TransferAgreement {
    pub fn new(ID: String, buyer_id: String) -> Asset {
        Asset {
            ID,
            buyer_id
        }
    }
}