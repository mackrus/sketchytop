.PHONY: build test fmt clippy clean

build:
	cargo build

test:
	cargo test

fmt:
	cargo fmt

clippy:
	cargo clippy

clean:
	cargo clean
