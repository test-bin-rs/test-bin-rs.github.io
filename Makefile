all: assert-true test-bin

assert-true:
	cargo install --git https://github.com/test-bin-rs/test-bin-rs.github.io --branch assert-true --root .cargo
	file .cargo/bin/assert-true
	.cargo/bin/assert-true

test-bin:
	cargo install --git https://github.com/test-bin-rs/test-bin-rs.github.io --branch doc --root .cargo
	file .cargo/bin/test-bin
	.cargo/bin/test-bin

clean:
	rm -rf .cargo
