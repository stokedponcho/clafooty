clean:
	cargo clean -v
	cd football_data && make clean

build:
	cargo build

run:
	cargo run

test: build
	cargo test
	cd football_data && make test

test-acceptance: build
	cargo test -- --ignored

install:
	cargo install --path .
