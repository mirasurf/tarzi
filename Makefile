# Makefile for tarzi - Rust-native lite search for AI applications

# Variables
CARGO = cargo
MATURIN = maturin
PYTHON = python3
PYTEST = pytest
RUST_TARGET = target/release/tarzi
PYTHON_PACKAGE = target/wheels/*.whl
PYTHON_TEST_DIR = tests/python
PYTHON_MODULES = examples tests/python

# Colors for output
BLUE = \033[34m
GREEN = \033[32m
RED = \033[31m
RESET = \033[0m

# =============================================================================
# HELP
# =============================================================================

.PHONY: help
help: ## Show this help message
	@echo "Available commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

# =============================================================================
# BUILD COMMANDS
# =============================================================================

.PHONY: build
build: ## Build Rust binary in release mode
	$(CARGO) build --release

.PHONY: build-debug
build-debug: ## Build Rust binary in debug mode
	$(CARGO) build

.PHONY: build-python
build-python: ## Build Python wheel
	$(MATURIN) build --release

.PHONY: build-all
build-all: build build-python ## Build everything (Rust + Python)

# =============================================================================
# INSTALL COMMANDS
# =============================================================================

.PHONY: install
install: build ## Install Rust binary
	cp $(RUST_TARGET) /usr/local/bin/tarzi

.PHONY: install-python
install-python: build-python ## Install Python package
	pip install $(PYTHON_PACKAGE)

.PHONY: install-python-dev
install-python-dev: ## Install Python package in development mode
	$(MATURIN) develop --release

.PHONY: install-all
install-all: install install-python ## Install everything (Rust + Python)

# =============================================================================
# TEST COMMANDS
# =============================================================================

.PHONY: test
test: ## Run all Rust tests
	$(CARGO) test --features test-helpers

.PHONY: test-unit
test-unit: ## Run Rust unit tests only
	$(CARGO) test --lib --features test-helpers

.PHONY: test-integration
test-integration: ## Run Rust integration tests
	$(CARGO) test --test '*' --features test-helpers

.PHONY: test-python
test-python: install-python-dev ## Run all Python tests
	pip install -e .[test]
	cd $(PYTHON_TEST_DIR) && $(PYTEST)

.PHONY: test-python-unit
test-python-unit: ## Run Python unit tests only
	pip install -e .[test]
	cd $(PYTHON_TEST_DIR) && $(PYTEST) unit/ -m unit

.PHONY: test-python-integration
test-python-integration: install-python-dev ## Run Python integration tests
	pip install -e .[test]
	cd $(PYTHON_TEST_DIR) && $(PYTEST) integration/ -m integration

.PHONY: test-python-coverage
test-python-coverage: install-python-dev ## Run Python tests with coverage
	pip install -e .[test]
	cd $(PYTHON_TEST_DIR) && $(PYTEST) --cov=tarzi --cov-report=html --cov-report=term

.PHONY: test-all
test-all: test test-python ## Run all tests (Rust + Python)

# =============================================================================
# CODE QUALITY COMMANDS
# =============================================================================

.PHONY: check
check: ## Run cargo check (Rust only)
	$(CARGO) check

.PHONY: clippy
clippy: ## Run clippy linter (Rust only)
	$(CARGO) clippy -- -D warnings

.PHONY: format
format: ## Format Rust code with rustfmt
	$(CARGO) fmt

.PHONY: format-check
format-check: ## Check Rust code formatting
	$(CARGO) fmt -- --check

.PHONY: format-python
format-python: ## Format Python code (autoflake, isort, black)
	@echo "$(BLUE)🎨 Formatting Python code...$(RESET)"
	@pip install -e .[dev] > /dev/null 2>&1
	@autoflake --in-place --recursive --remove-all-unused-imports --remove-unused-variables $(PYTHON_MODULES)
	@isort $(PYTHON_MODULES)
	@black $(PYTHON_MODULES)
	@echo "$(GREEN)✅ Python code formatting complete!$(RESET)"

.PHONY: format-python-check
format-python-check: ## Check if Python code is properly formatted
	@echo "$(BLUE)🔍 Checking Python code formatting...$(RESET)"
	@pip install -e .[dev] > /dev/null 2>&1
	@black --check $(PYTHON_MODULES) || (echo "$(RED)❌ Black formatting check failed. Run 'make format-python' to fix.$(RESET)" && exit 1)
	@isort --check-only $(PYTHON_MODULES) || (echo "$(RED)❌ Import sorting check failed. Run 'make format-python' to fix.$(RESET)" && exit 1)
	@echo "$(GREEN)✅ Python code formatting check passed!$(RESET)"

.PHONY: format-all
format-all: format format-python ## Format all code (Rust + Python)

.PHONY: format-check-all
format-check-all: format-check format-python-check ## Check all code formatting (Rust + Python)

.PHONY: lint
lint: clippy format-check ## Lint Rust code
	@echo "$(GREEN)✅ Rust linting passed!$(RESET)"

.PHONY: lint-python
lint-python: ## Lint Python code with ruff
	@echo "$(BLUE)🔍 Running Python linters...$(RESET)"
	@pip install -e .[dev] > /dev/null 2>&1
	@ruff check $(PYTHON_MODULES)
	@echo "$(GREEN)✅ Python linting passed!$(RESET)"

.PHONY: lint-python-fix
lint-python-fix: ## Auto-fix Python linting issues
	@echo "$(BLUE)🔧 Auto-fixing Python linting issues...$(RESET)"
	@pip install -e .[dev] > /dev/null 2>&1
	@autoflake --in-place --recursive --remove-all-unused-imports --remove-unused-variables $(PYTHON_MODULES)
	@ruff check --fix $(PYTHON_MODULES)
	@echo "$(GREEN)✅ Python linting fixes applied!$(RESET)"

.PHONY: lint-all
lint-all: lint lint-python ## Lint all code (Rust + Python)

.PHONY: quality
quality: format-check-all lint-all ## Run all quality checks
	@echo "$(GREEN)🎉 All quality checks passed!$(RESET)"

# =============================================================================
# CLEAN COMMANDS
# =============================================================================

.PHONY: clean
clean: ## Clean Rust build artifacts
	$(CARGO) clean
	rm -rf target/wheels/

.PHONY: clean-python
clean-python: ## Clean Python test artifacts
	rm -rf $(PYTHON_TEST_DIR)/.pytest_cache
	rm -rf $(PYTHON_TEST_DIR)/htmlcov
	rm -rf $(PYTHON_TEST_DIR)/.coverage
	find $(PYTHON_TEST_DIR) -name "__pycache__" -type d -exec rm -rf {} +
	find $(PYTHON_TEST_DIR) -name "*.pyc" -delete

.PHONY: clean-all
clean-all: clean clean-python ## Clean everything including dependencies
	rm -rf target/
	rm -rf .venv/
	rm -rf __pycache__/
	rm -rf *.egg-info/

# =============================================================================
# DOCUMENTATION COMMANDS
# =============================================================================

.PHONY: doc
doc: ## Generate and open Rust documentation
	$(CARGO) doc --no-deps --open

.PHONY: doc-build
doc-build: ## Build Rust documentation without opening
	$(CARGO) doc --no-deps

# =============================================================================
# RELEASE COMMANDS
# =============================================================================

.PHONY: release
release: ## Build release artifacts (Rust binary)
	$(CARGO) build --release

.PHONY: release-python
release-python: ## Build Python release artifacts
	$(MATURIN) build --release

.PHONY: release-all
release-all: release release-python ## Build all release artifacts (Rust + Python)

.PHONY: publish
publish: ## Publish Rust crate to crates.io (use with caution!)
	$(CARGO) publish

.PHONY: publish-python
publish-python: ## Publish Python package to PyPI
	twine upload $(PYTHON_PACKAGE)

# =============================================================================
# UTILITY COMMANDS
# =============================================================================

.PHONY: update
update: ## Update Rust dependencies
	$(CARGO) update

.PHONY: outdated
outdated: ## Check for outdated Rust dependencies
	$(CARGO) outdated

.PHONY: tree
tree: ## Show Rust dependency tree
	$(CARGO) tree

.PHONY: setup
setup: ## Setup development environment
	rustup update
	$(CARGO) install cargo-watch
	$(CARGO) install cargo-outdated
	pip install -e .[dev]

# =============================================================================
# DEVELOPMENT COMMANDS
# =============================================================================

.PHONY: dev
dev: ## Run in development mode (debug build)
	$(CARGO) run

.PHONY: dev-release
dev-release: ## Run in development mode (release build)
	$(CARGO) run --release

.PHONY: watch
watch: ## Watch for changes and rebuild automatically
	$(CARGO) watch -x run

.PHONY: dev-check
dev-check: check test-unit test-python-unit ## Quick development check (check + unit tests)

.PHONY: full-check
full-check: quality test-all build-all ## Full development check (quality + all tests + build) 