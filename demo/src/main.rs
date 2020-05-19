use fabric_contract::contract::Ledger;
use fabric_contract::contract::CollectionName;

fn main() {
    println!("Hello, world!");

    let ledger = Ledger::access_ledger();
    let collection = ledger.get_collection(CollectionName::World);
    
    let state = collection.create_state(String::from("asset001"),String::from("asset value").into_bytes());

    let v = state.value();
    
    println!("{:?}",String::from_utf8(v));
    // collection.get_state(String::from("asset001"))
}
