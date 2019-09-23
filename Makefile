build:
	cargo build

run:
	cargo run

test:
	cargo test

clean:
	rm -rf target

start-db:
	docker-compose up
