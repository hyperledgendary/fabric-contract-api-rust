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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_context() {
        let mut txctx = fabric_ledger_protos::common_messages::TransactionContext::new();
        txctx.set_channel_id("0x12345".to_string());
        txctx.set_transaction_id("0xCAFEBABE".to_string());
        let ctx = Context::new(&txctx);

        assert_eq!(ctx.get_txid(), "0xCAFEBABE");
        assert_eq!(ctx.get_channelid(), "0x12345");
    }
}
