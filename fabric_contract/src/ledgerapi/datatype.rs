#![allow(unused_variables)]

use crate::ledgerapi::state::State;

pub trait DataType: Default {
    /// Converts into a state that can be handled and put into
    /// the ledger and private collections
    fn to_state(&self) -> State;

    ///
    fn get_key(&self) -> String;

    ///
    fn build_from_state(state: State) -> Self;

    ///
    fn form_key(k: &String) -> String;
}
