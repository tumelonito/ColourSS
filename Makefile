.PHONY: all build run test clean fmt clippy check release help

# Default target
all: build

# Build the project
build:
	cargo build

# Build for release
release:
	cargo build --release

# Run the project with arguments
# Example: make run ARGS="--help"
# Example: make run ARGS="parse my_colors.txt"
run:
	cargo run -- $(ARGS)

# Run tests
test:
	cargo test

# Format code
fmt:
	cargo fmt

# Lint code
clippy:
	cargo clippy -- -D warnings # be strict

# Run all checks before committing
check: fmt clippy test
	@echo "All checks passed!"

# Clean build artifacts
clean:
	cargo clean

# Help target
help:
	@echo "Available commands:"
	@echo "  make build         - Build the project"
	@echo "  make release       - Build for release"
	@echo "  make run ARGS=...  - Run the project (e.g., make run ARGS=\"parse colors.txt\")"
	@echo "  make test          - Run unit tests"
	@echo "  make fmt           - Format code"
	@echo "  make clippy        - Lint code"
	@echo "  make check         - Run all checks (fmt, clippy, test)"
	@echo "  make clean         - Clean build artifacts"

# Це імітує публікацію. Справжня команда: cargo publish
publish: check release
	@echo "Project is ready to be published!"
	@echo "Run 'cargo publish' to release."