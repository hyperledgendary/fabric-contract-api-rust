/*
 * SPDX-License-Identifier: Apache-2.0
 */

use fabric_ledger_protos::common_messages::TransactionContext;

pub struct Context {
    tx_id: std::string::String,
    _channel_id: std::string::String,
}

impl Context {
    pub fn new(context:  &TransactionContext) -> Context {
        Context {
            _channel_id : context.channel_id.clone(),
            tx_id: context.transaction_id.clone()
        }
    }

    pub fn get_txid(&mut self) -> &std::string::String {
        return &self.tx_id;
    }


}
