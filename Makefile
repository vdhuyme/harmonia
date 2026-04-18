# ================================
# CONFIG
# ================================

APP ?= api

POSTGRES_HOST_PORT ?= 55432
REDIS_HOST_PORT ?= 56379
API_HOST_PORT ?= 9003

DATABASE_URL ?= postgres://harmonia:harmonia@127.0.0.1:$(POSTGRES_HOST_PORT)/harmonia
REDIS_URL ?= redis://127.0.0.1:$(REDIS_HOST_PORT)

# ================================
# RUN
# ================================

run:
	cargo run -p $(APP)

run-local:
	DATABASE_URL=$(DATABASE_URL) \
	REDIS_URL=$(REDIS_URL) \
	API_HOST=0.0.0.0 \
	API_PORT=$(API_HOST_PORT) \
	cargo run -p api

# ================================
# DOCKER DEPS
# ================================

deps-up:
	docker compose -f docker-compose.yml -f docker-compose.host.yml up -d postgres redis pgadmin

deps-down:
	docker compose -f docker-compose.yml -f docker-compose.host.yml stop postgres redis pgadmin

deps-reset:
	docker compose -f docker-compose.yml -f docker-compose.host.yml down -v

# ================================
# DB WAIT
# ================================

wait-db:
	@echo "Waiting for Postgres..."
	@powershell -Command "\
	while (-not (Test-NetConnection -ComputerName 127.0.0.1 -Port $(POSTGRES_HOST_PORT) -WarningAction SilentlyContinue).TcpTestSucceeded) { \
		Start-Sleep -Seconds 1 \
	}"
	@echo "Postgres is ready!"

# ================================
# MIGRATIONS (FIXED - NO sea-orm-cli)
# ================================

migrate-up:
	DATABASE_URL=$(DATABASE_URL) cargo run -p db -- up

migrate-down:
	DATABASE_URL=$(DATABASE_URL) cargo run -p db -- down

migrate-reset:
	DATABASE_URL=$(DATABASE_URL) cargo run -p db -- reset

migrate-fresh:
	DATABASE_URL=$(DATABASE_URL) cargo run -p db -- fresh

migrate-gen:
	@if [ -z "$(name)" ]; then echo "Usage: make migrate-gen name=your_migration_name"; exit 1; fi
	cargo run -p db -- generate $(name)

# ================================
# DEV FLOW
# ================================

dev:
	make deps-up
	make wait-db
	make migrate-up
	make run-local

# ================================
# TOOLING
# ================================

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

openapi-gen:
	cargo run -p api -- --export-openapi docs/openapi.json

# ================================
# HELP
# ================================

.PHONY: run run-local deps-up deps-down deps-reset dev \
        migrate-up migrate-down migrate-reset migrate-fresh migrate-gen \
        test build clean check fmt clippy openapi-gen wait-db help

help:
	@echo "Makefile commands:"
	@echo ""
	@echo "Run:"
	@echo "  run              - Run a crate (default: api)"
	@echo "  run APP=workers  - Run workers"
	@echo ""
	@echo "Docker:"
	@echo "  deps-up          - Start postgres/redis/pgadmin"
	@echo "  deps-down        - Stop containers"
	@echo "  deps-reset       - Remove containers + volumes"
	@echo ""
	@echo "Database:"
	@echo "  migrate-up       - Run migrations"
	@echo "  migrate-down     - Rollback last migration"
	@echo "  migrate-reset    - Reset DB schema"
	@echo "  migrate-fresh    - Drop + reapply migrations"
	@echo "  migrate-gen      - Create migration"
	@echo ""
	@echo "Dev:"
	@echo "  dev              - Full dev flow"
	@echo ""
	@echo "Tooling:"
	@echo "  test             - Run tests"
	@echo "  build            - Build release"
	@echo "  clean            - Clean artifacts"
	@echo "  check            - Compile check"
	@echo "  fmt              - Format code"
	@echo "  clippy           - Lint code"
	@echo "  openapi-gen      - Generate OpenAPI spec"
