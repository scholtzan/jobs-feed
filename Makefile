ENVIRONMENT=debug

.PHONY: server client

run: client server

build: build_client build_server

build_client:
	cd client/ && npm install . && npm run build

build_server:
	cd server/ && cargo build

client:
	cd client/ && npm run build:watch

server:
	cd server/ && cargo run -- $(ENVIRONMENT)

format:
	npx prettier --write client && cargo fmt --manifest-path server/Cargo.toml
