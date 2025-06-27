//! Constants used throughout the Tarzi application
//!
//! This module contains all magic values and constants to avoid duplication
//! and make maintenance easier.

use std::time::Duration;

// ============================================================================
// Network & WebDriver Constants
// ============================================================================

/// Default WebDriver URL for legacy compatibility (should use CHROMEDRIVER_DEFAULT_URL instead)
pub const WEBDRIVER_LEGACY_DEFAULT_URL: &str = "http://localhost:4444";

/// Default ChromeDriver URL
pub const CHROMEDRIVER_DEFAULT_URL: &str = "http://localhost:9515";

/// Default ChromeDriver port
pub const CHROMEDRIVER_DEFAULT_PORT: u16 = 9515;

/// Default GeckoDriver port
pub const GECKODRIVER_DEFAULT_PORT: u16 = 4444;

/// Default HTTP client user agent
pub const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36";

// ============================================================================
// Timeout Constants
// ============================================================================

/// Default timeout in seconds for various operations
pub const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// Default timeout duration for various operations
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(DEFAULT_TIMEOUT_SECS);

/// WebDriver availability check timeout in seconds
pub const WEBDRIVER_CHECK_TIMEOUT_SECS: u64 = 2;

/// WebDriver availability check timeout duration
pub const WEBDRIVER_CHECK_TIMEOUT: Duration = Duration::from_secs(WEBDRIVER_CHECK_TIMEOUT_SECS);

/// Browser launch timeout in seconds
pub const BROWSER_LAUNCH_TIMEOUT_SECS: u64 = 60;

/// Browser launch timeout duration
pub const BROWSER_LAUNCH_TIMEOUT: Duration = Duration::from_secs(BROWSER_LAUNCH_TIMEOUT_SECS);

/// Page load wait time in seconds
pub const PAGE_LOAD_WAIT_SECS: u64 = 2;

/// Page load wait duration
pub const PAGE_LOAD_WAIT: Duration = Duration::from_secs(PAGE_LOAD_WAIT_SECS);

// ============================================================================
// Test URLs
// ============================================================================

/// HTTPBin HTML test endpoint
pub const HTTPBIN_HTML_URL: &str = "https://httpbin.org/html";

/// HTTPBin JSON test endpoint
pub const HTTPBIN_JSON_URL: &str = "https://httpbin.org/json";

/// HTTPBin XML test endpoint
pub const HTTPBIN_XML_URL: &str = "https://httpbin.org/xml";

/// HTTPBin 404 status test endpoint
pub const HTTPBIN_404_URL: &str = "https://httpbin.org/status/404";

/// HTTPBin 500 status test endpoint
pub const HTTPBIN_500_URL: &str = "https://httpbin.org/status/500";

/// HTTPBin large response test endpoint (10KB)
pub const HTTPBIN_LARGE_URL: &str = "https://httpbin.org/bytes/10000";

/// Example domain for testing
pub const EXAMPLE_URL: &str = "https://example.com";

/// Example proxy URL for testing
pub const EXAMPLE_PROXY_URL: &str = "http://example.com:8080";

// ============================================================================
// Browser Arguments
// ============================================================================

/// Chrome/Chromium browser arguments for headless mode
pub const CHROME_HEADLESS_ARGS: &[&str] = &[
    "--headless",
    "--disable-gpu",
    "--disable-dev-shm-usage",
    "--no-sandbox",
];

/// Chrome/Chromium browser arguments for driver
pub const CHROME_DRIVER_ARGS: &[&str] =
    &["--disable-gpu", "--no-sandbox", "--disable-dev-shm-usage"];

/// Firefox browser arguments
pub const FIREFOX_DRIVER_ARGS: &[&str] = &["--log=warn"];

// ============================================================================
// Default Configuration Values
// ============================================================================

/// Default log level
pub const DEFAULT_LOG_LEVEL: &str = "info";

/// Default search limit
pub const DEFAULT_SEARCH_LIMIT: usize = 5;

/// Default fetcher mode string
pub const DEFAULT_FETCHER_MODE: &str = "browser_headless";

/// Default converter format string
pub const DEFAULT_FORMAT: &str = "markdown";

/// Default search engine
pub const DEFAULT_SEARCH_ENGINE: &str = "bing";

/// Default search mode
pub const DEFAULT_SEARCH_MODE: &str = "webquery";
