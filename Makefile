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
	$(CARGO) clippy --all-targets --all-features -- -D warnings

.PHONY: format
format: ## Format Rust code with rustfmt
	$(CARGO) fmt

.PHONY: format-check
format-check: ## Check Rust code formatting
	$(CARGO) fmt -- --check

.PHONY: format-python
format-python: ## Format Python code (autoflake, isort, black)
	@autoflake --in-place --recursive --remove-all-unused-imports --remove-unused-variables $(PYTHON_MODULES)
	@isort $(PYTHON_MODULES)
	@black $(PYTHON_MODULES)

.PHONY: format-python-check
format-python-check: ## Check if Python code is properly formatted
	@black --check $(PYTHON_MODULES) || (echo "$(RED)❌ Black formatting check failed. Run 'make format-python' to fix.$(RESET)" && exit 1)
	@isort --check-only $(PYTHON_MODULES) || (echo "$(RED)❌ Import sorting check failed. Run 'make format-python' to fix.$(RESET)" && exit 1)

.PHONY: format-all
format-all: format format-python ## Format all code (Rust + Python)

.PHONY: format-check-all
format-check-all: format-check format-python-check ## Check all code formatting (Rust + Python)

.PHONY: lint
lint: clippy format-check ## Lint Rust code

.PHONY: lint-python
lint-python: ## Lint Python code with ruff
	@ruff check $(PYTHON_MODULES)

.PHONY: lint-fix
lint-fix: ## Auto-fix Rust linting issues
	$(CARGO) clippy --fix --allow-dirty --allow-staged --all-targets --all-features -- -D warnings

.PHONY: lint-python-fix
lint-python-fix: ## Auto-fix Python linting issues
	@autoflake --in-place --recursive --remove-all-unused-imports --remove-unused-variables $(PYTHON_MODULES)
	@ruff check --fix $(PYTHON_MODULES)

.PHONY: lint-all
lint-all: lint lint-python ## Lint all code (Rust + Python)

.PHONY: autofix
autofix: lint-fix lint-python-fix ## Auto-fix all linting issues

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

.PHONY: update
update: ## Update Rust dependencies
	$(CARGO) update

.PHONY: outdated
outdated: ## Check for outdated Rust dependencies
	$(CARGO) outdated

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
	$(CARGO) run

.PHONY: dev-release
dev-release: ## Run in development mode (release build)
	$(CARGO) run --release

.PHONY: dev-check
dev-check: check test-unit test-python-unit ## Quick development check (check + unit tests)

.PHONY: full-check
full-check: format-check-all lint-all test-all build-all ## Full development check (all check + all tests + build) 