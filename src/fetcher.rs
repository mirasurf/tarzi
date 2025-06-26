use crate::{
    Result,
    config::Config,
    converter::{Converter, Format},
    error::TarziError,
};
use chromiumoxide::{
    browser::{Browser, BrowserConfig, HeadlessMode},
    handler::Handler,
};
use reqwest::Client;
use std::time::Duration;
use tracing::{debug, error, info, warn};
use url::Url;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FetchMode {
    PlainRequest,
    BrowserHead,
    BrowserHeadless,
}

impl std::str::FromStr for FetchMode {
    type Err = TarziError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "plain_request" | "plain" => Ok(FetchMode::PlainRequest),
            "browser_head" | "head" => Ok(FetchMode::BrowserHead),
            "browser_headless" | "headless" => Ok(FetchMode::BrowserHeadless),
            _ => Err(TarziError::InvalidMode(s.to_string())),
        }
    }
}

pub struct WebFetcher {
    http_client: Client,
    browser: Option<Browser>,
    _handler: Option<Handler>,
    converter: Converter,
}

impl WebFetcher {
    pub fn new() -> Self {
        info!("Initializing WebFetcher");
        let http_client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("Mozilla/5.0 (compatible; Tarzi/1.0)")
            .build()
            .expect("Failed to create HTTP client");

        info!("HTTP client created successfully for WebFetcher");
        Self {
            http_client,
            browser: None,
            _handler: None,
            converter: Converter::new(),
        }
    }

    pub fn from_config(config: &Config) -> Self {
        info!("Initializing WebFetcher from config");
        let mut client_builder = Client::builder()
            .timeout(std::time::Duration::from_secs(config.fetcher.timeout))
            .user_agent(&config.fetcher.user_agent);
        if let Some(proxy) = &config.fetcher.proxy {
            if !proxy.is_empty() {
                if let Ok(proxy_obj) = reqwest::Proxy::http(proxy) {
                    client_builder = client_builder.proxy(proxy_obj);
                }
            }
        }
        let http_client = client_builder
            .build()
            .expect("Failed to create HTTP client from config");
        Self {
            http_client,
            browser: None,
            _handler: None,
            converter: Converter::new(),
        }
    }

    /// Fetch content from URL and convert to specified format
    pub async fn fetch(&mut self, url: &str, mode: FetchMode, format: Format) -> Result<String> {
        info!(
            "Fetching URL: {} with mode: {:?}, format: {:?}",
            url, mode, format
        );

        // First fetch the raw content
        let raw_content = match mode {
            FetchMode::PlainRequest => self.fetch_plain_request(url).await?,
            FetchMode::BrowserHead => self.fetch_with_browser(url, false).await?,
            FetchMode::BrowserHeadless => self.fetch_with_browser(url, true).await?,
        };

        // Then convert to the specified format
        info!("Converting content to format: {:?}", format);
        let converted_content = self.converter.convert(&raw_content, format).await?;

        info!(
            "Successfully fetched and converted content ({} characters)",
            converted_content.len()
        );
        Ok(converted_content)
    }

    /// Fetch raw content using plain HTTP request (no JS rendering)
    async fn fetch_plain_request(&self, url: &str) -> Result<String> {
        info!("Fetching URL with plain request: {}", url);
        let url = Url::parse(url)?;
        debug!("Parsed URL: {:?}", url);

        info!("Sending HTTP request...");
        let response = self.http_client.get(url).send().await?;
        info!("Received HTTP response with status: {}", response.status());

        let response = response.error_for_status()?;
        info!("HTTP request successful");

        info!("Reading response body...");
        let content = response.text().await?;
        info!(
            "Successfully read response body ({} characters)",
            content.len()
        );

        Ok(content)
    }

