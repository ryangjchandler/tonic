.DEFAULT_GOAL := default

default:
	cargo run --release

test:
	cargo test --lib