


#[derive(Debug)]
// #[derive(Copy)]
pub struct Context {
    tx_id: std::string::String,
    callback: fn(String,Vec<u8>) -> String,
}

impl Context {
    pub fn new(callback: fn(String,Vec<u8>) -> String) -> Context {
        Context {
            tx_id: "01234567890".to_string(),
            callback : callback,
        }
    }

    pub fn create_state(&mut self, key: String, data: Vec<u8>) {
        println!("[createState] {}  {:?}", key, data);
        (&self.callback)(String::from("create state"),data);
    }

    pub fn retrieve_state(&mut self, key: String) -> Box<String> {
        println!("[retrieveState] {} ", key);
        Box::new(String::from("data"))
    }

    pub fn get_txid(&mut self) -> &std::string::String {
        return &self.tx_id;
    }
}
