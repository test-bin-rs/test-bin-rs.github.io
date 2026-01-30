all:
	cargo install cargo-expand
	cargo expand --lib
	cargo expand --bin test-bin-tokio-starter
	cargo run
