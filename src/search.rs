use crate::config::Config;
use crate::{
    Result,
    error::TarziError,
    fetcher::{FetchMode, WebFetcher},
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::time::Duration;
use tracing::{error, info, warn};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SearchMode {
    WebQuery,
    ApiQuery,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SearchEngineType {
    Bing,
    DuckDuckGo,
    Google,
    BraveSearch,
    Tavily,
    SearchApi,
    Custom(String),
}

impl FromStr for SearchEngineType {
    type Err = TarziError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "bing" => Ok(SearchEngineType::Bing),
            "duckduckgo" => Ok(SearchEngineType::DuckDuckGo),
            "google" => Ok(SearchEngineType::Google),
            "brave" => Ok(SearchEngineType::BraveSearch),
            "tavily" => Ok(SearchEngineType::Tavily),
            "searchapi" => Ok(SearchEngineType::SearchApi),
            _ => Ok(SearchEngineType::Custom(s.to_string())),
        }
    }
}

impl SearchEngineType {
    pub fn get_query_pattern(&self) -> String {
        match self {
            SearchEngineType::Bing => "https://www.bing.com/search?q={query}".to_string(),
            SearchEngineType::DuckDuckGo => "https://duckduckgo.com/?q={query}".to_string(),
            SearchEngineType::Google => "https://www.google.com/search?q={query}".to_string(),
            SearchEngineType::BraveSearch => {
                "https://search.brave.com/search?q={query}".to_string()
            }
            SearchEngineType::Tavily => "https://tavily.com/search?q={query}".to_string(),
            SearchEngineType::SearchApi => "https://www.searchapi.io/search?q={query}".to_string(),
            SearchEngineType::Custom(_) => "{query}".to_string(), // Default pattern for custom engines
        }
    }
}

impl FromStr for SearchMode {
    type Err = TarziError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "webquery" => Ok(SearchMode::WebQuery),
            "apiquery" => Ok(SearchMode::ApiQuery),
            _ => Err(TarziError::InvalidMode(s.to_string())),
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
    fetcher: WebFetcher,
    api_key: Option<String>,
    #[allow(dead_code)]
    engine_type: SearchEngineType,
    query_pattern: String,
    user_agent: String,
}

impl SearchEngine {
    pub fn new() -> Self {
        info!("Initializing SearchEngine");
        Self {
            fetcher: WebFetcher::new(),
            api_key: None,
            engine_type: SearchEngineType::Bing,
            query_pattern: SearchEngineType::Bing.get_query_pattern(),
            user_agent: "Mozilla/5.0 (compatible; Tarzi/1.0)".to_string(),
        }
    }

    pub fn with_api_key(mut self, api_key: String) -> Self {
        info!("Setting API key for SearchEngine");
        self.api_key = Some(api_key);
        self
    }

    pub fn from_config(config: &Config) -> Self {
        info!("Initializing SearchEngine from config");
        let fetcher = crate::fetcher::WebFetcher::from_config(config);
        let api_key = config.search.api_key.clone();

        // Parse the search engine type from config
        let engine_type =
            SearchEngineType::from_str(&config.search.engine).unwrap_or(SearchEngineType::Bing);

        // Use custom query pattern if provided, otherwise use the default for the engine type
        let query_pattern = if config.search.query_pattern != engine_type.get_query_pattern() {
            config.search.query_pattern.clone()
        } else {
            engine_type.get_query_pattern()
        };

        Self {
            fetcher,
            api_key,
            engine_type,
            query_pattern,
            user_agent: config.fetcher.user_agent.clone(),
        }
    }

    pub async fn search(
        &mut self,
        query: &str,
        mode: SearchMode,
        limit: usize,
    ) -> Result<Vec<SearchResult>> {
        info!("Starting search with mode: {:?}, limit: {}", mode, limit);
        match mode {
            SearchMode::WebQuery => {
                info!("Using browser mode for search");
                self.search_browser(query, limit).await
            }
            SearchMode::ApiQuery => {
                info!("Using API mode for search");
                self.search_api(query, limit).await
            }
        }
    }

