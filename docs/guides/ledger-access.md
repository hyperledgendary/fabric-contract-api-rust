---
title: Accessing Ledger State
layout: page
category: guides
---

Within transaction functions it is essential to be able access the ledger state; without this the transaction functions have little purpose.

The model in the Rust Contract API is based on the prototype Java & Node Ledger APIs.

Note this guide is assuming a basic understanding of Fabric concepts.

## Conceptual Model

Each transaction function that is run takes place within a transaction context. Within this context you can access the ledger (called the 'World State') or any private data collection. Data can be retrieved, updated, verified and the endorsment policies per state can be modified. You can also find out information about the submitter of the transaction to validate who or what entity has initiated the transactions. This guide will focus on the Ledger aspects.

## Ledger Access

### Collections
The starting point of the ledger access is choosing whether to access the World State or one of the Private Data Collections (either explicitly defined ones, or the implict organizational collections).

For example for World State:

```rust
  // get the collection that is backed by the world state
  let world = Ledger::access_ledger().get_collection(CollectionName::World);
```

Though at a low-level there are fundamental differences between the private data collections and world state, at the Contract level it is better to focus on the commonality of the operations. A collection provides access to states via a key. As well as the key, each state holds data. The format of which is not important to Peer, but is understandable by the contract. As with the original 'shim' API, your contract can still work with states as being a key - bytebuffer pair. But they can also handle more complex type and move to one side the serialization logic. This helps focus on the business logic to code rather than pure data handling.

- confirm the hash of a state
- change the endorsement policy

### CRUD Operations

The collections support the following 'crud' style operations. 

- create a new state
- update a state
- delete a state
- retrieve a state
- check a state exists

There are two style of CRUD operations, one that take the data as a BLOB (via a `Vec<u8>`) and one that uses generics and accepts types that implement the `DataType` trait. (Note that this includes the essential types, as the DataType trait has been implemented for String and the primitve number types).

**BLOB apis**

These apis, take a `String` as the key, and a `Vec<u8>` for the data either as an argument or a return value. Note the key should be the complete key - no processing is done it. 

**Datatype apis**

These apis, will take or return something that implements the DataType trait. Note that the trait itself will calculate the key. 

Typicaly these would be the APIs that primarily used, but when the contract wants to interact with the state-based endorsement for example access to the State object is needed. 

### Query states

Key range based queries are currently supported. Use the `get_states` api on the collection. This takes a `KeyQueryHandler` enum that defines the type of the query, defined as follows

```
pub enum KeyQueryHandler {
    /// Range(string,string) The start and end keys  
    Range(String, String),

    /// RangeFrom(string)    From the given key to the end
    RangeFrom(String),

    /// RangeTo(string)      From the start to the given key
    RangeTo(String),

    /// RangeAll(),  All composite keys. use with caution
    RangeAll(),
}
```

Note these are the raw keys; TODO: Improve handling here for datatypes. A complete example would be 

```
    #[Transaction(submit)]
    pub fn transfer_asset_by_color(&self,color: String, new_owner: String)-> Result<(),ContractError>{

        let world = Ledger::access_ledger().get_collection(CollectionName::World);     
        let index_key = State::make_composite_key(INDEX_NAME.into(), vec![color]);
        
        let states = world.get_states(KeyQueryHandler::RangeFrom(index_key))?;
        for s in states {
            let key = State::split_composite_key(s.key()).remove(1);            
            self.transfer_asset(key, new_owner.clone())?;
        }
        Ok(())

    }
```

This uses the raw key.

### DataType trait

Consider the struct

```
pub struct Asset {
  id: String,
  details: String
}
```

To use this as a DataType do the following

1 - Recommend to implement add  `#[derive(Serialize, Deserialize, Debug, Default)]`
2 - Impelment `impl DataType for Asset {..}`
3 - Implement `impl WireBufferFromReturnType<Asset> for WireBuffer { .. }`
4 - Implement `impl From<&WireBuffer> for Asset {..}`

TODO: Macro-ize these

## Transaction Access

This is a set of APIs that let the contract implementation find details of the submitter of the transaction.

### Transaction

To get the current transaction ID and timestampe

```rust
   let tx = fabric_contract::blockchain::TransactionContext::current_transaction();
   
   let client_msp = tx.get_submitting_identity()?.get_mspid();

   let id = tx.get_id();
   let timestamp = tx.get_timestamp();
   
```
