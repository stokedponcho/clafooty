clean:
	cargo clean -v
	cd football_data && make clean

build:
	cargo build

run: build
	cargo run

test: build
	cargo test
	cd football_data && make test
	cd sport_data && make test

test-acceptance: build
	cargo test -- --ignored

install:
	cargo install --path .
