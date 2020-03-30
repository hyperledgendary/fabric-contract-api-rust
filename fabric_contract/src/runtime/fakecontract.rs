
use crate::contractapi::contract::Contract;
use crate::contractapi::ledger::Context;
use std::str;

#[derive(Debug)]
pub struct FakeContract {

}

impl Contract for FakeContract {

    fn name(&self) -> String {
        format!("FakeContract")
    }

    fn before_transaction(&self,_ctx: Context)  {
        println!("FakeContract customer Before_Transaction");
    }
}

impl FakeContract {

    pub fn create_asset(&self,mut ctx: Context, arg: String) -> String {
        // get a state
        ctx.create_state(arg.clone(),b"json data here".to_vec());
        let s = format!("created asset {}",arg.clone());
        s
    }

    pub fn get_asset(&self,mut ctx: Context, arg: String) -> String {
        let result = ctx.retrieve_state(String::from(arg));
        let s = format!("retrieved asset data is {}",result);
        s
    }
}