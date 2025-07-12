# Python Tests for Tarzi

This directory contains the Python test suite for the tarzi library, organized using pytest.

## Structure

```
tests/python/
├── conftest.py              # Shared pytest fixtures and configuration
├── pytest.ini              # Pytest configuration
├── unit/                    # Unit tests (no external dependencies)
│   ├── test_converter.py    # Tests for Converter class and convert_html function
│   ├── test_config.py       # Tests for Config class
│   └── test_html_pipeline.py # Tests for HTML processing pipeline
├── integration/             # Integration tests (require external services)
│   ├── test_web_fetcher.py  # Tests for WebFetcher class and fetch_url function
│   └── test_search_engine.py # Tests for SearchEngine class and search functions
└── README.md               # This file
```

## Test Types

### Unit Tests
- Located in `unit/` directory
- No external dependencies (network, APIs, files)
- Fast execution
- Test core functionality and error handling
- Automatically marked with `@pytest.mark.unit`

### Integration Tests
- Located in `integration/` directory
- Require external services (network, APIs)
- May be slow or unreliable
- Test end-to-end functionality
- Automatically marked with `@pytest.mark.integration`

## Running Tests

### Run All Tests
```bash
cd tests/python
pytest
```

### Run Only Unit Tests
```bash
pytest --unit-only
# or
pytest -m unit
```

### Run Only Integration Tests
```bash
pytest --integration-only
# or
pytest -m integration
```

### Skip Network Tests (Offline Mode)
```bash
pytest --offline
```

### Run Specific Test Files
```bash
pytest unit/test_converter.py
pytest integration/test_web_fetcher.py
```

### Run with Verbose Output
```bash
pytest -v
```

### Run with Coverage
```bash
pytest --cov=tarzi --cov-report=html
```

## Test Markers

The test suite uses the following pytest markers:

- `unit`: Unit tests that don't require external dependencies
- `integration`: Integration tests that require external services
- `network`: Tests that require network access
- `slow`: Tests that may take a long time to run
- `api`: Tests that require API keys or external API access
- `browser`: Tests that require browser automation tools

### Using Markers
```bash
# Run only network tests
pytest -m network

# Run all tests except slow ones
pytest -m "not slow"

# Run unit tests but skip API tests
pytest -m "unit and not api"
```

## Configuration

The test suite is configured via:

- `pytest.ini`: Basic pytest configuration and marker definitions
- `conftest.py`: Shared fixtures and custom pytest hooks
- Command line options for test selection

## Fixtures

Common fixtures available to all tests:

- `default_config`: Default tarzi Config instance
- `sample_config_str`: Sample TOML configuration string
- `simple_html`: Simple HTML content for testing
- `complex_html`: Complex HTML document for testing

## Environment Variables

Some integration tests may use environment variables:

- `TARZI_API_KEY`: Search engine API key for API tests
- `TARZI_PROXY`: Proxy configuration for proxy tests

## Continuous Integration

For CI environments, use:

```bash
# Fast CI run (unit tests only)
pytest --unit-only

# Full CI run with timeouts
pytest --timeout=300

# CI with coverage
pytest --unit-only --cov=tarzi --cov-report=xml
```

## Development

When adding new tests:

1. Place unit tests in `unit/` directory
2. Place integration tests in `integration/` directory
3. Use appropriate pytest markers
4. Add fixtures to `conftest.py` for shared test data
5. Follow the naming convention `test_*.py` for files and `test_*` for functions
6. Use descriptive test names and docstrings

## Troubleshooting

### Network Test Failures
Integration tests may fail due to network issues or rate limiting. Use `--offline` to skip network-dependent tests during development.

### Slow Test Performance
Use `--unit-only` for rapid development feedback, or `pytest -m "not slow"` to skip time-consuming tests.

### Import Errors
Ensure tarzi is properly installed:
```bash
pip install -e .  # Install in development mode
``` 