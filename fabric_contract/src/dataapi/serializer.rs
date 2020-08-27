/*
 * SPDX-License-Identifier: Apache-2.0
 */
use super::typeschema::ContractType::*;
use super::TypeSchema;
pub trait Converter {
    fn into_string(&self, buffer: &[u8], ts: &TypeSchema) -> String;
}

#[derive(Debug, Clone, Copy)]
pub struct JSONConverter {}

impl Converter for JSONConverter {
    // straight conversion
    fn into_string(&self, buffer: &[u8], ts: &TypeSchema) -> String {
        match ts.contract_type {
            CTString => String::from_utf8(buffer.to_vec()).unwrap(),
            _ => "".to_string(),
        }
    }
}
