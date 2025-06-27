# Integration Tests

This directory contains integration tests for the Tarzi library that test real-world functionality.

## Test Categories

### 1. Fetcher Integration Tests (`fetcher_integration_tests.rs`)
Tests the web fetching functionality with real HTTP requests and browser automation.

### 2. External Browser Integration Tests (`external_browser_integration_tests.rs`) 
Tests the external browser functionality for connecting to existing browser instances.

### 3. Search Parser Integration Tests (`search_parser_integration_tests.rs`)
**NEW**: Real-world integration tests for search engine parsers using head browsers.

## Running the Tests

### Prerequisites for Browser Tests

The search parser integration tests require a WebDriver server to be running. You have several options:

#### Option 1: ChromeDriver (Recommended)
```bash
# Install ChromeDriver
brew install chromedriver  # macOS
# or download from https://chromedriver.chromium.org/

# Start ChromeDriver on port 4444
chromedriver --port=4444
```

#### Option 2: Selenium Standalone Server
```bash
# Download selenium-server-standalone
wget https://selenium-release.storage.googleapis.com/4.15/selenium-server-4.15.0.jar

# Start Selenium server
java -jar selenium-server-4.15.0.jar standalone --port 4444
```

#### Option 3: Docker Selenium
```bash
# Start Selenium Chrome container
docker run -d -p 4444:4444 --shm-size=2g selenium/standalone-chrome:latest
```

### Environment Variables

- `TARZI_WEBDRIVER_URL`: WebDriver server URL (default: `http://localhost:4444`)
- `TARZI_DEBUG`: Enable debug mode to save HTML for debugging failed tests

### Running the Tests

```bash
# Run all integration tests
cargo test --test "*_integration_tests"

# Run specific test categories
cargo test --test fetcher_integration_tests
cargo test --test external_browser_integration_tests
cargo test --test search_parser_integration_tests

# Run with output visible
cargo test --test search_parser_integration_tests -- --nocapture

# Run specific test
cargo test test_bing_parser_real_world_integration --test search_parser_integration_tests -- --nocapture
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