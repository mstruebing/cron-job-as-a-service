build:
	cargo build

build-release:
	cargo build --release

run:
	cargo run

test:
	cargo test

clean:
	rm -rf target

lint:
	cargo clippy -- -D warnings
	cargo fmt --all -- --check

start-db:
	docker-compose up
