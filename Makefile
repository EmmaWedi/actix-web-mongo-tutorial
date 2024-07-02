setup:
	cargo init

install:
	cargo add actix-web chrono futures-util mongodb serde

run:
	docker-compose up -d
	cargo run

stop:
	docker-compose down

watch:
	docker-compose up -d
	cargo watch -c -w src -x run

build: 
	cargo build