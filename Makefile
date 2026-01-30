all:
	cargo install cargo-expand
	cargo run
	cargo expand --lib
	cargo expand --bin assert-repo
