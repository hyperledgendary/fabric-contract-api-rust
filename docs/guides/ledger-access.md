---
title: Accessing Ledger State
layout: page
category: guides
---

Within transaction functions it is essential to be able access the ledger state; without this the transaction functions have little purpose.

The model in the Rust Contract API is based on the prototype Java & Node Ledger APIs.

Note this guide is assuming a basic understanding of Fabric concepts.

## Conceptual Model

Each transaction function that is run takes placed within a transaction context. Within this context you can access the ledger (called the 'World State') or any private data collection. Data can be retrieved, updated, verified and the endorsment policies per state can be modified.

You can also find out information about the submitter of the transaction to validate who or what entity has initiated the transactions. 

## Ledger Access

### Collections
The starting point of the ledger access is choosing whether to access the World State or one of the Private Data Collections (either explicitly defined ones, or the implicty organizational collections).

For example for World State:

```rust
  // get the collection that is backed by the world state
  let world = Ledger::access_ledger().get_collection(CollectionName::World);
```

Each collection has methods on it (directly or indirectly via the state object) to 
- create a new state
- update a state
- delete a state
- retrieve a state
- check a state exists
- confirm the hash of a state
- change the endorsement policy

### CRUD Operations

There are two style of CRUD operations, one that take the data as a BLOB (via a `Vec<u8>`) and one that uses generics and accepts types that implement the `DataType` trait

### State operations

implemented

### Query states

not yet implemented in the codebase

### DataType trait

implemented


## Transaction Access

This is a set of APIs that let the contract implementation find details of the submitter of the transaction.

### Transaction

To get the current transaction ID and timestampe

```rust
  let tx = fabric_contract::blockchain::Transaction::current_transaction();
 
  let id = tx.get_id();
  let timestamp = tx.get_timestamp();
  let mspid = tx.get_peer_mspid();
```

### Client Identity

This returns information about the submitter of the transaction.