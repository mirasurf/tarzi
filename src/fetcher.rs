use crate::{error::TarsierError, Result};
use chromiumoxide::{
    browser::{Browser, BrowserConfig, HeadlessMode},
    handler::Handler,
};
use reqwest::Client;
use std::time::Duration;
use url::Url;
use tracing::{info, error, debug};

pub struct WebFetcher {
    http_client: Client,
    browser: Option<Browser>,
    _handler: Option<Handler>,
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
        }
    }

    pub async fn fetch(&self, url: &str) -> Result<String> {
        info!("Fetching URL: {}", url);
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

    pub async fn fetch_with_js(&mut self, url: &str) -> Result<String> {
        info!("Fetching URL with JavaScript rendering: {}", url);
        
        info!("Getting or creating browser instance...");
        let browser = self.get_or_create_browser().await?;
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

    async fn get_or_create_browser(&mut self) -> Result<&Browser> {
        if self.browser.is_none() {
            info!("Creating new browser instance for WebFetcher...");
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

    pub async fn fetch_with_proxy(&self, url: &str, proxy: &str) -> Result<String> {
        let proxy_client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("Mozilla/5.0 (compatible; Tarsier/1.0)")
            .proxy(reqwest::Proxy::http(proxy)?)
            .build()
            .map_err(|e| TarsierError::Config(format!("Failed to create proxy client: {}", e)))?;

        let url = Url::parse(url)?;
        let response = proxy_client.get(url).send().await?;
        
        let response = response.error_for_status()?;
        let content = response.text().await?;
        Ok(content)
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