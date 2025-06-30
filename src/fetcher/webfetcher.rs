use crate::{
    Result,
    config::Config,
    constants::{DEFAULT_TIMEOUT, DEFAULT_USER_AGENT, PAGE_LOAD_WAIT},
    converter::{Converter, Format},
    error::TarziError,
};
use reqwest::Client;
use tracing::{debug, error, info, warn};
use url::Url;

use super::{browser::BrowserManager, types::FetchMode};

/// Main web content fetcher
pub struct WebFetcher {
    http_client: Client,
    browser_manager: BrowserManager,
    converter: Converter,
}

impl WebFetcher {
    pub fn new() -> Self {
        info!("Initializing WebFetcher");
        let http_client = Client::builder()
            .timeout(DEFAULT_TIMEOUT)
            .user_agent(DEFAULT_USER_AGENT)
            .build()
            .expect("Failed to create HTTP client");

        info!("HTTP client created successfully for WebFetcher");
        Self {
            http_client,
            browser_manager: BrowserManager::new(),
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
            browser_manager: BrowserManager::from_config(config),
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

        // Get or create browser instance
        info!("Getting or creating browser instance...");
        let browser = self.browser_manager.get_or_create_browser(headless).await?;
        info!("Using existing browser instance for fetching");

        // Navigate to the URL
        info!("Navigating to URL: {}", url);
        let navigation_result = tokio::time::timeout(DEFAULT_TIMEOUT, browser.get(url)).await;

        match navigation_result {
            Ok(Ok(_)) => {
                info!("Successfully navigated to page");
            }
            Ok(Err(e)) => {
                error!("Failed to navigate to URL: {}", e);
                return Err(TarziError::Browser(format!("Failed to navigate: {e}")));
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
        tokio::time::sleep(PAGE_LOAD_WAIT).await;
        info!("Wait completed");

        // Get the page content
        info!("Extracting page content...");
        let content_result = tokio::time::timeout(DEFAULT_TIMEOUT, browser.source()).await;

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
                return Err(TarziError::Browser(format!("Failed to get content: {e}")));
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
                    .timeout(DEFAULT_TIMEOUT)
                    .user_agent(DEFAULT_USER_AGENT)
                    .proxy(reqwest::Proxy::http(proxy)?)
                    .build()
                    .map_err(|e| {
                        TarziError::Config(format!("Failed to create proxy client: {e}"))
                    })?;

                let url = Url::parse(url)?;
                let response = proxy_client.get(url).send().await?;
                let response = response.error_for_status()?;
                response.text().await?
            }
            FetchMode::BrowserHead | FetchMode::BrowserHeadless => {
                // For browser modes with proxy, create a new browser instance with proxy configuration
                info!("Creating browser with proxy for fetching: {}", proxy);
                let headless = matches!(mode, FetchMode::BrowserHeadless);
                let instance_id = self
                    .browser_manager
                    .create_browser_with_proxy(
                        None,
                        headless,
                        Some("proxy_browser".to_string()),
                        Some(proxy.to_string()),
                    )
                    .await?;

                // Get the browser instance and fetch content
                let browser = self
                    .browser_manager
                    .get_browser(&instance_id)
                    .ok_or_else(|| {
                        TarziError::Browser("Failed to get proxy browser instance".to_string())
                    })?;

                // Navigate to URL
                let navigation_result =
                    tokio::time::timeout(DEFAULT_TIMEOUT, browser.get(url)).await;
                match navigation_result {
                    Ok(Ok(_)) => info!("Successfully navigated to page with proxy"),
                    Ok(Err(e)) => {
                        error!("Failed to navigate to URL with proxy: {}", e);
                        return Err(TarziError::Browser(format!(
                            "Failed to navigate with proxy: {e}"
                        )));
                    }
                    Err(_) => {
                        error!("Timeout while navigating to URL with proxy");
                        return Err(TarziError::Browser(
                            "Timeout while navigating with proxy".to_string(),
                        ));
                    }
                }

                // Wait for page load
                tokio::time::sleep(PAGE_LOAD_WAIT).await;

                // Get page content
                let content_result = tokio::time::timeout(DEFAULT_TIMEOUT, browser.source()).await;
                let content = match content_result {
                    Ok(Ok(content)) => {
                        info!(
                            "Successfully extracted page content with proxy ({} characters)",
                            content.len()
                        );
                        content
                    }
                    Ok(Err(e)) => {
                        error!("Failed to get page content with proxy: {}", e);
                        return Err(TarziError::Browser(format!(
                            "Failed to get content with proxy: {e}"
                        )));
                    }
                    Err(_) => {
                        error!("Timeout while extracting page content with proxy");
                        return Err(TarziError::Browser(
                            "Timeout while extracting content with proxy".to_string(),
                        ));
                    }
                };

                // Clean up the proxy browser instance
                if let Err(e) = self.browser_manager.remove_browser(&instance_id).await {
                    warn!("Failed to cleanup proxy browser instance: {}", e);
                }

                content
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

    /// Create a new browser instance with explicit proxy configuration
    pub async fn create_browser_with_proxy(
        &mut self,
        user_data_dir: Option<std::path::PathBuf>,
        headless: bool,
        instance_id: Option<String>,
        proxy: Option<String>,
    ) -> Result<String> {
        self.browser_manager
            .create_browser_with_proxy(user_data_dir, headless, instance_id, proxy)
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
    pub async fn remove_browser(&mut self, instance_id: &str) -> Result<()> {
        self.browser_manager.remove_browser(instance_id).await?;
        Ok(())
    }

    /// Fetch content from a specific browser instance
    pub async fn fetch_with_browser_instance(
        &mut self,
        url: &str,
        instance_id: &str,
        format: Format,
    ) -> Result<String> {
        info!(
            "Fetching URL with browser instance {}: {}",
            instance_id, url
        );

        // Get the browser instance
        let browser = self
            .browser_manager
            .get_browser(instance_id)
            .ok_or_else(|| {
                TarziError::Browser(format!("Browser instance {instance_id} not found"))
            })?;

        info!("Using browser instance {} for fetching", instance_id);

        // Navigate to the URL
        info!(
            "Navigating to URL in browser instance {}: {}",
            instance_id, url
        );
        let navigation_result = tokio::time::timeout(DEFAULT_TIMEOUT, browser.get(url)).await;

        match navigation_result {
            Ok(Ok(_)) => {
                info!(
                    "Successfully navigated to page in browser instance {}",
                    instance_id
                );
            }
            Ok(Err(e)) => {
                error!(
                    "Failed to navigate to URL in browser instance {}: {}",
                    instance_id, e
                );
                return Err(TarziError::Browser(format!("Failed to navigate: {e}")));
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

        // Wait for the page to load (simplified approach)
        info!(
            "Waiting for page to load in browser instance {} (2 seconds)...",
            instance_id
        );
        tokio::time::sleep(PAGE_LOAD_WAIT).await;
        info!("Wait completed for browser instance {}", instance_id);

        // Get the page content
        info!(
            "Extracting page content from browser instance {}...",
            instance_id
        );
        let content_result = tokio::time::timeout(DEFAULT_TIMEOUT, browser.source()).await;

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
                return Err(TarziError::Browser(format!("Failed to get content: {e}")));
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

    /// Clean up managed driver if any
    pub async fn cleanup_managed_driver(&mut self) -> Result<()> {
        self.browser_manager.cleanup_managed_driver().await
    }

    /// Check if this fetcher has a managed driver
    pub fn has_managed_driver(&self) -> bool {
        self.browser_manager.has_managed_driver()
    }

    /// Get information about the managed driver
    pub fn get_managed_driver_info(&self) -> Option<&super::driver::DriverInfo> {
        self.browser_manager.get_managed_driver_info()
    }
}

impl Default for WebFetcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for WebFetcher {
    fn drop(&mut self) {
        if self.browser_manager.has_browsers() {
            warn!("Cleaning up browser resources manually");
        }
        if self.browser_manager.has_managed_driver() {
            warn!("Managed WebDriver will be cleaned up automatically");
        }
    }
}
