# Run using just (make deriative written in Rust)
#
# https://github.com/casey/just
#

wasm:
	cargo build --target wasm32-unknown-unknown

amd64:
	cargo build

docs:
	cargo doc --no-deps --open
	cp -r ./target/doc/* ./docs/apidoc/

# use expand with the expand crate when debugging macros
expand:
	cargo expand --package basic_contract_rs

clippy:
	cargo clippy --all-targets --all-features

fmt:
	cargo fmt --all	

clean:
	cargo clean

all: clean wasm fmt clippy docs
