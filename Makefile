.PHONY: server client

run: server client

client:
	cd client/ && pnpm run dev

server:
	cd server/ && cargo run
