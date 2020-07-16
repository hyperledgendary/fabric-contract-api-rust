
use std::fmt;
use std::error;

/// Contract Error is what the contract will return to indicate an error
/// Typically this would be for contract developers to use to mark that a failure
/// has occured, and give information about what that means in the context
/// of their implementation
#[derive(Debug)]
pub struct ContractError {
    msg: String,
    ledger_error: Option<LedgerError>
}

impl std::convert::From<String> for ContractError {
    fn from(msg: String) -> Self {
       Self { msg,  ledger_error: Option::None }
     }
}

impl std::convert::From<LedgerError> for ContractError {
    fn from(ledger_error: LedgerError) -> Self {
       Self { msg:"Error caused by LedgerError".to_string(), ledger_error: Some(ledger_error) }
     }
}

impl fmt::Display for ContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.ledger_error {
            Some(le) =>  write!(f, "{} caused by {}", self.msg,le),
            None => write!(f, "{}", self.msg),
        }
     
    }
}

impl error::Error for ContractError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(self)
    }
}

/// Ledger Error is returned by the API calls made against the ledger
/// api, eg, if a state can be found
#[derive(Debug)] 
pub struct LedgerError {
    msg: String
}

impl error::Error for LedgerError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(self)
    }
}

impl std::convert::From<String> for LedgerError {
    fn from(msg: String) -> Self {
       Self { msg }
     }
}


impl fmt::Display for LedgerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "{}", self.msg)
    }
}
