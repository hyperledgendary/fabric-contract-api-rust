/*
 * SPDX-License-Identifier: Apache-2.0
 */
use super::TypeSchema;
use std::fmt::Debug;

pub struct WireBuffer {
    pub buffer: Option<Vec<u8>>,
    pub schema: TypeSchema,
    //  pub converter: Box<dyn Converter>,
}

impl WireBuffer {
    pub fn new(
        buffer: Vec<u8>,
        schema: TypeSchema, /*, converter: Box<dyn Converter>*/
    ) -> Self {
        Self {
            buffer: Some(buffer),
            schema,
            //converter,
        }
    }

    pub fn new_unfilled(schema: TypeSchema /*, converter: Box<dyn Converter>*/) -> Self {
        Self {
            buffer: Option::None,
            schema,
            // converter,
        }
    }
}

impl Debug for WireBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.buffer {
            Some(b) =>{
                write!(f, "WireBuffer: {:?}", b.as_slice())
            },
            None => { 
                write!(f, "WireBuffer: <emptry>")
            }
        }
    }
}

impl From<&WireBuffer> for String {
    fn from(wb: &WireBuffer) -> Self {
        match &wb.buffer {
            Some(buffer) => std::str::from_utf8(&buffer).unwrap().to_string(),
            None => "".to_string(),
        }
    }
}

impl From<&WireBuffer> for i32 {
    fn from(wb: &WireBuffer) -> Self {
        match &wb.buffer {
            Some(buffer) => {
                match std::str::from_utf8(&buffer) {
                    Ok(a) => i32::from_str_radix(a,10).unwrap_or(0),
                    _ => unreachable!(),
                }
            }
            None => 0,
        }
    }
}

impl From<&WireBuffer> for u32 {
    fn from(wb: &WireBuffer) -> Self {
        match &wb.buffer {
            Some(buffer) => {
                match std::str::from_utf8(&buffer) {
                    Ok(a) => u32::from_str_radix(a,10).unwrap_or(0),
                    _ => unreachable!(),
                }
            }
            None => 0,
        }
    }
}


// implementations here that given a value, of a certain type will fill out the wirebuffer
// they should also pay attention to the schema and ensure that the types meet the correct specifciation
pub trait WireBufferFromReturnType<T> {
    fn from_rt(self: &mut Self, _: T);
}

impl WireBufferFromReturnType<String> for WireBuffer {
    fn from_rt(self: &mut Self, s: String) {
        // we've got a wire buffer object and we need to set the bytes here from the string
        self.buffer = Some(s.into_bytes());
    }
}

impl WireBufferFromReturnType<()> for WireBuffer {
    fn from_rt(self: &mut Self, _: ()) {
        self.buffer = None;
    }
}

impl WireBufferFromReturnType<bool> for WireBuffer {
    fn from_rt(self: &mut Self, b: bool) {
        self.buffer = match b {
            true => Some(b"true".to_vec()),
            false => Some(b"false".to_vec()),
        };
    }
}

impl WireBufferFromReturnType<i8> for WireBuffer {
    fn from_rt(self: &mut Self, s: i8) {
        // we've got a wire buffer object and we need to set the bytes here from the string
        self.buffer = Some(s.to_string().into_bytes());
    }
}

impl WireBufferFromReturnType<i16> for WireBuffer {
    fn from_rt(self: &mut Self, s: i16) {
        // we've got a wire buffer object and we need to set the bytes here from the string
        self.buffer = Some(s.to_string().into_bytes());
    }
}

impl WireBufferFromReturnType<i32> for WireBuffer {
    fn from_rt(self: &mut Self, s: i32) {
        // we've got a wire buffer object and we need to set the bytes here from the string
        self.buffer = Some(s.to_string().into_bytes());
    }
}

impl WireBufferFromReturnType<i64> for WireBuffer {
    fn from_rt(self: &mut Self, s: i64) {
        // we've got a wire buffer object and we need to set the bytes here from the string
        self.buffer = Some(s.to_string().into_bytes());
    }
}

impl WireBufferFromReturnType<isize> for WireBuffer {
    fn from_rt(self: &mut Self, s: isize) {
        // we've got a wire buffer object and we need to set the bytes here from the string
        self.buffer = Some(s.to_string().into_bytes());
    }
}

impl WireBufferFromReturnType<u8> for WireBuffer {
    fn from_rt(self: &mut Self, s: u8) {
        // we've got a wire buffer object and we need to set the bytes here from the string
        self.buffer = Some(s.to_string().into_bytes());
    }
}

impl WireBufferFromReturnType<u16> for WireBuffer {
    fn from_rt(self: &mut Self, s: u16) {
        // we've got a wire buffer object and we need to set the bytes here from the string
        self.buffer = Some(s.to_string().into_bytes());
    }
}

impl WireBufferFromReturnType<u32> for WireBuffer {
    fn from_rt(self: &mut Self, s: u32) {
        // we've got a wire buffer object and we need to set the bytes here from the string
        self.buffer = Some(s.to_string().into_bytes());
    }
}

impl WireBufferFromReturnType<u64> for WireBuffer {
    fn from_rt(self: &mut Self, s: u64) {
        // we've got a wire buffer object and we need to set the bytes here from the string
        self.buffer = Some(s.to_string().into_bytes());
    }
}

impl WireBufferFromReturnType<usize> for WireBuffer {
    fn from_rt(self: &mut Self, s: usize) {
        // we've got a wire buffer object and we need to set the bytes here from the string
        self.buffer = Some(s.to_string().into_bytes());
    }
}

impl WireBufferFromReturnType<f32> for WireBuffer {
    fn from_rt(self: &mut Self, s: f32) {
        // we've got a wire buffer object and we need to set the bytes here from the string
        self.buffer = Some(s.to_string().into_bytes());
    }
}

impl WireBufferFromReturnType<f64> for WireBuffer {
    fn from_rt(self: &mut Self, s: f64) {
        // we've got a wire buffer object and we need to set the bytes here from the string
        self.buffer = Some(s.to_string().into_bytes());
    }
}
