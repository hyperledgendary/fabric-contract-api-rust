use super::{Converter, TypeSchema};
use std::fmt::Debug;


pub struct WireBuffer {
    pub buffer: Vec<u8>,
    pub schema: TypeSchema,
    pub converter: Box<dyn Converter>,
}

impl WireBuffer {
    pub fn new(buffer: Vec<u8>, schema: TypeSchema, converter: Box<dyn Converter>) -> Self { Self { buffer, schema, converter } }
}


impl Debug for WireBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WireBuffer: xxx")
    }
}

impl From<WireBuffer> for String {
    fn from(wb: WireBuffer) -> Self {
        wb.converter.into_string(wb.buffer, wb.schema)
    }   
}