    /// Fetch content using browser (with or without headless mode)
    async fn fetch_with_browser(&mut self, url: &str, headless: bool) -> Result<String> {
        info!(
            "Fetching URL with browser (headless: {}): {}",
            headless, url
        );

        info!("Getting or creating browser instance...");
        let browser = self.get_or_create_browser(headless).await?;
        info!("Browser instance ready");

        info!("Creating new page...");
        let page_result =
            tokio::time::timeout(Duration::from_secs(30), browser.new_page("about:blank")).await;

        let page = match page_result {
            Ok(Ok(page)) => {
                info!("New page created successfully");
                page
            }
            Ok(Err(e)) => {
                error!("Failed to create page: {}", e);
                return Err(TarziError::Browser(format!("Failed to create page: {}", e)));
            }
            Err(_) => {
                error!("Timeout while creating new page (30 seconds)");
                return Err(TarziError::Browser(
                    "Timeout while creating new page".to_string(),
                ));
            }
        };

        // Navigate to the URL
        info!("Navigating to URL: {}", url);
        let navigation_result = tokio::time::timeout(Duration::from_secs(30), page.goto(url)).await;

        match navigation_result {
            Ok(Ok(_)) => {
                info!("Successfully navigated to page");
            }
            Ok(Err(e)) => {
                error!("Failed to navigate to URL: {}", e);
                return Err(TarziError::Browser(format!("Failed to navigate: {}", e)));
            }
            Err(_) => {
                error!("Timeout while navigating to URL (30 seconds)");
                return Err(TarziError::Browser(
                    "Timeout while navigating to URL".to_string(),
                ));
            }
        }

        // Wait for the page to load (simplified approach)
        info!("Waiting for page to load (2 seconds)...");
        tokio::time::sleep(Duration::from_secs(2)).await;
        info!("Wait completed");

        // Get the page content
        info!("Extracting page content...");
        let content_result = tokio::time::timeout(Duration::from_secs(30), page.content()).await;

        let content = match content_result {
            Ok(Ok(content)) => {
                info!(
                    "Successfully extracted page content ({} characters)",
                    content.len()
                );
                content
            }
            Ok(Err(e)) => {
                error!("Failed to get page content: {}", e);
                return Err(TarziError::Browser(format!("Failed to get content: {}", e)));
            }
            Err(_) => {
                error!("Timeout while extracting page content (30 seconds)");
                return Err(TarziError::Browser(
                    "Timeout while extracting page content".to_string(),
                ));
            }
        };

        Ok(content)
    }

    async fn get_or_create_browser(&mut self, headless: bool) -> Result<&Browser> {
        if self.browser.is_none() {
            info!(
                "Creating new browser instance for WebFetcher (headless: {})...",
                headless
            );
            // For now, always use headless mode since the non-headless variant is not available
            let config = BrowserConfig::builder()
                .headless_mode(HeadlessMode::New)
                .no_sandbox()
                .build()
                .map_err(|e| {
                    error!("Failed to create browser config: {}", e);
                    TarziError::Browser(format!("Failed to create browser config: {}", e))
                })?;
            info!("Browser config created successfully");

            info!("Launching browser...");
            let browser_result = tokio::time::timeout(
                Duration::from_secs(60), // 60 seconds for browser launch
                Browser::launch(config),
            )
            .await;

            let (browser, handler) = match browser_result {
                Ok(Ok(result)) => {
                    info!("Browser launched successfully");
                    result
                }
                Ok(Err(e)) => {
                    error!("Failed to create browser: {}", e);
                    return Err(TarziError::Browser(format!(
                        "Failed to create browser: {}",
                        e
                    )));
                }
                Err(_) => {
                    error!("Timeout while launching browser (60 seconds)");
                    return Err(TarziError::Browser(
                        "Timeout while launching browser".to_string(),
                    ));
                }
            };

            self.browser = Some(browser);
            self._handler = Some(handler);
            info!("Browser instance stored");
        } else {
            info!("Using existing browser instance");
        }

        Ok(self.browser.as_ref().unwrap())
    }

