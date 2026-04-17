# Default crate (can be overridden with APP variable)
APP ?= api

run:
	cargo run -p $(APP)

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

.PHONY: run test build clean help check fmt clippy

help:
	@echo "Makefile commands:"
	@echo "  run        - Run a crate (default: api)"
	@echo "               make run APP=workers"
	@echo "  test       - Run all tests"
	@echo "  build      - Build all crates (release)"
	@echo "  clean      - Clean build artifacts"
	@echo "  check      - Check compile بسرعة"
	@echo "  fmt        - Format code"
	@echo "  clippy     - Lint code"
	@echo "  help       - Show this help"
