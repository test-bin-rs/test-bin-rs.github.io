all:
	cargo update
	cargo test
	cargo run --bin test-bin
	cargo run --bin test-assert-repo