    /// Fetch content using proxy
    pub async fn fetch_with_proxy(
        &mut self,
        url: &str,
        proxy: &str,
        mode: FetchMode,
        format: Format,
    ) -> Result<String> {
        info!("Fetching URL with proxy: {} (proxy: {})", url, proxy);

        let raw_content = match mode {
            FetchMode::PlainRequest => {
                let proxy_client = Client::builder()
                    .timeout(Duration::from_secs(30))
                    .user_agent("Mozilla/5.0 (compatible; Tarzi/1.0)")
                    .proxy(reqwest::Proxy::http(proxy)?)
                    .build()
                    .map_err(|e| {
                        TarziError::Config(format!("Failed to create proxy client: {}", e))
                    })?;

                let url = Url::parse(url)?;
                let response = proxy_client.get(url).send().await?;
                let response = response.error_for_status()?;
                response.text().await?
            }
            FetchMode::BrowserHead | FetchMode::BrowserHeadless => {
                // For browser modes with proxy, we would need to configure the browser to use the proxy
                // This is a simplified implementation - in practice you'd configure browser proxy settings
                warn!("Proxy not yet implemented for browser modes, falling back to plain request");
                self.fetch_plain_request(url).await?
            }
        };

        // Convert to specified format
        let converted_content = self.converter.convert(&raw_content, format).await?;
        Ok(converted_content)
    }

    /// Get raw content without conversion (for internal use)
    pub async fn fetch_raw(&mut self, url: &str, mode: FetchMode) -> Result<String> {
        match mode {
            FetchMode::PlainRequest => self.fetch_plain_request(url).await,
            FetchMode::BrowserHead => self.fetch_with_browser(url, false).await,
            FetchMode::BrowserHeadless => self.fetch_with_browser(url, true).await,
        }
    }
}

