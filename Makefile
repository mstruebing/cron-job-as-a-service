all: lint test build run

build:
	cargo build

build-release:
	cargo build --release

run:
	$(MAKE) -j2 run-api run-runner

run-api:
	cargo run --bin api

run-runner:
	cargo run --bin runner

test:
	cargo test

clean: clean-target clean-db

clean-target:
	cargo clean

clean-db:
	sudo rm -r .data

lint:
	cargo clippy -- -D warnings
	cargo fmt --all -- --check

start-db:
	docker-compose up

db-connect:
	docker-compose exec postgresql 'psql --user postgres cronjob_as_a_service'

db-reset:
	cd shared && (diesel migration redo || diesel migration run)
