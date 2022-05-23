init:
	cargo check

candid:
	cargo run > block_on.did

build: candid
	dfx ping local || dfx start --clean --background
	dfx canister create block_on
	dfx build block_on

local: build
	dfx deploy block_on

stop-replica:
	dfx stop

format:
	cargo fmt --all

lint:
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings -D clippy::all

clean:
	cargo clean
