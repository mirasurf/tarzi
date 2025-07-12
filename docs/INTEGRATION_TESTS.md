# Integration Test Controls

This document explains how to control integration test execution in the Tarzi project.

## Overview

Integration tests are **disabled by default** in both Rust and Python test suites to ensure fast CI/CD pipelines and avoid external dependencies in regular development workflow.

## Environment Variable

Use the `ENABLE_INTEGRATION_TESTS` environment variable to control integration test execution:

- `ENABLE_INTEGRATION_TESTS=false` (default): Integration tests are skipped
- `ENABLE_INTEGRATION_TESTS=true`: Integration tests are executed

## Usage

### Command Line

#### Rust Tests

```bash
# Run only unit tests (default behavior)
make test-rust

# Run unit + integration tests
make test-rust-with-integration

# Or using environment variable
ENABLE_INTEGRATION_TESTS=true make test-rust
```

#### Python Tests

```bash
# Run only unit tests (default behavior)
make test-python

# Run unit + integration tests
make test-python-with-integration

# Or using environment variable
ENABLE_INTEGRATION_TESTS=true make test-python

# Or using pytest directly with command-line option
cd tests/python && pytest --enable-integration
```

#### All Tests

```bash
# Run only unit tests for both Rust and Python (default)
make test

# Run all tests including integration tests
make test-with-integration

# Or using environment variable
ENABLE_INTEGRATION_TESTS=true make test
```

### CI Mode

```bash
# Run tests in CI mode (respects ENABLE_INTEGRATION_TESTS env var)
make test-ci-mode
```

### GitHub Actions

Integration tests are controlled via GitHub repository variables:

1. Go to your repository settings
2. Navigate to "Secrets and variables" > "Actions"
3. In the "Variables" tab, add:
   - **Name**: `ENABLE_INTEGRATION_TESTS`
   - **Value**: `true` (to enable) or `false` (to disable)

If the variable is not set, integration tests default to **disabled**.

## Test Categories

### Unit Tests
- **Location**: `tests/unit/` (Python), `src/lib.rs` and `src/*/mod.rs` (Rust)
- **Characteristics**: Fast, no external dependencies, always enabled
- **Runtime**: < 1 second per test

### Integration Tests
- **Location**: `tests/integration/` (Python), `tests/*.rs` (Rust)
- **Characteristics**: May require external services, network access, or browser automation
- **Runtime**: Variable (1-60 seconds per test with timeout protection)
- **Dependencies**: WebDriver, network access, external APIs

## Why Integration Tests Are Disabled by Default

1. **Speed**: Integration tests are significantly slower than unit tests
2. **Dependencies**: They may require external services or network access
3. **Reliability**: They can be flaky due to external factors
4. **CI Resources**: They consume more CI/CD resources and time
5. **Development Workflow**: Most development tasks only require unit tests

## When to Enable Integration Tests

Enable integration tests when:

- ✅ Testing major changes to web scraping functionality
- ✅ Validating browser automation features
- ✅ Testing API integration components
- ✅ Before releasing new versions
- ✅ Investigating integration-specific bugs

## Timeout Protection

All integration tests have timeout protection to prevent hanging:

- **Rust**: 60-second timeout using `tokio::time::timeout`
- **Python**: Automatic skip when `ENABLE_INTEGRATION_TESTS=false`
- **CI**: Tests fail gracefully if they exceed timeout limits

## Examples

### Local Development

```bash
# Quick development workflow (unit tests only)
make test

# Before committing major changes
make test-with-integration

# Testing specific components
ENABLE_INTEGRATION_TESTS=true cargo test test_browser_automation
ENABLE_INTEGRATION_TESTS=true pytest tests/python/integration/test_web_scraping.py --enable-integration
```

### CI Pipeline

```yaml
# In GitHub Actions workflow
env:
  ENABLE_INTEGRATION_TESTS: ${{ vars.ENABLE_INTEGRATION_TESTS || 'false' }}
```

### Makefile Integration

```bash
# Check current setting
make test-ci-mode

# Force enable for this run
make test-with-integration
```

## Best Practices

1. **Default to unit tests** for regular development
2. **Enable integration tests** for major releases
3. **Use CI variables** to control integration tests in different environments
4. **Document test requirements** in individual test files
5. **Keep integration tests focused** and avoid unnecessary external dependencies
6. **Use timeout protection** to prevent hanging tests

## Troubleshooting

### Integration Tests Not Running

Check that:
- `ENABLE_INTEGRATION_TESTS=true` is set
- Required dependencies are installed (WebDriver, etc.)
- Network access is available
- API keys are configured (if needed)

### Integration Tests Hanging

- All tests have 60-second timeout protection
- Use `timeout` command as additional safety: `timeout 300 make test-with-integration`
- Check WebDriver installation and configuration

### Python Integration Tests Skipped

```bash
# Check if integration tests are enabled
python -c "import os; print('ENABLE_INTEGRATION_TESTS:', os.environ.get('ENABLE_INTEGRATION_TESTS', 'false'))"

# Enable explicitly
pytest --enable-integration tests/python/integration/
```

This system ensures that integration tests are available when needed while maintaining fast and reliable default test execution.