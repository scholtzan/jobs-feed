.PHONY: server client

run: client server

client:
	cd client/ && pnpm run build

server:
	cd server/ && cargo run

format:
	npx prettier --write client