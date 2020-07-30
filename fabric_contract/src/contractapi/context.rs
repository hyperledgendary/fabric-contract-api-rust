/*
 * SPDX-License-Identifier: Apache-2.0
 */

use fabric_ledger_protos::common_messages::TransactionContext;

#[derive(Default, Clone)]
pub struct Context {
    tx_id: std::string::String,
    channel_id: std::string::String,
}

impl Context {
    pub fn new(context: &TransactionContext) -> Context {
        Context {
            channel_id: context.channel_id.clone(),
            tx_id: context.transaction_id.clone(),
        }
    }

    pub fn get_txid(&self) -> std::string::String {
        self.tx_id.clone()
    }

    pub fn get_channelid(&self) -> std::string::String {
        self.channel_id.clone()
    }
}
