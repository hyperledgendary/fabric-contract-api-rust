/*
 * SPDX-License-Identifier: Apache-2.0
 */
#![allow(dead_code)]

use std::fmt;

pub struct TransactionFn {
    name: String,
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
        }
    }

}