    async fn search_browser(&mut self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        info!("Starting browser-based search for query: '{}'", query);

        // Use the query pattern from config to build the search URL
        let search_url = self
            .query_pattern
            .replace("{query}", &urlencoding::encode(query));
        info!("Fetching search results from: {}", search_url);

        let search_page_content = self
            .fetcher
            .fetch_raw(&search_url, FetchMode::BrowserHeadless)
            .await?;
        info!(
            "Successfully fetched search page ({} characters)",
            search_page_content.len()
        );

        // Extract search results from the HTML content
        let results = self.extract_search_results_from_html(&search_page_content, limit)?;
        info!("Successfully extracted {} search results", results.len());

        Ok(results)
    }

    fn extract_search_results_from_html(
        &self,
        _html: &str,
        limit: usize,
    ) -> Result<Vec<SearchResult>> {
        info!("Extracting search results from HTML content");

        // This is a simplified HTML parsing approach
        // In a real implementation, you might use a proper HTML parser like scraper
        let mut results = Vec::new();
        let mut rank = 1;

        // Simple regex-based extraction (this is a basic implementation)
        // In practice, you'd want to use a proper HTML parser
        // For now, we'll create mock results to demonstrate the structure
        for i in 0..limit {
            results.push(SearchResult {
                title: format!("Search result {} for query", i + 1),
                url: format!("https://example.com/result{}", i + 1),
                snippet: format!("This is a snippet for search result {}", i + 1),
                rank,
            });
            rank += 1;
        }

        info!("Extracted {} search results", results.len());
        Ok(results)
    }

