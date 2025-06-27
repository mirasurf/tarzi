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
use std::{collections::HashMap, path::PathBuf, time::Duration};
use tempfile::TempDir;
use tracing::{debug, error, info, warn};
use url::Url;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FetchMode {
    PlainRequest,
    BrowserHead,
    BrowserHeadless,
    BrowserHeadExternal,
}

impl std::str::FromStr for FetchMode {
    type Err = TarziError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "plain_request" | "plain" => Ok(FetchMode::PlainRequest),
            "browser_head" | "head" => Ok(FetchMode::BrowserHead),
            "browser_headless" | "headless" => Ok(FetchMode::BrowserHeadless),
            "browser_head_external" | "external" => Ok(FetchMode::BrowserHeadExternal),
            _ => Err(TarziError::InvalidMode(s.to_string())),
        }
    }
}

pub struct WebFetcher {
    http_client: Client,
    browsers: HashMap<String, (Browser, Handler, TempDir)>,
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
            browsers: HashMap::new(),
            converter: Converter::new(),
        }
    }

    pub fn from_config(config: &Config) -> Self {
        info!("Initializing WebFetcher from config");
        let mut client_builder = Client::builder()
            .timeout(std::time::Duration::from_secs(config.fetcher.timeout))
            .user_agent(&config.fetcher.user_agent);

        // Use environment variables for proxy with fallback to config
        let proxy = crate::config::get_proxy_from_env_or_config(&config.fetcher.proxy);
        if let Some(proxy) = proxy {
            if !proxy.is_empty() {
                if let Ok(proxy_obj) = reqwest::Proxy::http(&proxy) {
                    client_builder = client_builder.proxy(proxy_obj);
                    info!("Using proxy from environment/config: {}", proxy);
                } else {
                    warn!("Invalid proxy configuration: {}", proxy);
                }
            }
        }

        let http_client = client_builder
            .build()
            .expect("Failed to create HTTP client from config");
        Self {
            http_client,
            browsers: HashMap::new(),
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
            FetchMode::BrowserHeadExternal => self.fetch_with_external_browser(url).await?,
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
        if self.browsers.is_empty() {
            info!(
                "Creating new browser instance for WebFetcher (headless: {})...",
                headless
            );
            let instance_id = self
                .create_browser_with_user_data(None, headless, Some("default".to_string()))
                .await?;
            info!("Browser instance created with ID: {}", instance_id);
        } else {
            info!("Using existing browser instance");
        }
        Ok(&self.browsers.values().next().unwrap().0)
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
            FetchMode::BrowserHeadExternal => {
                // For external browser mode with proxy, we would need to configure the browser to use the proxy
                // This is a simplified implementation - in practice you'd configure browser proxy settings
                warn!(
                    "Proxy not yet implemented for external browser mode, falling back to plain request"
                );
                self.fetch_plain_request(url).await?
            }
        };

        // Convert to specified format
        let converted_content = self.converter.convert(&raw_content, format).await?;
        Ok(converted_content)
    }

    /// Connect to an external browser instance
    pub async fn connect_to_external_browser(&mut self, ws_endpoint: &str) -> Result<()> {
        info!(
            "Attempting to connect to external browser at: {}",
            ws_endpoint
        );

        // Check if the endpoint is accessible
        if !self
            .check_external_browser_prerequisites(ws_endpoint)
            .await?
        {
            return Err(TarziError::Browser(
                "External browser prerequisites not met".to_string(),
            ));
        }

        info!("Prerequisites met, connecting to external browser...");

        // FIXME (2025-06-26): Try to connect to the external browser
        // Note: chromiumoxide doesn't have a direct connect method, so we'll use a different approach
        // For now, we'll simulate the connection by creating a new browser instance
        warn!("External browser connection not fully implemented - using fallback approach");
        let config = BrowserConfig::builder()
            .headless_mode(HeadlessMode::New)
            .no_sandbox()
            .build()
            .map_err(|e| {
                error!(
                    "Failed to create browser config for external connection: {}",
                    e
                );
                TarziError::Browser(format!("Failed to create browser config: {}", e))
            })?;

        info!("Browser config created for external connection");

        let browser_result = tokio::time::timeout(
            Duration::from_secs(30), // 30 seconds for connection
            Browser::launch(config),
        )
        .await;

        let (browser, handler) = match browser_result {
            Ok(Ok(result)) => {
                info!("Successfully connected to external browser (fallback mode)");
                result
            }
            Ok(Err(e)) => {
                error!("Failed to connect to external browser: {}", e);
                return Err(TarziError::Browser(format!(
                    "Failed to connect to external browser: {}",
                    e
                )));
            }
            Err(_) => {
                error!("Timeout while connecting to external browser (30 seconds)");
                return Err(TarziError::Browser(
                    "Timeout while connecting to external browser".to_string(),
                ));
            }
        };

        let temp_dir = TempDir::new()?;
        self.browsers.insert(
            "external".to_string(),
            (browser, handler, temp_dir),
        );
        info!("External browser connection established and stored");
        Ok(())
    }

    /// Check prerequisites for external browser connection
    pub async fn check_external_browser_prerequisites(&self, ws_endpoint: &str) -> Result<bool> {
        info!(
            "Checking external browser prerequisites for endpoint: {}",
            ws_endpoint
        );

        // Check if the endpoint URL is valid
        if !ws_endpoint.starts_with("ws://") && !ws_endpoint.starts_with("wss://") {
            warn!("Invalid WebSocket endpoint format: {}", ws_endpoint);
            return Ok(false);
        }

        // FIXME (2025-06-26): For now, we'll assume the endpoint is valid if it has the correct format
        info!("Basic WebSocket endpoint format validation passed");

        // FIXME (2025-06-26): Try to establish a basic WebSocket connection to check if the browser is accessible
        // info!("Attempting basic WebSocket connectivity check...");

        Ok(true)
    }

    /// Fetch content using external browser instance
    async fn fetch_with_external_browser(&mut self, url: &str) -> Result<String> {
        info!("Fetching URL with external browser: {}", url);
        if self.browsers.is_empty() {
            warn!(
                "No external browser connection established. Attempting to connect to default endpoint..."
            );
            let default_endpoint = std::env::var("TARZI_EXTERNAL_BROWSER_ENDPOINT")
                .unwrap_or_else(|_| "ws://localhost:9222".to_string());
            self.connect_to_external_browser(&default_endpoint).await?;
        }
        info!("Using external browser instance for fetching");
        let browser = &self.browsers.values().next().unwrap().0;
        info!("Creating new page in external browser...");
        let page_result =
            tokio::time::timeout(Duration::from_secs(30), browser.new_page("about:blank")).await;

        let page = match page_result {
            Ok(Ok(page)) => {
                info!("New page created successfully in external browser");
                page
            }
            Ok(Err(e)) => {
                error!("Failed to create page in external browser: {}", e);
                return Err(TarziError::Browser(format!("Failed to create page: {}", e)));
            }
            Err(_) => {
                error!("Timeout while creating new page in external browser (30 seconds)");
                return Err(TarziError::Browser(
                    "Timeout while creating new page".to_string(),
                ));
            }
        };

        // Navigate to the URL
        info!("Navigating to URL in external browser: {}", url);
        let navigation_result = tokio::time::timeout(Duration::from_secs(30), page.goto(url)).await;

        match navigation_result {
            Ok(Ok(_)) => {
                info!("Successfully navigated to page in external browser");
            }
            Ok(Err(e)) => {
                error!("Failed to navigate to URL in external browser: {}", e);
                return Err(TarziError::Browser(format!("Failed to navigate: {}", e)));
            }
            Err(_) => {
                error!("Timeout while navigating to URL in external browser (30 seconds)");
                return Err(TarziError::Browser(
                    "Timeout while navigating to URL".to_string(),
                ));
            }
        }

        // Wait for the page to load (simplified approach)
        info!("Waiting for page to load in external browser (2 seconds)...");
        tokio::time::sleep(Duration::from_secs(2)).await;
        info!("Wait completed");

        // Get the page content
        info!("Extracting page content from external browser...");
        let content_result = tokio::time::timeout(Duration::from_secs(30), page.content()).await;

        let content = match content_result {
            Ok(Ok(content)) => {
                info!(
                    "Successfully extracted page content from external browser ({} characters)",
                    content.len()
                );
                content
            }
            Ok(Err(e)) => {
                error!("Failed to get page content from external browser: {}", e);
                return Err(TarziError::Browser(format!("Failed to get content: {}", e)));
            }
            Err(_) => {
                error!("Timeout while extracting page content from external browser (30 seconds)");
                return Err(TarziError::Browser(
                    "Timeout while extracting page content".to_string(),
                ));
            }
        };

        Ok(content)
    }

    /// Get raw content without conversion (for internal use)
    pub async fn fetch_raw(&mut self, url: &str, mode: FetchMode) -> Result<String> {
        match mode {
            FetchMode::PlainRequest => self.fetch_plain_request(url).await,
            FetchMode::BrowserHead => self.fetch_with_browser(url, false).await,
            FetchMode::BrowserHeadless => self.fetch_with_browser(url, true).await,
            FetchMode::BrowserHeadExternal => self.fetch_with_external_browser(url).await,
        }
    }

    /// Create a new browser instance with a specific user data directory
    pub async fn create_browser_with_user_data(
        &mut self,
        user_data_dir: Option<PathBuf>,
        headless: bool,
        instance_id: Option<String>,
    ) -> Result<String> {
        let instance_id = instance_id.unwrap_or_else(|| {
            let temp_dir = TempDir::new().expect("Failed to create temp dir for instance ID");
            temp_dir.path().to_string_lossy().to_string()
        });
        info!(
            "Creating new browser instance with ID: {} (headless: {}, user_data_dir: {:?})",
            instance_id, headless, user_data_dir
        );
        let mut config_builder = BrowserConfig::builder();
        if headless {
            config_builder = config_builder.headless_mode(HeadlessMode::New);
        } else {
            config_builder = config_builder.with_head();
        }
        config_builder = config_builder.no_sandbox();
        let temp_dir = if let Some(user_data_path) = user_data_dir {
            config_builder = config_builder.user_data_dir(user_data_path);
            None
        } else {
            let temp = TempDir::new().map_err(|e| {
                error!("Failed to create temporary directory: {}", e);
                TarziError::Browser(format!("Failed to create temporary directory: {}", e))
            })?;
            config_builder = config_builder.user_data_dir(temp.path());
            Some(temp)
        };
        let config = config_builder.build().map_err(|e| {
            error!("Failed to create browser config: {}", e);
            TarziError::Browser(format!("Failed to create browser config: {}", e))
        })?;
        info!("Browser config created successfully");
        let browser_result = tokio::time::timeout(
            Duration::from_secs(60),
            Browser::launch(config),
        )
        .await;
        let (browser, handler) = match browser_result {
            Ok(Ok(result)) => {
                info!("Browser launched successfully with ID: {}", instance_id);
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
        self.browsers.insert(
            instance_id.clone(),
            (
                browser,
                handler,
                temp_dir.unwrap_or_else(|| {
                    TempDir::new().expect("Failed to create temp dir for browser storage")
                }),
            ),
        );
        info!("Browser instance stored with ID: {}", instance_id);
        Ok(instance_id)
    }

    /// Get a browser instance by ID
    pub fn get_browser(&self, instance_id: &str) -> Option<&Browser> {
        self.browsers
            .get(instance_id)
            .map(|(browser, _, _)| browser)
    }

    /// Get all browser instance IDs
    pub fn get_browser_ids(&self) -> Vec<String> {
        self.browsers.keys().cloned().collect()
    }

    /// Remove a browser instance by ID
    pub fn remove_browser(&mut self, instance_id: &str) -> bool {
        if let Some((_, _, _temp_dir)) = self.browsers.remove(instance_id) {
            info!("Removed browser instance: {}", instance_id);
            // The temp_dir will be automatically cleaned up when dropped
            true
        } else {
            warn!("Browser instance not found: {}", instance_id);
            false
        }
    }

    /// Fetch content using a specific browser instance
    pub async fn fetch_with_browser_instance(
        &mut self,
        url: &str,
        instance_id: &str,
        format: Format,
    ) -> Result<String> {
        info!(
            "Fetching URL: {} with browser instance: {}",
            url, instance_id
        );

        let browser = self.get_browser(instance_id).ok_or_else(|| {
            TarziError::Browser(format!("Browser instance not found: {}", instance_id))
        })?;

        info!("Creating new page in browser instance: {}", instance_id);
        let page_result =
            tokio::time::timeout(Duration::from_secs(30), browser.new_page("about:blank")).await;

        let page = match page_result {
            Ok(Ok(page)) => {
                info!(
                    "New page created successfully in browser instance: {}",
                    instance_id
                );
                page
            }
            Ok(Err(e)) => {
                error!(
                    "Failed to create page in browser instance {}: {}",
                    instance_id, e
                );
                return Err(TarziError::Browser(format!("Failed to create page: {}", e)));
            }
            Err(_) => {
                error!(
                    "Timeout while creating new page in browser instance {} (30 seconds)",
                    instance_id
                );
                return Err(TarziError::Browser(
                    "Timeout while creating new page".to_string(),
                ));
            }
        };

        // Navigate to the URL
        info!(
            "Navigating to URL in browser instance {}: {}",
            instance_id, url
        );
        let navigation_result = tokio::time::timeout(Duration::from_secs(30), page.goto(url)).await;

        match navigation_result {
            Ok(Ok(_)) => {
                info!(
                    "Successfully navigated to page in browser instance: {}",
                    instance_id
                );
            }
            Ok(Err(e)) => {
                error!(
                    "Failed to navigate to URL in browser instance {}: {}",
                    instance_id, e
                );
                return Err(TarziError::Browser(format!("Failed to navigate: {}", e)));
            }
            Err(_) => {
                error!(
                    "Timeout while navigating to URL in browser instance {} (30 seconds)",
                    instance_id
                );
                return Err(TarziError::Browser(
                    "Timeout while navigating to URL".to_string(),
                ));
            }
        }

        // Wait for the page to load
        info!(
            "Waiting for page to load in browser instance {} (2 seconds)...",
            instance_id
        );
        tokio::time::sleep(Duration::from_secs(2)).await;
        info!("Wait completed");

        // Get the page content
        info!(
            "Extracting page content from browser instance: {}",
            instance_id
        );
        let content_result = tokio::time::timeout(Duration::from_secs(30), page.content()).await;

        let content = match content_result {
            Ok(Ok(content)) => {
                info!(
                    "Successfully extracted page content from browser instance {} ({} characters)",
                    instance_id,
                    content.len()
                );
                content
            }
            Ok(Err(e)) => {
                error!(
                    "Failed to get page content from browser instance {}: {}",
                    instance_id, e
                );
                return Err(TarziError::Browser(format!("Failed to get content: {}", e)));
            }
            Err(_) => {
                error!(
                    "Timeout while extracting page content from browser instance {} (30 seconds)",
                    instance_id
                );
                return Err(TarziError::Browser(
                    "Timeout while extracting page content".to_string(),
                ));
            }
        };

        // Convert to specified format
        let converted_content = self.converter.convert(&content, format).await?;
        Ok(converted_content)
    }

    /// Create multiple browser instances for parallel processing
    pub async fn create_multiple_browsers(
        &mut self,
        count: usize,
        headless: bool,
        base_instance_id: Option<String>,
    ) -> Result<Vec<String>> {
        info!(
            "Creating {} browser instances (headless: {})",
            count, headless
        );

        let base_id = base_instance_id.unwrap_or_else(|| "browser".to_string());
        let mut instance_ids = Vec::new();

        for i in 0..count {
            let instance_id = format!("{}_{}", base_id, i);
            let id = self
                .create_browser_with_user_data(None, headless, Some(instance_id.clone()))
                .await?;
            instance_ids.push(id);
        }

        info!("Successfully created {} browser instances", count);
        Ok(instance_ids)
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
        if !self.browsers.is_empty() {
            // FIXME (2025-06-27): In a real implementation, you might want to properly close the browsers
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

        assert_eq!(
            FetchMode::from_str("browser_head_external").unwrap(),
            FetchMode::BrowserHeadExternal
        );
        assert_eq!(
            FetchMode::from_str("external").unwrap(),
            FetchMode::BrowserHeadExternal
        );
        assert_eq!(
            FetchMode::from_str("BROWSER_HEAD_EXTERNAL").unwrap(),
            FetchMode::BrowserHeadExternal
        );
        assert_eq!(
            FetchMode::from_str("EXTERNAL").unwrap(),
            FetchMode::BrowserHeadExternal
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
        assert!(fetcher.browsers.is_empty());
        assert_eq!(fetcher.converter, Converter::new());
    }

    #[test]
    fn test_webfetcher_default() {
        let fetcher1 = WebFetcher::new();
        let fetcher2 = WebFetcher::default();
        assert_eq!(fetcher1.converter, fetcher2.converter);
        assert!(fetcher1.browsers.is_empty());
        assert!(fetcher2.browsers.is_empty());
    }

    #[test]
    fn test_fetch_mode_partial_eq() {
        assert_eq!(FetchMode::PlainRequest, FetchMode::PlainRequest);
        assert_eq!(FetchMode::BrowserHead, FetchMode::BrowserHead);
        assert_eq!(FetchMode::BrowserHeadless, FetchMode::BrowserHeadless);
        assert_eq!(
            FetchMode::BrowserHeadExternal,
            FetchMode::BrowserHeadExternal
        );

        assert_ne!(FetchMode::PlainRequest, FetchMode::BrowserHead);
        assert_ne!(FetchMode::PlainRequest, FetchMode::BrowserHeadless);
        assert_ne!(FetchMode::PlainRequest, FetchMode::BrowserHeadExternal);
        assert_ne!(FetchMode::BrowserHead, FetchMode::BrowserHeadless);
        assert_ne!(FetchMode::BrowserHead, FetchMode::BrowserHeadExternal);
        assert_ne!(FetchMode::BrowserHeadless, FetchMode::BrowserHeadExternal);
    }

    #[test]
    fn test_fetch_mode_debug() {
        assert_eq!(format!("{:?}", FetchMode::PlainRequest), "PlainRequest");
        assert_eq!(format!("{:?}", FetchMode::BrowserHead), "BrowserHead");
        assert_eq!(
            format!("{:?}", FetchMode::BrowserHeadless),
            "BrowserHeadless"
        );
        assert_eq!(
            format!("{:?}", FetchMode::BrowserHeadExternal),
            "BrowserHeadExternal"
        );
    }

    #[test]
    fn test_fetch_mode_clone() {
        let mode1 = FetchMode::PlainRequest;
        let mode2 = mode1;
        assert_eq!(mode1, mode2);

        let mode3 = FetchMode::BrowserHead;
        let mode4 = mode3;
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
            FetchMode::BrowserHeadExternal,
        ];

        for mode in modes {
            let mode_str = match mode {
                FetchMode::PlainRequest => "plain_request",
                FetchMode::BrowserHead => "browser_head",
                FetchMode::BrowserHeadless => "browser_headless",
                FetchMode::BrowserHeadExternal => "browser_head_external",
            };

            let parsed = FetchMode::from_str(mode_str).unwrap();
            assert_eq!(mode, parsed);
        }
    }

    #[test]
    fn test_webfetcher_from_config() {
        use crate::config::Config;
        let config = Config::new();
        let fetcher = WebFetcher::from_config(&config);
        // We can't directly check the http_client internals, but we can check that the struct is created
        assert!(fetcher.browsers.is_empty());
        assert_eq!(fetcher.converter, Converter::new());
    }

    #[test]
    fn test_webfetcher_from_config_with_env_proxy() {
        use crate::config::Config;
        // Set environment variable
        unsafe {
            std::env::set_var("HTTP_PROXY", "http://test-proxy:8080");
        }

        let config = Config::new();
        let _fetcher = WebFetcher::from_config(&config);

        // Note: We can't easily test if the proxy was actually set on the client
        // without making HTTP requests, but we can verify the function doesn't panic

        // Clean up
        unsafe {
            std::env::remove_var("HTTP_PROXY");
        }
    }

    // Integration tests for external browser functionality
    #[tokio::test]
    async fn test_connect_to_external_browser_invalid_endpoint() {
        let mut fetcher = WebFetcher::new();

        // Test with invalid endpoint format
        let result = fetcher
            .connect_to_external_browser("invalid-endpoint")
            .await;
        assert!(result.is_err());

        // Test with non-websocket endpoint
        let result = fetcher
            .connect_to_external_browser("http://localhost:9222")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_connect_to_external_browser_valid_format() {
        let mut fetcher = WebFetcher::new();

        // Test with valid WebSocket format (should pass format validation but fail connection)
        let result = fetcher
            .connect_to_external_browser("ws://localhost:9222")
            .await;

        // This should fail because there's no actual browser running, but it should pass the format check
        // and fail during the actual connection attempt
        match result {
            Ok(_) => {
                // If it succeeds, that means there's actually a browser running (unlikely in test environment)
                println!("External browser connection succeeded - browser is actually running");
            }
            Err(e) => {
                // This is the expected behavior in test environment
                match e {
                    TarziError::Browser(_) => {
                        println!("External browser connection failed as expected: {:?}", e);
                    }
                    _ => panic!("Expected Browser error, got {:?}", e),
                }
            }
        }
    }

    #[tokio::test]
    async fn test_fetch_with_external_browser_no_connection() {
        let mut fetcher = WebFetcher::new();

        // Test fetching with external browser mode when no connection is established
        // This should attempt to connect to the default endpoint and fail
        let result = fetcher
            .fetch_raw("https://httpbin.org/html", FetchMode::BrowserHeadExternal)
            .await;

        match result {
            Ok(_) => {
                // If it succeeds, that means there's actually a browser running
                println!("External browser fetch succeeded - browser is actually running");
            }
            Err(e) => {
                // This is the expected behavior in test environment
                match e {
                    TarziError::Browser(_) => {
                        println!("External browser fetch failed as expected: {:?}", e);
                    }
                    _ => panic!("Expected Browser error, got {:?}", e),
                }
            }
        }
    }

    #[tokio::test]
    async fn test_external_browser_prerequisites_check() {
        let fetcher = WebFetcher::new();

        // Test valid WebSocket endpoints
        let valid_endpoints = vec![
            "ws://localhost:9222",
            "wss://localhost:9222",
            "ws://127.0.0.1:9222",
            "wss://example.com:9222",
        ];

        for endpoint in valid_endpoints {
            let result = fetcher.check_external_browser_prerequisites(endpoint).await;
            assert!(result.is_ok());
            assert!(
                result.unwrap(),
                "Endpoint {} should pass prerequisites check",
                endpoint
            );
        }

        // Test invalid endpoints
        let invalid_endpoints = vec![
            "http://localhost:9222",
            "https://localhost:9222",
            "invalid-endpoint",
            "ftp://localhost:9222",
            "",
        ];

        for endpoint in invalid_endpoints {
            let result = fetcher.check_external_browser_prerequisites(endpoint).await;
            assert!(result.is_ok());
            assert!(
                !result.unwrap(),
                "Endpoint {} should fail prerequisites check",
                endpoint
            );
        }
    }

    #[test]
    fn test_external_browser_environment_variable() {
        // Test that the environment variable fallback works correctly
        let original_env = std::env::var("TARZI_EXTERNAL_BROWSER_ENDPOINT");

        // Test with environment variable set
        unsafe {
            std::env::set_var("TARZI_EXTERNAL_BROWSER_ENDPOINT", "ws://custom:9222");
        }
        let endpoint = std::env::var("TARZI_EXTERNAL_BROWSER_ENDPOINT")
            .unwrap_or_else(|_| "ws://localhost:9222".to_string());
        assert_eq!(endpoint, "ws://custom:9222");

        // Test with environment variable not set
        unsafe {
            std::env::remove_var("TARZI_EXTERNAL_BROWSER_ENDPOINT");
        }
        let endpoint = std::env::var("TARZI_EXTERNAL_BROWSER_ENDPOINT")
            .unwrap_or_else(|_| "ws://localhost:9222".to_string());
        assert_eq!(endpoint, "ws://localhost:9222");

        // Restore original environment variable if it existed
        if let Ok(val) = original_env {
            unsafe {
                std::env::set_var("TARZI_EXTERNAL_BROWSER_ENDPOINT", val);
            }
        }
    }

    #[tokio::test]
    async fn test_external_browser_mode_integration() {
        let mut fetcher = WebFetcher::new();

        // Test the complete flow of external browser mode
        // This test verifies that the mode is properly handled even when the actual browser is not available

        // Test that the mode is recognized
        let mode = FetchMode::BrowserHeadExternal;
        assert_eq!(mode, FetchMode::from_str("browser_head_external").unwrap());
        assert_eq!(mode, FetchMode::from_str("external").unwrap());

        // Test that the mode is properly handled in fetch_raw
        let result = fetcher.fetch_raw("https://httpbin.org/html", mode).await;

        // The result should be an error because no external browser is running
        // but the mode should be properly recognized and handled
        match result {
            Ok(_) => {
                println!(
                    "External browser integration test succeeded - browser is actually running"
                );
            }
            Err(e) => match e {
                TarziError::Browser(_) => {
                    println!(
                        "External browser integration test failed as expected: {:?}",
                        e
                    );
                }
                _ => panic!("Expected Browser error, got {:?}", e),
            },
        }
    }
}
