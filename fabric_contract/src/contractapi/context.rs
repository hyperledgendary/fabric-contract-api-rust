/*
 * SPDX-License-Identifier: Apache-2.0
 */


pub struct Context {
    tx_id: std::string::String,
    _channel_id: std::string::String,
    logfn : fn(&str),
    // callback: fn(String,Vec<u8>) -> String,  // next step to add in the callback logic
}

impl Context {
    pub fn new(channel_id: String, tx_id: String, logfn: fn(&str)) -> Context {
        Context {
            _channel_id : channel_id,
            tx_id,
            logfn,
        }
    }

    pub fn get_txid(&mut self) -> &std::string::String {
        return &self.tx_id;
    }

    pub fn log(&self, data: String) {
        (self.logfn)(&data[..]);
    }
}
