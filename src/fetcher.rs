use crate::{error::TarsierError, Result};
use chromiumoxide::{
    browser::{Browser, BrowserConfig, HeadlessMode},
    handler::Handler,
};
use reqwest::Client;
use std::time::Duration;
use url::Url;

pub struct WebFetcher {
    http_client: Client,
    browser: Option<Browser>,
    _handler: Option<Handler>,
}

impl WebFetcher {
    pub fn new() -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("Mozilla/5.0 (compatible; Tarsier/1.0)")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            http_client,
            browser: None,
            _handler: None,
        }
    }

    pub async fn fetch(&self, url: &str) -> Result<String> {
        let url = Url::parse(url)?;
        let response = self.http_client.get(url).send().await?;
        
        let response = response.error_for_status()?;
        let content = response.text().await?;
        Ok(content)
    }

    pub async fn fetch_with_js(&mut self, url: &str) -> Result<String> {
        let browser = self.get_or_create_browser().await?;
        let page = browser.new_page("about:blank").await
            .map_err(|e| TarsierError::Browser(format!("Failed to create page: {}", e)))?;

        // Navigate to the URL
        page.goto(url).await
            .map_err(|e| TarsierError::Browser(format!("Failed to navigate: {}", e)))?;

        // Wait for the page to load (simplified approach)
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Get the page content
        let content = page.content().await
            .map_err(|e| TarsierError::Browser(format!("Failed to get content: {}", e)))?;

        Ok(content)
    }

    async fn get_or_create_browser(&mut self) -> Result<&Browser> {
        if self.browser.is_none() {
            let config = BrowserConfig::builder()
                .headless_mode(HeadlessMode::New)
                .no_sandbox()
                .build()
                .map_err(|e| TarsierError::Browser(format!("Failed to create browser config: {}", e)))?;

            let (browser, handler) = Browser::launch(config).await
                .map_err(|e| TarsierError::Browser(format!("Failed to create browser: {}", e)))?;

            self.browser = Some(browser);
            self._handler = Some(handler);
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