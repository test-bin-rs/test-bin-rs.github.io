all: assert-true

assert-true:
	cargo install --git https://github.com/test-bin-rs/test-bin-rs.github.io --branch assert-true --root .cargo
	.cargo/bin/assert-true

clean:
	rm -rf .cargo
