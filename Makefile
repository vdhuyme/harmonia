# Default crate (can be overridden with APP variable)
APP ?= api
POSTGRES_HOST_PORT ?= 55432
REDIS_HOST_PORT ?= 56379
API_HOST_PORT ?= 9003

run:
	cargo run -p $(APP)

# Run API on Windows host while DB/Redis stay in Docker containers.
run-local:
	DATABASE_URL=postgres://harmonia:harmonia@127.0.0.1:$(POSTGRES_HOST_PORT)/harmonia REDIS_URL=redis://127.0.0.1:$(REDIS_HOST_PORT) API_HOST=0.0.0.0 API_PORT=$(API_HOST_PORT) cargo run -p api

# Start only DB/Redis for local host development (ports mapped to localhost only).
deps-up:
	docker compose -f docker-compose.yml -f docker-compose.host.yml up -d postgres redis

deps-down:
	docker compose -f docker-compose.yml -f docker-compose.host.yml stop postgres redis

test:
	cargo test --workspace

build:
	cargo build --workspace --release

clean:
	cargo clean

check:
	cargo check --workspace

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all -- -D warnings

# Usage: make run APP=your-crate-name

.PHONY: run run-local deps-up deps-down test build clean help check fmt clippy

help:
	@echo "Makefile commands:"
	@echo "  run        - Run a crate (default: api)"
	@echo "               make run APP=workers"
	@echo "  deps-up    - Start postgres/redis in Docker for local host run"
	@echo "  deps-down  - Stop postgres/redis Docker dependencies"
	@echo "  run-local  - Run API on host with Docker DB/Redis (localhost:9003)"
	@echo "               make run-local POSTGRES_HOST_PORT=55432 REDIS_HOST_PORT=56379 API_HOST_PORT=9003"
	@echo "  test       - Run all tests"
	@echo "  build      - Build all crates (release)"
	@echo "  clean      - Clean build artifacts"
	@echo "  check      - Check compile بسرعة"
	@echo "  fmt        - Format code"
	@echo "  clippy     - Lint code"
	@echo "  help       - Show this help"
