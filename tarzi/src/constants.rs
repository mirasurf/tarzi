//! Constants used throughout the Tarzi application
//!
//! This module contains all magic values and constants to avoid duplication
//! and make maintenance easier.

use std::time::Duration;

// ============================================================================
// Network & WebDriver Constants
// ============================================================================

/// Default ChromeDriver URL
pub const CHROMEDRIVER_DEFAULT_URL: &str = "http://localhost:9515";

/// Default ChromeDriver port
pub const CHROMEDRIVER_DEFAULT_PORT: u16 = 9515;

/// Default GeckoDriver port
pub const GECKODRIVER_DEFAULT_PORT: u16 = 4444;

/// Default HTTP client user agent
pub const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36";

/// Default ChromeDriver
pub const CHROMEDRIVER: &str = "chromedriver";

/// Default GeckoDriver
pub const GECKODRIVER: &str = "geckodriver";

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
pub const DEFAULT_SEARCH_ENGINE: &str = "duckduckgo";

/// Default search mode
pub const DEFAULT_SEARCH_MODE: &str = "webquery";

// Default log levels
pub const LOG_LEVEL_INFO: &str = "info";
pub const LOG_LEVEL_DEBUG: &str = "debug";
pub const LOG_LEVEL_WARN: &str = "warn";

// Default formats
pub const FORMAT_MARKDOWN: &str = "markdown";
pub const FORMAT_JSON: &str = "json";
pub const FORMAT_YAML: &str = "yaml";
pub const FORMAT_HTML: &str = "html";

// Default fetcher modes
pub const FETCHER_MODE_BROWSER_HEADLESS: &str = "browser_headless";
pub const FETCHER_MODE_PLAIN_REQUEST: &str = "plain_request";
pub const FETCHER_MODE_HEAD: &str = "head";

// Default search engines
pub const SEARCH_ENGINE_DUCKDUCKGO: &str = "duckduckgo";
pub const SEARCH_ENGINE_BING: &str = "bing";
pub const SEARCH_ENGINE_GOOGLE: &str = "google";
pub const SEARCH_ENGINE_BRAVE: &str = "brave";
pub const SEARCH_ENGINE_BAIDU: &str = "baidu";
pub const SEARCH_ENGINE_EXA: &str = "exa";
pub const SEARCH_ENGINE_TRAVILY: &str = "travily";

// Default search modes
pub const SEARCH_MODE_WEBQUERY: &str = "webquery";
pub const SEARCH_MODE_APIQUERY: &str = "apiquery";

// Default autoswitch strategy
pub const AUTOSWITCH_STRATEGY_SMART: &str = "smart";
pub const AUTOSWITCH_STRATEGY_NONE: &str = "none";

// ============================================================================
// Search Engine Query Patterns
// ============================================================================

/// Web query patterns
pub const DUCKDUCKGO_QUERY_PATTERN: &str = "https://duckduckgo.com/?q={query}";
pub const BING_QUERY_PATTERN: &str = "https://www.bing.com/search?q={query}";
pub const GOOGLE_QUERY_PATTERN: &str = "https://www.google.com/search?q={query}";
pub const BRAVE_QUERY_PATTERN: &str = "https://search.brave.com/search?q={query}";
pub const BAIDU_QUERY_PATTERN: &str = "https://www.baidu.com/s?wd={query}";
pub const EXA_QUERY_PATTERN: &str = "https://exa.ai/search?q={query}";

/// API query patterns
pub const DUCKDUCKGO_API_PATTERN: &str = "https://api.duckduckgo.com/?q={query}&format=json";
pub const BRAVE_API_PATTERN: &str = "https://api.search.brave.com/res/v1/web/search";
pub const BAIDU_API_PATTERN: &str = "https://api.baidu.com/search";
pub const EXA_API_PATTERN: &str = "https://api.exa.ai/search";
pub const TRAVILY_API_PATTERN: &str = "https://api.tavily.com/search";

/// Empty pattern for engines that don't support certain modes
pub const EMPTY_PATTERN: &str = "";

// ============================================================================
// API Key Field Names
// ============================================================================

/// API key field names for configuration
pub const BRAVE_API_KEY_FIELD: &str = "brave_api_key";
pub const BAIDU_API_KEY_FIELD: &str = "baidu_api_key";
pub const EXA_API_KEY_FIELD: &str = "exa_api_key";
pub const TRAVILY_API_KEY_FIELD: &str = "travily_api_key";

// ============================================================================
// Default Values
// ============================================================================

/// Default query pattern (DuckDuckGo)
pub const DEFAULT_QUERY_PATTERN: &str = DUCKDUCKGO_QUERY_PATTERN;
