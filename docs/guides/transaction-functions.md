---
title: Transaction Functions
layout: page
category: guides
---

This guide explains how individual functions can be marked as transaction functions. This means that they are invokeable from the Client SDKs.

## Marking as a transaction function

Use the macro `#[Transaction]` on the functions, the functions must be public and not associated functions. Marking any function this was does not prohibit it being called as a 'normal' function, any ledger work down will be under the same Fabric Transaction. 

**Arguments and Return**

These should be either the primitive types, (number and string) or a type that implements the DataType trait. For example:

```rust
    pub fn change_public_description(
        &self,
        asset_id: String,
        public_desc: String,
    ) 
```


The return should be a Result, passing a ContractError and any other return type needed. 
`Result<(), ContractError>` or `Result<String, ContractError>`

**Submit or Evaluate**
To indicate if this should a tranasction that is submited for ordering, or just evaluated (query type) . Use the transaction macro as follows. `#[Transaction(submit)]` or `#[Transaction(evaluate)]`
Submit is the default if it's not specified.

**Transient Data**

Transient data is passed to the transaction function as 

```
   #[Transaction(submit, transient = "price")]
    pub fn agree_to_sell(&self, asset_id: String, price: u32) -> Result<(), ContractError> { .. }
```