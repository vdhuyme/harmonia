run:
	cargo run

test:
	cargo test

build:
	cargo build --release

clean:
	cargo clean

.PHONY: run test build clean

help:
	@echo "Makefile commands:"
	@echo "  run   - Run the application"
	@echo "  test  - Run tests"
	@echo "  build - Build the application in release mode"
	@echo "  clean - Clean the build artifacts"
	@echo "  help  - Show this help message"