clean:
	cargo clean -v

build:
	cargo build

run:
	cargo run

test: build
	cd tests &&	docker-compose up -d && sleep 3
	cargo test
