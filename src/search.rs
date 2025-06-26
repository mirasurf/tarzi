use crate::{error::TarsierError, Result};
use chromiumoxide::{
    browser::{Browser, BrowserConfig, HeadlessMode},
    handler::Handler,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::time::Duration;
use std::fs;
use tracing::{info, warn, error, debug};

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
        info!("Initializing SearchEngine");
        let http_client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("Mozilla/5.0 (compatible; Tarsier/1.0)")
            .build()
            .expect("Failed to create HTTP client");

        info!("HTTP client created successfully");
        Self {
            http_client,
            browser: None,
            _handler: None,
            api_key: None,
        }
    }

    pub fn with_api_key(mut self, api_key: String) -> Self {
        info!("Setting API key for SearchEngine");
        self.api_key = Some(api_key);
        self
    }

    pub async fn search(&mut self, query: &str, mode: SearchMode, limit: usize) -> Result<Vec<SearchResult>> {
        info!("Starting search with mode: {:?}, limit: {}", mode, limit);
        match mode {
            SearchMode::Browser => {
                info!("Using browser mode for search");
                self.search_browser(query, limit).await
            }
            SearchMode::Api => {
                info!("Using API mode for search");
                self.search_api(query, limit).await
            }
        }
    }

    async fn search_browser(&mut self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        info!("Starting browser-based search for query: '{}'", query);
        
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

        // Navigate to Google search
        let search_url = format!("https://www.google.com/search?q={}", urlencoding::encode(query));
        info!("Navigating to search URL: {}", search_url);
        
        let navigation_result = tokio::time::timeout(
            Duration::from_secs(30),
            page.goto(&search_url)
        ).await;
        
        match navigation_result {
            Ok(Ok(_)) => {
                info!("Successfully navigated to search page");
            }
            Ok(Err(e)) => {
                error!("Failed to navigate to search URL: {}", e);
                return Err(TarsierError::Browser(format!("Failed to navigate: {}", e)));
            }
            Err(_) => {
                error!("Timeout while navigating to search URL (30 seconds)");
                return Err(TarsierError::Browser("Timeout while navigating to search URL".to_string()));
            }
        }

        // Wait for search results to load
        info!("Waiting for search results to load (2 seconds)...");
        tokio::time::sleep(Duration::from_secs(2)).await;
        info!("Wait completed");

        // Extract search results using JavaScript
        info!("Extracting search results using JavaScript...");
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

        debug!("Executing JavaScript: {}", results_js);
        
        let js_result = tokio::time::timeout(
            Duration::from_secs(30),
            page.evaluate(results_js)
        ).await;
        
        let results: Vec<SearchResult> = match js_result {
            Ok(Ok(eval_result)) => {
                eval_result.into_value()
                    .map_err(|e| {
                        error!("Failed to parse results: {}", e);
                        TarsierError::Browser(format!("Failed to parse results: {}", e))
                    })?
            }
            Ok(Err(e)) => {
                error!("Failed to evaluate JavaScript: {}", e);
                return Err(TarsierError::Browser(format!("Failed to evaluate JavaScript: {}", e)));
            }
            Err(_) => {
                error!("Timeout while evaluating JavaScript (30 seconds)");
                return Err(TarsierError::Browser("Timeout while evaluating JavaScript".to_string()));
            }
        };

        info!("Successfully extracted {} search results", results.len());
        debug!("Search results: {:?}", results);
        
        Ok(results)
    }

    async fn search_api(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        info!("Starting API-based search for query: '{}'", query);
        
        // This is a simplified API search implementation
        // In a real implementation, you would use actual search APIs like Google Custom Search, Bing, etc.
        
        if self.api_key.is_none() {
            warn!("No API key provided for API mode");
            return Err(TarsierError::Config("API key required for API mode".to_string()));
        }

        info!("Using API key for search");
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

        let results: Vec<SearchResult> = mock_results.into_iter().take(limit).collect();
        info!("API search completed, returning {} results", results.len());
        Ok(results)
    }

    /// Clean up stale browser lock files
    fn cleanup_stale_locks() {
        let temp_dir = std::env::temp_dir();
        let lock_pattern = "chromiumoxide-runner";
        
        if let Ok(entries) = fs::read_dir(temp_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() && path.file_name()
                    .and_then(|name| name.to_str())
                    .map(|name| name.contains(lock_pattern))
                    .unwrap_or(false) {
                    
                    info!("Found stale browser directory: {:?}", path);
                    if let Err(e) = fs::remove_dir_all(&path) {
                        warn!("Failed to remove stale browser directory {:?}: {}", path, e);
                    } else {
                        info!("Removed stale browser directory: {:?}", path);
                    }
                }
            }
        }
    }

    async fn get_or_create_browser(&mut self) -> Result<&Browser> {
        if self.browser.is_none() {
            info!("Creating new browser instance...");
            
            // Clean up any stale browser lock files
            Self::cleanup_stale_locks();
            
            // Create a unique temporary directory for this browser instance
            let temp_dir = std::env::temp_dir().join(format!("tarsier-browser-{}", std::process::id()));
            info!("Using browser data directory: {:?}", temp_dir);
            
            let config = BrowserConfig::builder()
                .headless_mode(HeadlessMode::New)
                .no_sandbox()
                .user_data_dir(temp_dir)
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

    pub async fn search_with_proxy(&mut self, query: &str, mode: SearchMode, limit: usize, proxy: &str) -> Result<Vec<SearchResult>> {
        info!("Starting search with proxy: {}", proxy);
        match mode {
            SearchMode::Browser => {
                warn!("Proxy support for browser mode is simplified");
                // For browser mode with proxy, we would need to configure the browser with proxy settings
                // This is a simplified implementation
                self.search_browser(query, limit).await
            }
            SearchMode::Api => {
                info!("Creating proxy-enabled HTTP client");
                // For API mode with proxy, we would use a proxy-enabled HTTP client
                let _proxy_client = Client::builder()
                    .timeout(Duration::from_secs(30))
                    .user_agent("Mozilla/5.0 (compatible; Tarsier/1.0)")
                    .proxy(reqwest::Proxy::http(proxy)?)
                    .build()
                    .map_err(|e| {
                        error!("Failed to create proxy client: {}", e);
                        TarsierError::Config(format!("Failed to create proxy client: {}", e))
                    })?;

                info!("Proxy client created successfully");
                // Use the proxy client for API calls
                // This is a simplified implementation
                self.search_api(query, limit).await
            }
        }
    }

    /// Clean up browser resources and close the browser
    pub async fn cleanup(&mut self) -> Result<()> {
        if let Some(browser) = self.browser.take() {
            info!("Closing browser instance");
            // The browser will be automatically closed when dropped
            drop(browser);
            self._handler = None;
            info!("Browser instance closed");
        }
        Ok(())
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
            info!("Cleaning up browser resources");
            // Note: In a real implementation, you might want to properly close the browser
            // This is a simplified version
        }
    }
} 