# Makefile for tarzi - Rust-native lite search for AI applications

# Variables
CARGO = cargo
MATURIN = maturin
PYTHON = python3
RUST_TARGET = target/release/tarzi
PYTHON_PACKAGE = target/wheels/*.whl

# Default target
.PHONY: help
help: ## Show this help message
	@echo "Available commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

# Build commands
.PHONY: build
build: ## Build the Rust binary in release mode
	$(CARGO) build --release

.PHONY: build-debug
build-debug: ## Build the Rust binary in debug mode
	$(CARGO) build

.PHONY: build-python
build-python: ## Build Python wheel
	$(MATURIN) build --release

# Development commands
.PHONY: dev
dev: ## Run in development mode (debug build)
	$(CARGO) run

.PHONY: dev-release
dev-release: ## Run in development mode (release build)
	$(CARGO) run --release

.PHONY: watch
watch: ## Watch for changes and rebuild automatically
	$(CARGO) watch -x run

# Install commands
.PHONY: install
install: build-python ## Install Python package
	pip install $(PYTHON_PACKAGE)

.PHONY: install-dev
install-dev: ## Install Python package in development mode
	$(MATURIN) develop --release

.PHONY: install-rust
install-rust: build ## Install Rust binary
	cp $(RUST_TARGET) /usr/local/bin/tarzi

# Test commands
.PHONY: test
test: ## Run all tests
	$(CARGO) test --features test-helpers

.PHONY: test-unit
test-unit: ## Run unit tests only
	$(CARGO) test --lib --features test-helpers

.PHONY: test-integration
test-integration: ## Run integration tests
	$(CARGO) test --test '*' --features test-helpers

.PHONY: test-python
test-python: ## Run Python tests
	$(PYTHON) -m pytest tests/ -v

.PHONY: test-all
test-all: test test-python ## Run all tests (Rust + Python)

# Code quality commands
.PHONY: check
check: ## Run cargo check
	$(CARGO) check

.PHONY: clippy
clippy: ## Run clippy linter
	$(CARGO) clippy -- -D warnings

.PHONY: fmt
fmt: ## Format code with rustfmt
	$(CARGO) fmt

.PHONY: fmt-check
fmt-check: ## Check code formatting
	$(CARGO) fmt -- --check

.PHONY: lint
lint: clippy fmt-check ## Run all linters

# Clean commands
.PHONY: clean
clean: ## Clean build artifacts
	$(CARGO) clean
	rm -rf target/wheels/

.PHONY: clean-all
clean-all: clean ## Clean everything including dependencies
	rm -rf target/
	rm -rf .venv/
	rm -rf __pycache__/
	rm -rf *.egg-info/

# Documentation commands
.PHONY: doc
doc: ## Generate documentation
	$(CARGO) doc --no-deps --open

.PHONY: doc-build
doc-build: ## Build documentation without opening
	$(CARGO) doc --no-deps

# Release commands
.PHONY: release
release: ## Build release artifacts
	$(CARGO) build --release
	$(MATURIN) build --release

.PHONY: publish
publish: ## Publish to crates.io (use with caution!)
	$(CARGO) publish

.PHONY: publish-python
publish-python: ## Publish Python package to PyPI
	twine upload $(PYTHON_PACKAGE)

# Utility commands
.PHONY: update
update: ## Update dependencies
	$(CARGO) update

.PHONY: outdated
outdated: ## Check for outdated dependencies
	$(CARGO) outdated

.PHONY: tree
tree: ## Show dependency tree
	$(CARGO) tree

# Development setup
.PHONY: setup
setup: ## Setup development environment
	rustup update
	$(CARGO) install cargo-watch
	$(CARGO) install cargo-outdated
	pip install maturin pytest twine

# Quick development workflow
.PHONY: quick
quick: check test build ## Quick development check (check + test + build)

.PHONY: full-check
full-check: lint test-all build ## Full development check (lint + test + build) 