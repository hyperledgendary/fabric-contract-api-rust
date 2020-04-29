# justfile - very simple task running
# https://github.com/casey/just

wasm:
    cargo build --target wasm32-unknown-unknown

docs:
    cargo docs --no-deps

# use expand with the expand crate when debugging macros
expand:
    cargo expand --package basic_contract_rs
  
    