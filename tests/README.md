# Tests

This directory contains integration tests for the tarzi library.

## Test Files

### 1. Fetcher Integration Tests (`fetcher_integration_tests.rs`)
Tests the web fetching functionality including browser automation and HTTP requests.

### 2. Search Parser Integration Tests (`search_parser_integration_tests.rs`)
Tests the search parsing functionality for various search engines.

## Test Configuration

All tests are integration tests that test the library as a whole rather than individual units.

Some tests may require network access or browser installations and may be skipped in CI environments.

## Running Tests

Run all tests:
```bash
cargo test
```

Run specific test file:
```bash
cargo test --test fetcher_integration_tests
cargo test --test search_parser_integration_tests
```

Run tests with output:
```bash
cargo test -- --nocapture
```

Run tests matching a pattern:
```bash
cargo test browser
```

## Test Environment

- Tests use the `tokio::test` runtime for async operations
- Browser tests may require Chrome/Chromium to be installed
- Network-dependent tests may fail in offline environments
- Some tests include timeouts to prevent hanging

## Adding New Tests

When adding new integration tests:

1. Create a new `.rs` file in the `tests/` directory
2. Add the necessary imports and test functions
3. Use `#[tokio::test]` for async tests
4. Include appropriate error handling and assertions
5. Document the test purpose and requirements

Example test structure:
```rust
use tarzi::*;

#[tokio::test]
async fn test_feature() {
    // Test implementation
    assert!(result.is_ok());
}
```

## Search Parser Integration Tests

### `test_bing_parser_real_world_integration`
- **Purpose**: Tests the BingParser with real Bing search results
- **What it does**:
  1. Opens a Chrome browser
  2. Navigates to Bing.com
  3. Performs a search for "rust programming language"
  4. Extracts the HTML source
  5. Uses BingParser to parse the results
  6. Validates the extracted search results
- **Requirements**: WebDriver server running on port 4444
- **Duration**: ~6-10 seconds

### `test_bing_parser_with_different_queries`
- **Purpose**: Tests the parser with multiple different search queries
- **What it does**: Performs searches for different topics and validates results
- **Requirements**: WebDriver server running
- **Duration**: ~15-20 seconds (with delays between searches)

### `test_bing_parser_performance`
- **Purpose**: Measures parsing performance with real HTML
- **What it does**: Times multiple parsing operations on the same HTML
- **Requirements**: WebDriver server running
- **Duration**: ~5-8 seconds

## Test Behavior

### Graceful Degradation
If no WebDriver server is available, the browser-dependent tests will:
- Detect the missing WebDriver
- Skip the test with an informative message
- Not fail the test suite

### Debug Mode
Set `TARZI_DEBUG=1` to enable debug features:
- HTML content is saved to `debug_bing.html` when parsing fails
- Additional debugging output

### Example Output
```
Starting real-world Bing search integration test...
Navigated to Bing homepage
Submitted search query: 'rust programming language'
Search results loaded successfully
Retrieved page source, length: 610434 characters
✓ Successfully retrieved Bing search results HTML
✓ BingParser created and validated
✓ Successfully parsed 2 search results
Result 1: Learn Rust - Rust Programming Language - https://www.rust-lang.org/learn
Result 2: The Rust Programming Language - https://doc.rust-lang.org/stable/book/
✓ All results validated successfully
🎉 Real-world Bing parser integration test completed successfully!
```

## Troubleshooting

### Common Issues

1. **Test skipped: "WebDriver not available"**
   - Solution: Start ChromeDriver or Selenium server on port 4444

2. **Connection refused to localhost:4444**
   - Check if WebDriver server is running
   - Verify the port number
   - Set `TARZI_WEBDRIVER_URL` if using a different endpoint

3. **Browser fails to start**
   - Ensure Chrome is installed
   - Check ChromeDriver version compatibility
   - Try running with `--headless` mode

4. **Search results not found**
   - Bing may be blocking automated requests
   - Try running the test with a delay
   - Check if Bing has changed its HTML structure

5. **Parsing fails**
   - Enable debug mode: `TARZI_DEBUG=1`
   - Check the saved `debug_bing.html` file
   - Bing may have updated their HTML structure

### Performance Notes

- Tests use real network requests and may be affected by:
  - Internet connection speed
  - Bing server response times
  - Geographic location (different Bing regions)
- Average test duration: 5-10 seconds per test
- Parsing performance: typically < 500ms per operation 