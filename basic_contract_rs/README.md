# basic_contract_rs

To create a basic Smart Contract.

- Create a Rust library crate
- add the following dependencies

```
fabric_contract = { path = "../fabric_contract" }
log = "0.4.8"
wapc-guest = "0.3.1"
```

- In the lib.rs of the your crate, register the contracts that will be required, by passing the a function that will create a new instance of the contract.

```rust
fabric_contract::register!( AssetContract::new );
```

- All contracts must implement the Contract trait
- The `#[Contract_Impl]` and `#[Transaction()]` macros should be used on the contract implementation, and the transaction functions
- Compile to Wasm