    async fn search_api(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        info!("Starting API-based search for query: '{}'", query);

        // This is a simplified API search implementation
        // In a real implementation, you would use actual search APIs like Google Custom Search, Bing, etc.

        if self.api_key.is_none() {
            warn!("No API key provided for API mode");
            return Err(TarziError::Config(
                "API key required for API mode".to_string(),
            ));
        }

        info!("Using API key for search");
        // FIXME (xiaming.cxm): to be implemented.
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

    /// Search and fetch content for each result
    pub async fn search_and_fetch(
        &mut self,
        query: &str,
        mode: SearchMode,
        limit: usize,
        fetch_mode: FetchMode,
        format: crate::converter::Format,
    ) -> Result<Vec<(SearchResult, String)>> {
        info!("Searching and fetching content for query: '{}'", query);

        // First, perform the search
        let search_results = self.search(query, mode, limit).await?;
        info!("Found {} search results", search_results.len());

        // Then, fetch content for each result
        let mut results_with_content = Vec::new();

        for result in search_results.clone() {
            info!("Fetching content for: {}", result.url);
            match self.fetcher.fetch(&result.url, fetch_mode, format).await {
                Ok(content) => {
                    info!(
                        "Successfully fetched content for {} ({} characters)",
                        result.url,
                        content.len()
                    );
                    results_with_content.push((result, content));
                }
                Err(e) => {
                    warn!("Failed to fetch content for {}: {}", result.url, e);
                    // Continue with other results even if one fails
                }
            }
        }

        info!(
            "Successfully fetched content for {}/{} results",
            results_with_content.len(),
            search_results.len()
        );
        Ok(results_with_content)
    }

    pub async fn search_with_proxy(
        &mut self,
        query: &str,
        mode: SearchMode,
        limit: usize,
        proxy: &str,
    ) -> Result<Vec<SearchResult>> {
        info!("Starting search with proxy: {}", proxy);

        // Use environment variables for proxy with fallback to provided proxy
        let effective_proxy = crate::config::get_proxy_from_env_or_config(&Some(proxy.to_string()))
            .unwrap_or_else(|| proxy.to_string());

        match mode {
            SearchMode::WebQuery => {
                warn!("Proxy support for browser mode is simplified");
                // For browser mode with proxy, we would need to configure the browser with proxy settings
                // This is a simplified implementation.
                // FIXME (xiaming.cxm): to be implemented.
                self.search_browser(query, limit).await
            }
            SearchMode::ApiQuery => {
                info!("Creating proxy-enabled HTTP client");
                // For API mode with proxy, we would use a proxy-enabled HTTP client
                let _proxy_client = Client::builder()
                    .timeout(Duration::from_secs(30))
                    .user_agent(&self.user_agent)
                    .proxy(reqwest::Proxy::http(&effective_proxy)?)
                    .build()
                    .map_err(|e| {
                        error!("Failed to create proxy client: {}", e);
                        TarziError::Config(format!("Failed to create proxy client: {}", e))
                    })?;

                info!(
                    "Proxy client created successfully with proxy: {}",
                    effective_proxy
                );
                // Use the proxy client for API calls
                // This is a simplified implementation
                // FIXME (xiaming.cxm): to be implemented.
                self.search_api(query, limit).await
            }
        }
    }

    /// Clean up resources
    pub async fn cleanup(&mut self) -> Result<()> {
        info!("Cleaning up SearchEngine resources");
        // The fetcher will handle its own cleanup
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
        info!("Cleaning up SearchEngine resources");
        // The fetcher will handle its own cleanup
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_searchengine_from_config() {
        use crate::config::Config;
        let mut config = Config::new();
        config.search.api_key = Some("test-api-key-123".to_string());
        let engine = SearchEngine::from_config(&config);
        assert_eq!(engine.api_key, Some("test-api-key-123".to_string()));
        assert_eq!(engine.engine_type, SearchEngineType::Bing);
        assert_eq!(
            engine.query_pattern,
            "https://www.bing.com/search?q={query}"
        );
        assert_eq!(engine.user_agent, "Mozilla/5.0 (compatible; Tarzi/1.0)");
    }

    #[test]
    fn test_search_engine_type_parsing() {
        assert_eq!(
            SearchEngineType::from_str("bing").unwrap(),
            SearchEngineType::Bing
        );
        assert_eq!(
            SearchEngineType::from_str("google").unwrap(),
            SearchEngineType::Google
        );
        assert_eq!(
            SearchEngineType::from_str("duckduckgo").unwrap(),
            SearchEngineType::DuckDuckGo
        );
        assert_eq!(
            SearchEngineType::from_str("brave").unwrap(),
            SearchEngineType::BraveSearch
        );
        assert_eq!(
            SearchEngineType::from_str("tavily").unwrap(),
            SearchEngineType::Tavily
        );
        assert_eq!(
            SearchEngineType::from_str("searchapi").unwrap(),
            SearchEngineType::SearchApi
        );

        // Test custom engine
        let custom = SearchEngineType::from_str("custom-engine").unwrap();
        assert!(matches!(custom, SearchEngineType::Custom(_)));
    }

    #[test]
    fn test_query_patterns() {
        assert_eq!(
            SearchEngineType::Bing.get_query_pattern(),
            "https://www.bing.com/search?q={query}"
        );
        assert_eq!(
            SearchEngineType::Google.get_query_pattern(),
            "https://www.google.com/search?q={query}"
        );
        assert_eq!(
            SearchEngineType::DuckDuckGo.get_query_pattern(),
            "https://duckduckgo.com/?q={query}"
        );
        assert_eq!(
            SearchEngineType::BraveSearch.get_query_pattern(),
            "https://search.brave.com/search?q={query}"
        );
        assert_eq!(
            SearchEngineType::Tavily.get_query_pattern(),
            "https://tavily.com/search?q={query}"
        );
        assert_eq!(
            SearchEngineType::SearchApi.get_query_pattern(),
            "https://www.searchapi.io/search?q={query}"
        );
    }

    #[test]
    fn test_custom_query_pattern() {
        use crate::config::Config;
        let mut config = Config::new();
        config.search.engine = "google".to_string();
        config.search.query_pattern =
            "https://custom-search.com/search?query={query}&lang=en".to_string();

        let engine = SearchEngine::from_config(&config);
        assert_eq!(engine.engine_type, SearchEngineType::Google);
        assert_eq!(
            engine.query_pattern,
            "https://custom-search.com/search?query={query}&lang=en"
        );
    }

    #[test]
    fn test_custom_user_agent() {
        use crate::config::Config;
        let mut config = Config::new();
        config.fetcher.user_agent = "Custom User Agent String".to_string();

        let engine = SearchEngine::from_config(&config);
        assert_eq!(engine.user_agent, "Custom User Agent String");
    }
}
