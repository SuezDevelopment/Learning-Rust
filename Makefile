all: build run


build:
	cd client && npm run build

run:
	RUST_LOG=debug cargo watch -x 'run'

dev:
	cd client && npm run dev & cargo watch -x 'run'

curl:
	curl -H "Authorization: Bearer valid_token" http://localhost:9080/api/temperature/room1

.PHONY: curl

health:
	curl -H "Authorization: Bearer valid_token" http://localhost:9080/health

.PHONY: health

sock:
	wscat -c ws://127.0.0.1:9080/ws/ --protocol 13 -H "Authorization: Bearer vali_token"