all: lint test build-docker build run

build:
	cargo build

build-docker:
	docker build -t caas:latest -f ./misc/Dockerfile ./misc

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

db-start:
	docker-compose -f misc/docker-compose.yml up

db-connect:
	docker-compose exec postgresql 'psql --user postgres cronjob_as_a_service'

db-reset:
	cd shared && (diesel migration redo || diesel migration run)
