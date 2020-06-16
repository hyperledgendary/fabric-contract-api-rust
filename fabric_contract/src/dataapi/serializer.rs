

use super::TypeSchema;
use super::typeschema::ContractType::*;
pub trait Converter {
    fn into_string(&self, buffer: &Vec<u8>,ts: &TypeSchema) -> String;
}


#[derive(Debug, Clone, Copy)]
pub struct JSONConverter {

}

impl Converter for JSONConverter {

    // straight conversion 
    fn into_string(&self, buffer: &Vec<u8>,ts: &TypeSchema) -> String {
        match ts.contract_type {
            CTString => { String::from_utf8(buffer.to_vec()).unwrap() }
            _ => { "".to_string() }
        }
    }
    
}