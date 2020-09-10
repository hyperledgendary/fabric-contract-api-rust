/*
 * SPDX-License-Identifier: Apache-2.0
 */
use super::{DataType, StateBasedEndorsement};
use crate::{contract::LedgerError, runtimeapi::LedgerService};
use fabric_ledger_protos::ledger_messages;

///
/// A State is the combination of key and value that are contained within a collection.
///
/// State-based endorsement should be set on this object as well.
///
///  Represents the things that are contained within the Collections.
///
#[derive(Default)]
pub struct State {
    key: String,
    data: Vec<u8>,
    collection_name: Option<String>,
}

impl State {
    /// Make a composite key.
    ///
    /// # Remarks
    ///
    /// Takes a set of strings and arranges them in a ':' separate single string
    ///
    /// # Arguments
    ///
    /// - `keytype`     The first part of the key. Referred to as 'type' but is just the 1st string
    ///                 part of the key.
    /// - `attributes`  Vector of Strings to make the rest of the ky
    pub fn make_composite_key(keytype: String, attributes: Vec<String>) -> String {
        todo!("make key");
    }

    /// Splits a composite key
    ///
    /// Splits the composite key into the parts, the first is the
    /// string that is passed in as the keytype above
    pub fn split_composite_key(key: String) -> Vec<String> {
        todo!("make key");
    }

    /// Creates a new state
    pub fn new(key: String, data: Vec<u8>, collection_name: String) -> State {
        State {
            key,
            data,
            collection_name: Some(collection_name),
        }
    }

    /// Get the buffer that  this state
    pub fn value(&self) -> Vec<u8> {
        self.data.clone()
    }

    /// Get the key that used for this state
    pub fn key(&self) -> String {
        self.key.clone()
    }

    /// Returns an iterator of the state history for this state
    pub fn get_history(&self) /* -> TODO  Iterator of StateHistory */ {}

    /// gets the private hash for this stae
    pub fn get_hash(&self) ->Result<Vec<u8>,LedgerError> {
        let name = self
        .collection_name
        .clone()
        .ok_or(LedgerError::from("Put to a colletion first".to_string()))?;

        LedgerService::get_hash(&self.key, &name)

    }

    /// Sets the State Based Endorsment for this state
    pub fn set_endorsment(&self, sbe: StateBasedEndorsement) -> Result<(), LedgerError> {
        let name = self
            .collection_name
            .clone()
            .ok_or(LedgerError::from("Put to a colletion first".to_string()))?;
        LedgerService::set_endorsement_policy(&self.key, &name, sbe)
    }

    pub fn get_endorsement(&self) -> Result<Option<StateBasedEndorsement>, LedgerError> {
        let name = self
            .collection_name
            .clone()
            .ok_or(LedgerError::from("Put to a colletion first".to_string()))?;

        match LedgerService::get_endorsement_policy(&self.key, &name) {
            Ok(e) => Ok(Some(e)),
            Err(e) => Err(e),
        }
    }
}

impl From<()> for State {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

/// Implementation of converting to a state from a datatype
///
/// # Example
///
/// ```ignore
/// let myAsset = MyAsset::new();
/// state.from(myAsset);
///
/// impl From<Box<dyn DataType>> for State {
///     fn from(_:Box<dyn DataType>) -> Self {
///        Self::default()
///     }
/// }
/// ```

impl From<(String, Vec<u8>)> for State {
    fn from((a, b): (String, Vec<u8>)) -> Self {
        Self {
            key: a,
            data: b,
            collection_name: Option::None,
        }
    }
}

impl From<&ledger_messages::State> for State {
    fn from(lms: &ledger_messages::State) -> Self {
        Self {
            key: lms.get_key().to_string(),
            data: lms.get_value().to_vec(),
            collection_name: Option::None,
        }
    }
}

impl From<(&ledger_messages::State, String)> for State {
    fn from((lms, c): (&ledger_messages::State, String)) -> Self {
        Self {
            key: lms.get_key().to_string(),
            data: lms.get_value().to_vec(),
            collection_name: Some(c),
        }
    }
}

impl From<ledger_messages::State> for State {
    fn from(lms: ledger_messages::State) -> Self {
        Self {
            key: lms.get_key().to_string(),
            data: lms.get_value().to_vec(),
            collection_name: Option::None,
        }
    }
}

impl std::convert::From<State> for fabric_ledger_protos::ledger_messages::State {
    fn from(_: State) -> Self {
        todo!()
    }
}

pub struct StateHash {
    pub hash: Vec<u8>,
}

pub trait VerifyHashConsistency<T> {
    fn verify_consistent(&self, o: T) -> Result<bool, LedgerError>;
}

impl VerifyHashConsistency<StateHash> for StateHash {
    fn verify_consistent(&self, o: StateHash) -> Result<bool, LedgerError> {
        todo!()
    }
}

// impl VerifyHashConsistency<State> for State {
//     fn verify_consistent(&self, o: State) -> Result<bool, LedgerError> {
//         todo!()
//     }
// }

impl<T: DataType> VerifyHashConsistency<T> for StateHash {
    fn verify_consistent(&self, o: T) -> Result<bool, LedgerError> {
        todo!()
    }
}
