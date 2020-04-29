


use std::fmt;



pub struct TransactionFn {
    name: String,
    tx: fn(&str) -> bool,
}

impl fmt::Debug for TransactionFn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TxFn <{}> ", self.name)
    }
}

impl TransactionFn {

    pub fn new(name: &str, func: fn(&str)->bool) -> TransactionFn {
        TransactionFn {
            name: String::from(name),
            tx:func,
        }
    }


    pub fn call(self: &TransactionFn,arg: &str) {
        (self.tx)(arg);
    }
}