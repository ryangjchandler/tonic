.DEFAULT_GOAL := default

default:
	cargo run --release

test:
	cargo test --lib

sink:
	cargo run -- ./examples/sink.tn