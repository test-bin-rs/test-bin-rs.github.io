all: 	assert-repo assert-true test-bin test-bin-tokio-starter

assert-repo:
	cargo install --git https://github.com/test-bin-rs/test-bin-rs.github.io --branch assert-repo --root .cargo
	file .cargo/bin/assert-repo
	.cargo/bin/assert-repo

assert-true:
	cargo install --git https://github.com/test-bin-rs/test-bin-rs.github.io --branch assert-true --root .cargo
	file .cargo/bin/assert-true
	.cargo/bin/assert-true

test-bin:
	cargo install --git https://github.com/test-bin-rs/test-bin-rs.github.io --branch doc --root .cargo
	file .cargo/bin/test-bin
	.cargo/bin/test-bin

test-bin-tokio-starter:
	cargo install --git https://github.com/test-bin-rs/test-bin-rs.github.io --branch test-bin-tokio-starter --root .cargo
	file .cargo/bin/test-bin-tokio-starter
	.cargo/bin/test-bin-tokio-starter

clean:
	rm -rf .cargo
