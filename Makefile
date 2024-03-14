build: test README.md
	cargo build --release
	cargo doc

test:
	cargo clippy
	cargo test

update:
	cargo upgrade --incompatible
	cargo update

check:
	cargo outdated
	cargo audit

all: update check build

install:
	cargo install --path .

install-dev:
	cargo install cargo-audit cargo-edit cargo-outdated

clean:
	cargo clean

.PHONY: build test update check all install install-dev clean

