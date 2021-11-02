.DEFAULT_GOAL := default

default:
	cargo run --release

test:
	cargo test

sink:
	cargo run -- ./examples/sink.tn