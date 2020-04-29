/*
 * SPDX-License-Identifier: Apache-2.0
 */


pub struct Context {
    tx_id: std::string::String,
    logfn : fn(&str),
    // callback: fn(String,Vec<u8>) -> String,  // next step to add in the callback logic
}

impl Context {
    pub fn new(logfn: fn(&str)) -> Context {
        Context {
            tx_id: "01234567890".to_string(),
            logfn,
        }
    }

    pub fn create_state(&mut self, key: String, data: Vec<u8>) {
        println!("[createState] {}  {:?}", key, data);
        // (&self.callback)(String::from("create state"),data);
    }

    pub fn retrieve_state(&mut self, key: String) -> Box<String> {
        println!("[retrieveState] {} ", key);
        Box::new(String::from("data"))
    }

    pub fn get_txid(&mut self) -> &std::string::String {
        return &self.tx_id;
    }

    pub fn log(&self, data: String) {
        (self.logfn)(&data[..]);
    }
}
