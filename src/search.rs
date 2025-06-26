use crate::{error::TarsierError, Result};
use chromiumoxide::{
    browser::{Browser, BrowserConfig, HeadlessMode},
    handler::Handler,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SearchMode {
    Browser,
    Api,
}

impl FromStr for SearchMode {
    type Err = TarsierError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "browser" => Ok(SearchMode::Browser),
            "api" => Ok(SearchMode::Api),
            _ => Err(TarsierError::InvalidMode(s.to_string())),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
    pub rank: usize,
}

pub struct SearchEngine {
    http_client: Client,
    browser: Option<Browser>,
    _handler: Option<Handler>,
    api_key: Option<String>,
}

impl SearchEngine {
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
            api_key: None,
        }
    }

    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    pub async fn search(&mut self, query: &str, mode: SearchMode, limit: usize) -> Result<Vec<SearchResult>> {
        match mode {
            SearchMode::Browser => self.search_browser(query, limit).await,
            SearchMode::Api => self.search_api(query, limit).await,
        }
    }

    async fn search_browser(&mut self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let browser = self.get_or_create_browser().await?;
        let page = browser.new_page("about:blank").await
            .map_err(|e| TarsierError::Browser(format!("Failed to create page: {}", e)))?;

        // Navigate to Google search
        let search_url = format!("https://www.google.com/search?q={}", urlencoding::encode(query));
        page.goto(&search_url).await
            .map_err(|e| TarsierError::Browser(format!("Failed to navigate: {}", e)))?;

        // Wait for search results to load (simplified approach)
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Extract search results using JavaScript
        let results_js = format!(r#"
            Array.from(document.querySelectorAll('.g')).slice(0, {}).map((result, index) => {{
                const titleElement = result.querySelector('h3');
                const linkElement = result.querySelector('a');
                const snippetElement = result.querySelector('.VwiC3b');
                
                return {{
                    title: titleElement ? titleElement.textContent : '',
                    url: linkElement ? linkElement.href : '',
                    snippet: snippetElement ? snippetElement.textContent : '',
                    rank: index + 1
                }};
            }})
        "#, limit);

        let results: Vec<SearchResult> = page.evaluate(results_js).await
            .map_err(|e| TarsierError::Browser(format!("Failed to evaluate JavaScript: {}", e)))?
            .into_value()
            .map_err(|e| TarsierError::Browser(format!("Failed to parse results: {}", e)))?;

        Ok(results)
    }

    async fn search_api(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        // This is a simplified API search implementation
        // In a real implementation, you would use actual search APIs like Google Custom Search, Bing, etc.
        
        if self.api_key.is_none() {
            return Err(TarsierError::Config("API key required for API mode".to_string()));
        }

        // For demonstration, we'll simulate API search results
        // In practice, you would make actual API calls here
        let mock_results = vec![
            SearchResult {
                title: format!("Search result for: {}", query),
                url: "https://example.com/result1".to_string(),
                snippet: format!("This is a mock search result for the query: {}", query),
                rank: 1,
            },
            SearchResult {
                title: format!("Another result for: {}", query),
                url: "https://example.com/result2".to_string(),
                snippet: format!("Another mock search result for: {}", query),
                rank: 2,
            },
        ];

        Ok(mock_results.into_iter().take(limit).collect())
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

    pub async fn search_with_proxy(&mut self, query: &str, mode: SearchMode, limit: usize, proxy: &str) -> Result<Vec<SearchResult>> {
        match mode {
            SearchMode::Browser => {
                // For browser mode with proxy, we would need to configure the browser with proxy settings
                // This is a simplified implementation
                self.search_browser(query, limit).await
            }
            SearchMode::Api => {
                // For API mode with proxy, we would use a proxy-enabled HTTP client
                let _proxy_client = Client::builder()
                    .timeout(Duration::from_secs(30))
                    .user_agent("Mozilla/5.0 (compatible; Tarsier/1.0)")
                    .proxy(reqwest::Proxy::http(proxy)?)
                    .build()
                    .map_err(|e| TarsierError::Config(format!("Failed to create proxy client: {}", e)))?;

                // Use the proxy client for API calls
                // This is a simplified implementation
                self.search_api(query, limit).await
            }
        }
    }
}

impl Default for SearchEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SearchEngine {
    fn drop(&mut self) {
        // Clean up browser resources if needed
        if let Some(_browser) = &self.browser {
            // Note: In a real implementation, you might want to properly close the browser
            // This is a simplified version
        }
    }
} 