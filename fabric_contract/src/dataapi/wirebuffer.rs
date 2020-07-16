use super::{TypeSchema};
use std::fmt::Debug;

pub struct WireBuffer {
    pub buffer: Option<Vec<u8>>,
    pub schema: TypeSchema,
  //  pub converter: Box<dyn Converter>,
}

impl WireBuffer {
    pub fn new(buffer: Vec<u8>,schema: TypeSchema/*, converter: Box<dyn Converter>*/) -> Self {
        Self {
            buffer : Some(buffer),
            schema,
            //converter,
        }
    }

    pub fn new_unfilled(schema: TypeSchema/*, converter: Box<dyn Converter>*/) -> Self {
        Self {
            buffer: Option::None,
            schema,
           // converter,
        }
    }
}

impl Debug for WireBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WireBuffer: xxx")
    }
}

impl From<&WireBuffer> for String {
    fn from(wb: &WireBuffer) -> Self {
        match &wb.buffer {
            Some(buffer) => std::str::from_utf8(&buffer).unwrap().to_string(),
            None => "".to_string()
        }
    }
}

// implementations here that given a value, of a certain type will fill out the wirebuffer
// they should also pay attention to the schema and ensure that the types meet the correct specifciation
pub trait WireBufferFromReturnType<T> {
    fn from_rt(self: &mut Self, _: T) ;
}

impl WireBufferFromReturnType<String> for WireBuffer {
    fn from_rt(self: &mut Self, s: String) {
        // we've got a wire buffer object and we need to set the bytes here from the string
        self.buffer = Some(s.into_bytes());
   
    }
}

impl WireBufferFromReturnType<()> for WireBuffer {
    fn from_rt(self: &mut Self, _: ())  {
        self.buffer = None;
     
    }
}

impl WireBufferFromReturnType<bool> for WireBuffer {
    fn from_rt(self : &mut Self, b: bool)  {
       self.buffer = match b {
           true => Some("true".as_bytes().to_vec()),
           false => Some("false".as_bytes().to_vec())
       };
     
    }
}