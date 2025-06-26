use crate::{error::TarsierError, Result, converter::{Converter, Format}};
use chromiumoxide::{
    browser::{Browser, BrowserConfig, HeadlessMode},
    handler::Handler,
};
use reqwest::Client;
use std::time::Duration;
use url::Url;
use tracing::{info, error, debug, warn};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FetchMode {
    PlainRequest,
    BrowserHead,
    BrowserHeadless,
}

impl std::str::FromStr for FetchMode {
    type Err = TarsierError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "plain_request" | "plain" => Ok(FetchMode::PlainRequest),
            "browser_head" | "head" => Ok(FetchMode::BrowserHead),
            "browser_headless" | "headless" => Ok(FetchMode::BrowserHeadless),
            _ => Err(TarsierError::InvalidMode(s.to_string())),
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
            .user_agent("Mozilla/5.0 (compatible; Tarsier/1.0)")
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

    /// Fetch content from URL and convert to specified format
    pub async fn fetch(&mut self, url: &str, mode: FetchMode, format: Format) -> Result<String> {
        info!("Fetching URL: {} with mode: {:?}, format: {:?}", url, mode, format);
        
        // First fetch the raw content
        let raw_content = match mode {
            FetchMode::PlainRequest => self.fetch_plain_request(url).await?,
            FetchMode::BrowserHead => self.fetch_with_browser(url, false).await?,
            FetchMode::BrowserHeadless => self.fetch_with_browser(url, true).await?,
        };
        
        // Then convert to the specified format
        info!("Converting content to format: {:?}", format);
        let converted_content = self.converter.convert(&raw_content, format).await?;
        
        info!("Successfully fetched and converted content ({} characters)", converted_content.len());
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
        info!("Successfully read response body ({} characters)", content.len());
        
        Ok(content)
    }

    /// Fetch content using browser (with or without headless mode)
    async fn fetch_with_browser(&mut self, url: &str, headless: bool) -> Result<String> {
        info!("Fetching URL with browser (headless: {}): {}", headless, url);
        
        info!("Getting or creating browser instance...");
        let browser = self.get_or_create_browser(headless).await?;
        info!("Browser instance ready");
        
        info!("Creating new page...");
        let page_result = tokio::time::timeout(
            Duration::from_secs(30),
            browser.new_page("about:blank")
        ).await;
        
        let page = match page_result {
            Ok(Ok(page)) => {
                info!("New page created successfully");
                page
            }
            Ok(Err(e)) => {
                error!("Failed to create page: {}", e);
                return Err(TarsierError::Browser(format!("Failed to create page: {}", e)));
            }
            Err(_) => {
                error!("Timeout while creating new page (30 seconds)");
                return Err(TarsierError::Browser("Timeout while creating new page".to_string()));
            }
        };

        // Navigate to the URL
        info!("Navigating to URL: {}", url);
        let navigation_result = tokio::time::timeout(
            Duration::from_secs(30),
            page.goto(url)
        ).await;
        
        match navigation_result {
            Ok(Ok(_)) => {
                info!("Successfully navigated to page");
            }
            Ok(Err(e)) => {
                error!("Failed to navigate to URL: {}", e);
                return Err(TarsierError::Browser(format!("Failed to navigate: {}", e)));
            }
            Err(_) => {
                error!("Timeout while navigating to URL (30 seconds)");
                return Err(TarsierError::Browser("Timeout while navigating to URL".to_string()));
            }
        }

        // Wait for the page to load (simplified approach)
        info!("Waiting for page to load (2 seconds)...");
        tokio::time::sleep(Duration::from_secs(2)).await;
        info!("Wait completed");

        // Get the page content
        info!("Extracting page content...");
        let content_result = tokio::time::timeout(
            Duration::from_secs(30),
            page.content()
        ).await;
        
        let content = match content_result {
            Ok(Ok(content)) => {
                info!("Successfully extracted page content ({} characters)", content.len());
                content
            }
            Ok(Err(e)) => {
                error!("Failed to get page content: {}", e);
                return Err(TarsierError::Browser(format!("Failed to get content: {}", e)));
            }
            Err(_) => {
                error!("Timeout while extracting page content (30 seconds)");
                return Err(TarsierError::Browser("Timeout while extracting page content".to_string()));
            }
        };

        Ok(content)
    }

    async fn get_or_create_browser(&mut self, headless: bool) -> Result<&Browser> {
        if self.browser.is_none() {
            info!("Creating new browser instance for WebFetcher (headless: {})...", headless);
            // For now, always use headless mode since the non-headless variant is not available
            let config = BrowserConfig::builder()
                .headless_mode(HeadlessMode::New)
                .no_sandbox()
                .build()
                .map_err(|e| {
                    error!("Failed to create browser config: {}", e);
                    TarsierError::Browser(format!("Failed to create browser config: {}", e))
                })?;
            info!("Browser config created successfully");

            info!("Launching browser...");
            let browser_result = tokio::time::timeout(
                Duration::from_secs(60), // 60 seconds for browser launch
                Browser::launch(config)
            ).await;
            
            let (browser, handler) = match browser_result {
                Ok(Ok(result)) => {
                    info!("Browser launched successfully");
                    result
                }
                Ok(Err(e)) => {
                    error!("Failed to create browser: {}", e);
                    return Err(TarsierError::Browser(format!("Failed to create browser: {}", e)));
                }
                Err(_) => {
                    error!("Timeout while launching browser (60 seconds)");
                    return Err(TarsierError::Browser("Timeout while launching browser".to_string()));
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
    pub async fn fetch_with_proxy(&mut self, url: &str, proxy: &str, mode: FetchMode, format: Format) -> Result<String> {
        info!("Fetching URL with proxy: {} (proxy: {})", url, proxy);
        
        let raw_content = match mode {
            FetchMode::PlainRequest => {
                let proxy_client = Client::builder()
                    .timeout(Duration::from_secs(30))
                    .user_agent("Mozilla/5.0 (compatible; Tarsier/1.0)")
                    .proxy(reqwest::Proxy::http(proxy)?)
                    .build()
                    .map_err(|e| TarsierError::Config(format!("Failed to create proxy client: {}", e)))?;

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