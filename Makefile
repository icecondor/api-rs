.PHONY: all test
	
all:
	cargo build

run: all
	./target/debug/api-rs

format:
	find src tests -type f -exec rustfmt {} \;

test:
	cargo test
