#![allow(unused_variables)]

use crate::ledgerapi::state::State;

pub trait DataType {

    /// Converts into a state that can be handled and put into
    /// the ledger and private collections
    fn to_state(&self) -> &State;

    /// 
    fn get_key(&self) -> String; 

}

pub trait LedgerSerializer {
    fn restore_from( );
    fn encode_to();    
}

pub trait WireSerializer {
    fn restore_from( );
    fn encode_to();
}