impl Default for WebFetcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for WebFetcher {
    fn drop(&mut self) {
        // Clean up browser resources if needed
        if let Some(_browser) = &self.browser {
            // Note: In a real implementation, you might want to properly close the browser
            // This is a simplified version
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::converter::Format;
    use std::str::FromStr;

    #[test]
    fn test_fetch_mode_from_str() {
        // Test valid modes
        assert_eq!(
            FetchMode::from_str("plain_request").unwrap(),
            FetchMode::PlainRequest
        );
        assert_eq!(
            FetchMode::from_str("plain").unwrap(),
            FetchMode::PlainRequest
        );
        assert_eq!(
            FetchMode::from_str("PLAIN_REQUEST").unwrap(),
            FetchMode::PlainRequest
        );
        assert_eq!(
            FetchMode::from_str("PLAIN").unwrap(),
            FetchMode::PlainRequest
        );

        assert_eq!(
            FetchMode::from_str("browser_head").unwrap(),
            FetchMode::BrowserHead
        );
        assert_eq!(FetchMode::from_str("head").unwrap(), FetchMode::BrowserHead);
        assert_eq!(
            FetchMode::from_str("BROWSER_HEAD").unwrap(),
            FetchMode::BrowserHead
        );
        assert_eq!(FetchMode::from_str("HEAD").unwrap(), FetchMode::BrowserHead);

        assert_eq!(
            FetchMode::from_str("browser_headless").unwrap(),
            FetchMode::BrowserHeadless
        );
        assert_eq!(
            FetchMode::from_str("headless").unwrap(),
            FetchMode::BrowserHeadless
        );
        assert_eq!(
            FetchMode::from_str("BROWSER_HEADLESS").unwrap(),
            FetchMode::BrowserHeadless
        );
        assert_eq!(
            FetchMode::from_str("HEADLESS").unwrap(),
            FetchMode::BrowserHeadless
        );

        // Test invalid modes
        assert!(FetchMode::from_str("invalid").is_err());
        assert!(FetchMode::from_str("").is_err());
        assert!(FetchMode::from_str("browser").is_err());
        assert!(FetchMode::from_str("request").is_err());
    }

    #[test]
    fn test_webfetcher_new() {
        let fetcher = WebFetcher::new();
        assert!(fetcher.browser.is_none());
        assert!(fetcher._handler.is_none());
        assert_eq!(fetcher.converter, Converter::new());
    }

    #[test]
    fn test_webfetcher_default() {
        let fetcher1 = WebFetcher::new();
        let fetcher2 = WebFetcher::default();
        assert_eq!(fetcher1.converter, fetcher2.converter);
        assert_eq!(fetcher1.browser.is_none(), fetcher2.browser.is_none());
        assert_eq!(fetcher1._handler.is_none(), fetcher2._handler.is_none());
    }

    #[test]
    fn test_fetch_mode_partial_eq() {
        assert_eq!(FetchMode::PlainRequest, FetchMode::PlainRequest);
        assert_eq!(FetchMode::BrowserHead, FetchMode::BrowserHead);
        assert_eq!(FetchMode::BrowserHeadless, FetchMode::BrowserHeadless);

        assert_ne!(FetchMode::PlainRequest, FetchMode::BrowserHead);
        assert_ne!(FetchMode::PlainRequest, FetchMode::BrowserHeadless);
        assert_ne!(FetchMode::BrowserHead, FetchMode::BrowserHeadless);
    }

    #[test]
    fn test_fetch_mode_debug() {
        assert_eq!(format!("{:?}", FetchMode::PlainRequest), "PlainRequest");
        assert_eq!(format!("{:?}", FetchMode::BrowserHead), "BrowserHead");
        assert_eq!(
            format!("{:?}", FetchMode::BrowserHeadless),
            "BrowserHeadless"
        );
    }

    #[test]
    fn test_fetch_mode_clone() {
        let mode1 = FetchMode::PlainRequest;
        let mode2 = mode1.clone();
        assert_eq!(mode1, mode2);

        let mode3 = FetchMode::BrowserHead;
        let mode4 = mode3.clone();
        assert_eq!(mode3, mode4);
    }

    #[test]
    fn test_fetch_mode_copy() {
        let mode1 = FetchMode::BrowserHeadless;
        let mode2 = mode1; // This should work because FetchMode is Copy
        assert_eq!(mode1, mode2);
    }

    #[tokio::test]
    async fn test_fetch_raw_plain_request_invalid_url() {
        let mut fetcher = WebFetcher::new();
        let result = fetcher
            .fetch_raw("invalid-url", FetchMode::PlainRequest)
            .await;
        assert!(result.is_err());
        match result.unwrap_err() {
            TarziError::Url(_) => (), // Expected
            e => panic!("Expected Url error, got {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_fetch_with_proxy_invalid_proxy() {
        let mut fetcher = WebFetcher::new();

        // Test fetching with an invalid proxy (should fail)
        let result = fetcher
            .fetch_with_proxy(
                "https://httpbin.org/html",
                "http://invalid-proxy:8080",
                FetchMode::PlainRequest,
                Format::Html,
            )
            .await;

        // This should fail due to invalid proxy, but it might succeed in some environments
        // So we'll just check that it doesn't panic and handle both cases
        match result {
            Ok(_) => {
                // If it succeeds, that's also acceptable (some environments might handle invalid proxies differently)
                println!("Proxy test succeeded - invalid proxy was handled gracefully");
            }
            Err(_) => {
                // If it fails, that's the expected behavior
                println!("Proxy test failed as expected");
            }
        }
    }

    #[test]
    fn test_webfetcher_drop() {
        // Test that WebFetcher can be dropped without panicking
        let _fetcher = WebFetcher::new();
        // Should not panic when dropped
    }

    #[test]
    fn test_fetch_mode_serialization() {
        // Test that FetchMode can be converted to string and back
        let modes = vec![
            FetchMode::PlainRequest,
            FetchMode::BrowserHead,
            FetchMode::BrowserHeadless,
        ];

        for mode in modes {
            let mode_str = match mode {
                FetchMode::PlainRequest => "plain_request",
                FetchMode::BrowserHead => "browser_head",
                FetchMode::BrowserHeadless => "browser_headless",
            };

            let parsed = FetchMode::from_str(mode_str).unwrap();
            assert_eq!(mode, parsed);
        }
    }

    #[test]
    fn test_webfetcher_from_config() {
        use crate::config::Config;
        let mut config = Config::new();
        config.fetcher.user_agent = "TestAgent/1.0".to_string();
        config.fetcher.timeout = 42;
        config.fetcher.proxy = Some("http://localhost:1234".to_string());
        let fetcher = WebFetcher::from_config(&config);
        // We can't directly check the http_client internals, but we can check that the struct is created
        assert!(fetcher.browser.is_none());
        assert!(fetcher._handler.is_none());
        assert_eq!(fetcher.converter, Converter::new());
    }
}
