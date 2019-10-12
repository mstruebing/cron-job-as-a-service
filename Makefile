all: lint test build run

build:
	cargo build

build-release:
	cargo build --release

run:
	cargo run --bin cass-runner
	cargo run --bin cass-api

test:
	cargo test

clean:
	rm -rf target

lint:
	cargo clippy -- -D warnings
	cargo fmt --all -- --check

start-db:
	docker-compose up

db-connect:
	docker-compose exec postgresql 'psql --user postgres cronjob_as_a_service'
