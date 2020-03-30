use crate::contractapi::ledger::Context;

pub trait Contract {

    fn name(&self) -> String;

    fn before_transaction(&self, ctx: Context);
    fn after_transaction(&self, _ctx: Context) {
            println!("Default After Tranasction");
    }
}