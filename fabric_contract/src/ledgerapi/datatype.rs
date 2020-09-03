#![allow(unused_variables)]

use crate::ledgerapi::state::State;

pub trait DataType: Default {
    /// Converts into a state that can be handled and put into
    /// the ledger and private collections
    fn to_state(&self) -> State;

    ///
    fn get_key(&self) -> String;

    ///
    // fn from_state(&mut self, state: State);

    ///
    fn build_from_state(state: State) -> Self;
}
