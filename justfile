# justfile - very simple task running
# https://github.com/casey/just

wasm:
    cargo build --target wasm32-unknown-unknown

amd64:
    cargo build


docs:
    cargo doc --no-deps --open
    cp -r ./target/doc/* ./docs/

# use expand with the expand crate when debugging macros
expand:
    cargo expand --package basic_contract_rs
  
    
