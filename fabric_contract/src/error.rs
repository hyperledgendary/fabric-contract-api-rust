
use std::fmt;
use std::error;

#[derive(Debug)]
pub struct ContractError {
    msg: String
}

impl std::convert::From<String> for ContractError {
    fn from(msg: String) -> Self {
       Self { msg }
     }
}

impl fmt::Display for ContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl error::Error for ContractError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(self)
    }
}