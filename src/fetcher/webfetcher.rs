use crate::{
    Result,
    config::Config,
    converter::{Converter, Format},
    error::TarziError,
};
use reqwest::Client;
use std::time::Duration;
use tracing::{debug, error, info, warn};
use url::Url;

use super::{browser::BrowserManager, external::ExternalBrowserManager, types::FetchMode};

/// Main web content fetcher
pub struct WebFetcher {
    http_client: Client,
    browser_manager: BrowserManager,
    external_browser_manager: ExternalBrowserManager,
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
            browser_manager: BrowserManager::new(),
            external_browser_manager: ExternalBrowserManager::new(),
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
            browser_manager: BrowserManager::new(),
            external_browser_manager: ExternalBrowserManager::new(),
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
        let browser = self.browser_manager.get_or_create_browser(headless).await?;
        info!("Browser instance ready");

        info!("Creating new page...");
        let page_result = tokio::time::timeout(Duration::from_secs(30), browser.new_window()).await;

        let window_handle = match page_result {
            Ok(Ok(handle)) => {
                info!("New window created successfully");
                handle
            }
            Ok(Err(e)) => {
                error!("Failed to create window: {}", e);
                return Err(TarziError::Browser(format!(
                    "Failed to create window: {}",
                    e
                )));
            }
            Err(_) => {
                error!("Timeout while creating new window (30 seconds)");
                return Err(TarziError::Browser(
                    "Timeout while creating new window".to_string(),
                ));
            }
        };

        // Switch to the new window
        browser.switch_to_window(window_handle).await.map_err(|e| {
            error!("Failed to switch to window: {}", e);
            TarziError::Browser(format!("Failed to switch to window: {}", e))
        })?;

        // Navigate to the URL
        info!("Navigating to URL: {}", url);
        let navigation_result =
            tokio::time::timeout(Duration::from_secs(30), browser.get(url)).await;

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
        let content_result = tokio::time::timeout(Duration::from_secs(30), browser.source()).await;

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

    /// Fetch content using external browser instance
    async fn fetch_with_external_browser(&mut self, url: &str) -> Result<String> {
        info!("Fetching URL with external browser: {}", url);
        if !self.external_browser_manager.is_connected() {
            warn!(
                "No external browser connection established. Attempting to connect to default endpoint..."
            );
            let default_endpoint = ExternalBrowserManager::get_default_endpoint();
            self.external_browser_manager
                .connect_to_external_browser(&default_endpoint)
                .await?;
        }
        info!("Using external browser instance for fetching");
        let browser = self
            .external_browser_manager
            .get_external_browser()
            .unwrap();
        info!("Creating new window in external browser...");
        let window_result =
            tokio::time::timeout(Duration::from_secs(30), browser.new_window()).await;

        let window_handle = match window_result {
            Ok(Ok(handle)) => {
                info!("New window created successfully in external browser");
                handle
            }
            Ok(Err(e)) => {
                error!("Failed to create window in external browser: {}", e);
                return Err(TarziError::Browser(format!(
                    "Failed to create window: {}",
                    e
                )));
            }
            Err(_) => {
                error!("Timeout while creating new window in external browser (30 seconds)");
                return Err(TarziError::Browser(
                    "Timeout while creating new window".to_string(),
                ));
            }
        };

        // Switch to the new window
        browser.switch_to_window(window_handle).await.map_err(|e| {
            error!("Failed to switch to window in external browser: {}", e);
            TarziError::Browser(format!("Failed to switch to window: {}", e))
        })?;

        // Navigate to the URL
        info!("Navigating to URL in external browser: {}", url);
        let navigation_result =
            tokio::time::timeout(Duration::from_secs(30), browser.get(url)).await;

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
        let content_result = tokio::time::timeout(Duration::from_secs(30), browser.source()).await;

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

    /// Get raw content without conversion (for internal use)
    pub async fn fetch_raw(&mut self, url: &str, mode: FetchMode) -> Result<String> {
        match mode {
            FetchMode::PlainRequest => self.fetch_plain_request(url).await,
            FetchMode::BrowserHead => self.fetch_with_browser(url, false).await,
            FetchMode::BrowserHeadless => self.fetch_with_browser(url, true).await,
            FetchMode::BrowserHeadExternal => self.fetch_with_external_browser(url).await,
        }
    }

    /// Connect to an external browser instance
    pub async fn connect_to_external_browser(&mut self, ws_endpoint: &str) -> Result<()> {
        self.external_browser_manager
            .connect_to_external_browser(ws_endpoint)
            .await
    }

    /// Check prerequisites for external browser connection
    pub async fn check_external_browser_prerequisites(&self, ws_endpoint: &str) -> Result<bool> {
        self.external_browser_manager
            .check_external_browser_prerequisites(ws_endpoint)
            .await
    }

    /// Create a new browser instance with a specific user data directory
    pub async fn create_browser_with_user_data(
        &mut self,
        user_data_dir: Option<std::path::PathBuf>,
        headless: bool,
        instance_id: Option<String>,
    ) -> Result<String> {
        self.browser_manager
            .create_browser_with_user_data(user_data_dir, headless, instance_id)
            .await
    }

    /// Get a browser instance by ID
    pub fn get_browser(&self, instance_id: &str) -> Option<&thirtyfour::WebDriver> {
        self.browser_manager.get_browser(instance_id)
    }

    /// Get all browser instance IDs
    pub fn get_browser_ids(&self) -> Vec<String> {
        self.browser_manager.get_browser_ids()
    }

    /// Remove a browser instance by ID
    pub async fn remove_browser(&mut self, instance_id: &str) -> Result<bool> {
        self.browser_manager.remove_browser(instance_id).await
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

        info!("Creating new window in browser instance: {}", instance_id);
        let window_result =
            tokio::time::timeout(Duration::from_secs(30), browser.new_window()).await;

        let window_handle = match window_result {
            Ok(Ok(handle)) => {
                info!(
                    "New window created successfully in browser instance: {}",
                    instance_id
                );
                handle
            }
            Ok(Err(e)) => {
                error!(
                    "Failed to create window in browser instance {}: {}",
                    instance_id, e
                );
                return Err(TarziError::Browser(format!(
                    "Failed to create window: {}",
                    e
                )));
            }
            Err(_) => {
                error!(
                    "Timeout while creating new window in browser instance {} (30 seconds)",
                    instance_id
                );
                return Err(TarziError::Browser(
                    "Timeout while creating new window".to_string(),
                ));
            }
        };

        // Switch to the new window
        browser.switch_to_window(window_handle).await.map_err(|e| {
            error!(
                "Failed to switch to window in browser instance {}: {}",
                instance_id, e
            );
            TarziError::Browser(format!("Failed to switch to window: {}", e))
        })?;

        // Navigate to the URL
        info!(
            "Navigating to URL in browser instance {}: {}",
            instance_id, url
        );
        let navigation_result =
            tokio::time::timeout(Duration::from_secs(30), browser.get(url)).await;

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
        let content_result = tokio::time::timeout(Duration::from_secs(30), browser.source()).await;

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
        self.browser_manager
            .create_multiple_browsers(count, headless, base_instance_id)
            .await
    }
}

impl Default for WebFetcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for WebFetcher {
    fn drop(&mut self) {
        if self.browser_manager.has_browsers() || self.external_browser_manager.is_connected() {
            warn!("Cleaning up external browser resources manually");
        }
    }
}
