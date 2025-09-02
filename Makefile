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

# =============================================================================
# TEST COMMANDS
# =============================================================================

.PHONY: test
test: test-unit test-integration ## Run all tests (unit + integration)

.PHONY: test-unit
test-unit: test-unit-rust test-unit-python ## Run all unit tests (Rust + Python)

.PHONY: test-unit-rust
test-unit-rust: ## Run Rust unit tests only
	$(CARGO) test --lib --features test-helpers

.PHONY: test-unit-python
test-unit-python: ## Run Python unit tests only
	python3 -m venv .venv
	.venv/bin/pip install -e .[test] pytest pytest-mock pytest-asyncio
	cd $(PYTHON_TEST_DIR) && $(PWD)/.venv/bin/python -m pytest -m unit -v

.PHONY: test-integration
test-integration: test-integration-rust test-integration-python ## Run all integration tests (Rust + Python)

.PHONY: test-integration-rust
test-integration-rust: ## Run Rust integration tests
	$(CARGO) test --test '*' --features test-helpers

.PHONY: test-integration-python
test-integration-python: ## Run Python integration tests only
	python3 -m venv .venv
	.venv/bin/pip install -e .[test] pytest pytest-mock pytest-asyncio
	cd $(PYTHON_TEST_DIR) && $(PWD)/.venv/bin/python -m pytest -m integration -v

# =============================================================================
# CODE QUALITY COMMANDS
# =============================================================================

.PHONY: format
format: format-rust format-python ## Format code (Rust + Python)

.PHONY: format-rust
format-rust: ## Format Rust code
	$(CARGO) fmt

.PHONY: format-python
format-python: ## Format Python code (optional tools)
	@command -v autoflake >/dev/null 2>&1 && autoflake --in-place --recursive --remove-all-unused-imports --remove-unused-variables $(PYTHON_MODULES) || echo "autoflake not found, skipping Python auto-cleanup"
	@command -v isort >/dev/null 2>&1 && isort $(PYTHON_MODULES) || echo "isort not found, skipping Python import sorting"
	@command -v black >/dev/null 2>&1 && black $(PYTHON_MODULES) || echo "black not found, skipping Python formatting"

.PHONY: format-check
format-check: format-check-rust format-check-python ## Check code formatting (Rust + Python)

.PHONY: format-check-rust
format-check-rust: ## Check Rust code formatting
	$(CARGO) fmt -- --check

.PHONY: format-check-python
format-check-python: ## Check Python code formatting (optional tools)
	@command -v black >/dev/null 2>&1 && black --check $(PYTHON_MODULES) || echo "black not found, skipping Python format check"
	@command -v isort >/dev/null 2>&1 && isort --check-only $(PYTHON_MODULES) || echo "isort not found, skipping Python import check"

.PHONY: lint
lint: format-check lint-rust lint-python ## Lint code (Rust + Python)

.PHONY: lint-rust
lint-rust: ## Lint Rust code
	$(CARGO) clippy --all-targets --all-features -- -D warnings

.PHONY: lint-python
lint-python: ## Lint Python code (optional tools)
	@command -v ruff >/dev/null 2>&1 && ruff check $(PYTHON_MODULES) || echo "ruff not found, skipping Python linting"

.PHONY: lint-fix
lint-fix: lint-fix-rust lint-fix-python ## Fix linting issues (Rust + Python)

.PHONY: lint-fix-rust
lint-fix-rust: ## Fix Rust linting issues
	$(CARGO) clippy --fix --allow-dirty --allow-staged --all-targets --all-features -- -D warnings

.PHONY: lint-fix-python
lint-fix-python: ## Fix Python linting issues (optional tools)
	@command -v ruff >/dev/null 2>&1 && ruff check --fix $(PYTHON_MODULES) || echo "ruff not found, skipping Python lint fixes"

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

.PHONY: doc-build
doc-build:
	$(CARGO) doc --no-deps
	@cd docs && make html
	@echo "$(GREEN)Documentation built: docs/_build/html/index.html$(RESET)"

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
	@echo "Python (pyproject.toml): $(shell grep '^version = ' pyproject.toml | cut -d'"' -f2)"

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
	@# Update pyproject.toml
	@sed -i.bak 's/^version = ".*"/version = "$(VERSION)"/' pyproject.toml
	@rm -f pyproject.toml.bak
	@echo "$(GREEN)✅ Updated pyproject.toml$(RESET)"
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
