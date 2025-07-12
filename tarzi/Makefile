# Makefile for tarzi - Rust-native lite search for AI applications

# Variables
CARGO = cargo
MATURIN = maturin
PYTHON = python3
PYTEST = pytest
RUST_TARGET = target/release/tarzi
PYTHON_PACKAGE = target/wheels/*.whl
PYTHON_TEST_DIR = tests/python
PYTHON_MODULES = examples python

# Integration test control (disabled by default)
ENABLE_INTEGRATION_TESTS ?= false

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
build: build-rust build-python ## Build everything (Rust + Python)

.PHONY: build-rust
build-rust: ## Build Rust binary in release mode
	$(CARGO) build --release

.PHONY: build-debug
build-debug: ## Build Rust binary in debug mode
	$(CARGO) build

.PHONY: build-python
build-python: ## Build Python wheel
	$(MATURIN) build --release

# =============================================================================
# INSTALL COMMANDS
# =============================================================================

.PHONY: install
install: install-rust install-python ## Install everything (Rust + Python)

.PHONY: install-rust
install-rust: build-rust ## Install Rust binary
	cp $(RUST_TARGET) /usr/local/bin/tarzi

.PHONY: install-python
install-python: build-python ## Install Python package
	pip install $(PYTHON_PACKAGE)

.PHONY: install-python-dev
install-python-dev: ## Install Python package in development mode
	$(MATURIN) develop --release

# =============================================================================
# TEST COMMANDS
# =============================================================================

.PHONY: test
test: test-rust test-python ## Run all tests (Rust + Python)

.PHONY: test-rust
test-rust: test-unit-rust ## Run Rust tests (unit + integration if enabled)
ifeq ($(ENABLE_INTEGRATION_TESTS),true)
	@echo "$(GREEN)Running Rust integration tests (enabled)$(RESET)"
	$(CARGO) test --test '*' --features test-helpers
else
	@echo "$(BLUE)Skipping Rust integration tests (disabled by default)$(RESET)"
	@echo "$(BLUE)Set ENABLE_INTEGRATION_TESTS=true to enable them$(RESET)"
endif

.PHONY: test-unit
test-unit: test-unit-rust test-unit-python ## Run all unit tests (Rust + Python)

.PHONY: test-unit-rust
test-unit-rust: ## Run Rust unit tests only
	$(CARGO) test --lib --features test-helpers

.PHONY: test-integration
test-integration: test-integration-rust test-integration-python ## Run all integration tests (Rust + Python)

.PHONY: test-integration-rust
test-integration-rust: ## Run Rust integration tests
	$(CARGO) test --test '*' --features test-helpers

.PHONY: test-python
test-python: test-unit-python ## Run Python tests (unit + integration if enabled)
ifeq ($(ENABLE_INTEGRATION_TESTS),true)
	@echo "$(GREEN)Running Python integration tests (enabled)$(RESET)"
	pip install -e .[test]
	cd $(PYTHON_TEST_DIR) && $(PYTEST) integration/ -m integration
else
	@echo "$(BLUE)Skipping Python integration tests (disabled by default)$(RESET)"
	@echo "$(BLUE)Set ENABLE_INTEGRATION_TESTS=true to enable them$(RESET)"
endif

.PHONY: test-unit-python
test-unit-python: install-python-dev ## Run Python unit tests only
	pip install -e .[test]
	cd $(PYTHON_TEST_DIR) && $(PYTEST) unit/ -m unit

.PHONY: test-integration-python
test-integration-python: install-python-dev ## Run Python integration tests
	pip install -e .[test]
	cd $(PYTHON_TEST_DIR) && $(PYTEST) integration/ -m integration

.PHONY: test-python-coverage
test-python-coverage: install-python-dev ## Run Python tests with coverage
	pip install -e .[test]
	cd $(PYTHON_TEST_DIR) && $(PYTEST) --cov=tarzi --cov-report=html --cov-report=term

# =============================================================================
# INTEGRATION TEST CONTROL COMMANDS
# =============================================================================

.PHONY: test-with-integration
test-with-integration: ## Run all tests including integration tests (Rust + Python)
	$(MAKE) test ENABLE_INTEGRATION_TESTS=true

.PHONY: test-rust-with-integration
test-rust-with-integration: ## Run all Rust tests including integration tests
	$(MAKE) test-rust ENABLE_INTEGRATION_TESTS=true

.PHONY: test-python-with-integration
test-python-with-integration: ## Run all Python tests including integration tests
	$(MAKE) test-python ENABLE_INTEGRATION_TESTS=true

.PHONY: test-ci-mode
test-ci-mode: ## Run tests in CI mode (respects ENABLE_INTEGRATION_TESTS env var)
	@echo "$(BLUE)Running tests in CI mode (ENABLE_INTEGRATION_TESTS=$(ENABLE_INTEGRATION_TESTS))$(RESET)"
	$(MAKE) test

# =============================================================================
# CODE QUALITY COMMANDS
# =============================================================================

.PHONY: format
format:
	$(CARGO) fmt
	@autoflake --in-place --recursive --remove-all-unused-imports --remove-unused-variables $(PYTHON_MODULES)
	@isort $(PYTHON_MODULES)
	@black $(PYTHON_MODULES)

.PHONY: format-check
format-check:
	$(CARGO) fmt -- --check
	@black --check $(PYTHON_MODULES)
	@isort --check-only $(PYTHON_MODULES)

.PHONY: lint
lint: format-check
	$(CARGO) clippy --all-targets --all-features -- -D warnings
	@ruff check $(PYTHON_MODULES)

.PHONY: lint-fix
lint-fix:
	$(CARGO) clippy --fix --allow-dirty --allow-staged --all-targets --all-features -- -D warnings
	@ruff check --fix $(PYTHON_MODULES)

.PHONY: check
check: format-check lint
	$(CARGO) check

.PHONY: autofix
autofix: lint-fix format

# =============================================================================
# CLEAN COMMANDS
# =============================================================================

.PHONY: clean
clean: clean-rust clean-python ## Clean everything including dependencies
	rm -rf target/
	rm -rf .venv/
	rm -rf __pycache__/
	rm -rf *.egg-info/

.PHONY: clean-rust
clean-rust: ## Clean Rust build artifacts
	$(CARGO) clean
	rm -rf target/wheels/

.PHONY: clean-python
clean-python: ## Clean Python test artifacts
	rm -rf $(PYTHON_TEST_DIR)/.pytest_cache
	rm -rf $(PYTHON_TEST_DIR)/htmlcov
	rm -rf $(PYTHON_TEST_DIR)/.coverage
	find $(PYTHON_TEST_DIR) -name "__pycache__" -type d -exec rm -rf {} +
	find $(PYTHON_TEST_DIR) -name "*.pyc" -delete

# =============================================================================
# DOCUMENTATION COMMANDS
# =============================================================================

.PHONY: doc
doc: doc-rust ## Generate and open Rust documentation
	$(CARGO) doc --no-deps --open

.PHONY: doc-rust
doc-rust: ## Generate and open Rust documentation
	$(CARGO) doc --no-deps --open

.PHONY: doc-build
doc-build: doc-build-rust ## Build Rust documentation without opening
	$(CARGO) doc --no-deps

.PHONY: doc-build-rust
doc-build-rust: ## Build Rust documentation without opening
	$(CARGO) doc --no-deps

.PHONY: docs-python
docs-python: ## Build Python documentation with Sphinx
	@cd docs && make html
	@echo "$(GREEN)Documentation built: docs/_build/html/index.html$(RESET)"

.PHONY: docs-python-serve
docs-python-serve: docs-python ## Build and serve Python documentation locally
	@echo "$(GREEN)Serving documentation at http://localhost:8000$(RESET)"
	@cd docs/_build/html && python -m http.server 8000

.PHONY: docs-clean
docs-clean: ## Clean documentation build artifacts
	@rm -rf docs/_build/

# =============================================================================
# RELEASE COMMANDS
# =============================================================================

.PHONY: release
release: release-rust release-python ## Build all release artifacts (Rust + Python)

.PHONY: release-rust
release-rust: ## Build release artifacts (Rust binary)
	$(CARGO) build --release

.PHONY: release-python
release-python: ## Build Python release artifacts
	$(MATURIN) build --release

.PHONY: publish
publish: publish-rust ## Publish Rust crate to crates.io (use with caution!)
	$(CARGO) publish

.PHONY: publish-rust
publish-rust: ## Publish Rust crate to crates.io (use with caution!)
	$(CARGO) publish

.PHONY: publish-python
publish-python: ## Publish Python package to PyPI
	@if [ -z "$(shell ls -A target/wheels/ 2>/dev/null)" ]; then \
		echo "$(RED)❌ No wheels found. Run 'make build-python' first.$(RESET)"; \
		exit 1; \
	fi
	twine check $(PYTHON_PACKAGE)
	twine upload $(PYTHON_PACKAGE)
	@echo "$(GREEN)✅ Package published to PyPI$(RESET)"

.PHONY: publish-python-test
publish-python-test: ## Publish Python package to TestPyPI
	@if [ -z "$(shell ls -A target/wheels/ 2>/dev/null)" ]; then \
		echo "$(RED)❌ No wheels found. Run 'make build-python' first.$(RESET)"; \
		exit 1; \
	fi
	twine check $(PYTHON_PACKAGE)
	twine upload --repository testpypi $(PYTHON_PACKAGE)
	@echo "$(GREEN)✅ Package published to TestPyPI$(RESET)"

.PHONY: check-publish-prereqs
check-publish-prereqs: ## Check prerequisites for publishing
	@command -v twine >/dev/null 2>&1 || (echo "$(RED)❌ twine not found. Install with: pip install twine$(RESET)" && exit 1)
	@python -c "import twine" 2>/dev/null || (echo "$(RED)❌ twine not available in Python. Install with: pip install twine$(RESET)" && exit 1)
	@if [ -z "$${TWINE_USERNAME}" ] && [ -z "$${TWINE_PASSWORD}" ] && [ ! -f ~/.pypirc ]; then \
		echo "$(RED)❌ PyPI credentials not found. Set TWINE_USERNAME/TWINE_PASSWORD or configure ~/.pypirc$(RESET)"; \
		exit 1; \
	fi

.PHONY: build-and-publish-python
build-and-publish-python: check-publish-prereqs build-python publish-python ## Build and publish Python package to PyPI

.PHONY: build-and-publish-python-test
build-and-publish-python-test: check-publish-prereqs build-python publish-python-test ## Build and publish Python package to TestPyPI

# =============================================================================
# UTILITY COMMANDS
# =============================================================================

.PHONY: version
version: ## Show current version
	@echo "$(BLUE)Current version:$(RESET)"
	@echo "Workspace (Cargo.toml): $(shell grep '^version = ' Cargo.toml | cut -d'"' -f2)"
	@echo "Tarzi Rust (tarzi/Cargo.toml): $(shell grep '^version = ' tarzi/Cargo.toml | cut -d'"' -f2)"
	@echo "Python (pyproject.toml): $(shell grep '^version = ' pyproject.toml | cut -d'"' -f2)"
	@echo "Tarzi Python (tarzi/pyproject.toml): $(shell grep '^version = ' tarzi/pyproject.toml | cut -d'"' -f2)"

.PHONY: version-update
version-update:
	@if [ -z "$(VERSION)" ]; then \
		echo "$(RED)❌ VERSION parameter is required. Usage: make version-update VERSION=1.2.3$(RESET)"; \
		exit 1; \
	fi
	@echo "$(BLUE)Updating version to $(VERSION)...$(RESET)"
	@# Update workspace Cargo.toml
	@sed -i.bak 's/^version = ".*"/version = "$(VERSION)"/' Cargo.toml
	@rm -f Cargo.toml.bak
	@echo "$(GREEN)✅ Updated workspace Cargo.toml$(RESET)"
	@# Update tarzi Cargo.toml
	@sed -i.bak 's/^version = ".*"/version = "$(VERSION)"/' tarzi/Cargo.toml
	@rm -f tarzi/Cargo.toml.bak
	@echo "$(GREEN)✅ Updated tarzi/Cargo.toml$(RESET)"
	@# Update pyproject.toml
	@sed -i.bak 's/^version = ".*"/version = "$(VERSION)"/' pyproject.toml
	@rm -f pyproject.toml.bak
	@echo "$(GREEN)✅ Updated pyproject.toml$(RESET)"
	@# Update tarzi pyproject.toml
	@sed -i.bak 's/^version = ".*"/version = "$(VERSION)"/' tarzi/pyproject.toml
	@rm -f tarzi/pyproject.toml.bak
	@echo "$(GREEN)✅ Updated tarzi/pyproject.toml$(RESET)"
	@# Update Cargo.lock
	@$(CARGO) update
	@echo "$(GREEN)✅ Updated Cargo.lock$(RESET)"
	@echo "$(GREEN)✅ Version updated to $(VERSION)$(RESET)"

.PHONY: setup
setup: ## Setup development environment
	rustup update
	$(CARGO) install cargo-outdated
	pip install -e .[dev]
	@echo "$(GREEN)✅ Development environment ready$(RESET)"

.PHONY: setup-docs
setup-docs: ## Setup documentation development environment
	pip install -r docs/requirements.txt
	@echo "$(GREEN)✅ Documentation environment ready$(RESET)"

# =============================================================================
# DEVELOPMENT COMMANDS
# =============================================================================

.PHONY: dev
dev: ## Run in development mode (debug build)
	$(CARGO) run -p tarzi

.PHONY: dev-release
dev-release: ## Run in development mode (release build)
	$(CARGO) run --release -p tarzi

.PHONY: dev-check
dev-check: check test-unit ## Quick development check (check + unit tests)

.PHONY: full-check
full-check: format-check lint test build ## Full development check (all check + all tests + build) 