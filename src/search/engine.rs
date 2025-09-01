use super::parser::ParserFactory;
use super::types::{SearchEngineType, SearchResult};
use crate::config::Config;
use crate::{
    Result,
    error::TarziError,
    fetcher::{FetchMode, WebFetcher},
};
use reqwest::Client;
use std::str::FromStr;

use crate::constants::DEFAULT_QUERY_PATTERN;
use tracing::{info, warn};

pub struct SearchEngine {
    fetcher: WebFetcher,
    engine_type: SearchEngineType,
    query_pattern: String,
    user_agent: String,
    parser_factory: ParserFactory,
}

impl SearchEngine {
    pub fn new() -> Self {
        // Initialize SearchEngine with default configuration
        Self {
            fetcher: WebFetcher::new(),
            engine_type: SearchEngineType::DuckDuckGo,
            query_pattern: SearchEngineType::DuckDuckGo.get_query_pattern(),
            user_agent: crate::constants::DEFAULT_USER_AGENT.to_string(),
            parser_factory: ParserFactory::new(),
        }
    }

    // Getter methods for testing
    pub fn engine_type(&self) -> &SearchEngineType {
        &self.engine_type
    }

    pub fn query_pattern(&self) -> &str {
        &self.query_pattern
    }

    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }

    // Custom parser registration removed - custom engines are no longer supported

    pub fn from_config(config: &Config) -> Self {
        let fetcher = crate::fetcher::WebFetcher::from_config(config);

        // Parse the search engine type from config
        let engine_type = SearchEngineType::from_str(&config.search.engine)
            .unwrap_or(SearchEngineType::DuckDuckGo);

        // Use custom query pattern if provided, otherwise use the default for the engine type
        let query_pattern = if config.search.query_pattern != DEFAULT_QUERY_PATTERN {
            // If a custom query pattern is explicitly set in config, use it
            config.search.query_pattern.clone()
        } else {
            // Otherwise use the engine-specific pattern
            engine_type.get_query_pattern()
        };

        Self {
            fetcher,
            engine_type,
            query_pattern,
            user_agent: config.fetcher.user_agent.clone(),
            parser_factory: ParserFactory::new(),
        }
    }

    pub async fn search(&mut self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        self.search_browser(query, limit).await
    }

    async fn search_browser(&mut self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        // Use the query pattern from config to build the search URL
        let search_url = self
            .query_pattern
            .replace("{query}", &urlencoding::encode(query));

        // Try browser mode first, then fallback to plain HTTP if it fails
        let search_page_content = match self
            .fetch_with_retry(&search_url, FetchMode::BrowserHeadless)
            .await
        {
            Ok(content) => content,
            Err(browser_error) => {
                return Err(TarziError::Search(format!(
                    "Browser mode failed: {browser_error}"
                )));
            }
        };

        // Extract search results from the HTML content using web parser
        let results = self.extract_search_results_from_html(&search_page_content, limit)?;

        Ok(results)
    }

    async fn fetch_with_retry(&mut self, url: &str, fetch_mode: FetchMode) -> Result<String> {
        const MAX_RETRIES: usize = 3;
        const RETRY_DELAY: std::time::Duration = std::time::Duration::from_secs(2);

        for attempt in 1..=MAX_RETRIES {
            match self.fetcher.fetch_raw(url, fetch_mode).await {
                Ok(content) => {
                    if attempt > 1 {
                        info!("Successfully fetched content on attempt {}", attempt);
                    }
                    return Ok(content);
                }
                Err(e) => {
                    let error_str = e.to_string();
                    let is_network_error = error_str.contains("nssFailure")
                        || error_str.contains("network")
                        || error_str.contains("timeout")
                        || error_str.contains("connection");

                    if is_network_error && attempt < MAX_RETRIES {
                        warn!(
                            "Network error on attempt {}: {}. Retrying in {} seconds...",
                            attempt,
                            e,
                            RETRY_DELAY.as_secs()
                        );
                        tokio::time::sleep(RETRY_DELAY).await;
                        continue;
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        // This should never be reached, but just in case
        Err(TarziError::Network("Max retries exceeded".to_string()))
    }

    fn extract_search_results_from_html(
        &self,
        html: &str,
        limit: usize,
    ) -> Result<Vec<SearchResult>> {
        let parser = self.parser_factory.get_parser(&self.engine_type);

        // Use the parser to extract results
        let results = parser.parse(html, limit)?;

        Ok(results)
    }

    /// Search and fetch content for each result
    pub async fn search_with_content(
        &mut self,
        query: &str,
        limit: usize,
        fetch_mode: FetchMode,
        format: crate::converter::Format,
    ) -> Result<Vec<(SearchResult, String)>> {
        // For web search, use the provided fetch_mode or default to browser_headless
        let effective_fetch_mode = if matches!(fetch_mode, FetchMode::PlainRequest) {
            FetchMode::PlainRequest
        } else {
            FetchMode::BrowserHeadless
        };

        // First, perform the search
        let search_results = self.search(query, limit).await?;

        // Then, fetch content for each result using the effective fetch mode
        let mut results_with_content = Vec::new();

        for result in search_results.clone() {
            match self
                .fetcher
                .fetch(&result.url, effective_fetch_mode, format)
                .await
            {
                Ok(content) => {
                    results_with_content.push((result, content));
                }
                Err(e) => {
                    warn!("Failed to fetch content for {}: {}", result.url, e);
                    // Continue with other results even if one fails
                }
            }
        }

        Ok(results_with_content)
    }

    pub async fn search_with_proxy(
        &mut self,
        query: &str,
        limit: usize,
        proxy: &str,
    ) -> Result<Vec<SearchResult>> {
        info!("Starting search with proxy: {}", proxy);

        // Use environment variables for proxy with fallback to provided proxy
        let effective_proxy = crate::config::get_proxy_from_env_or_config(&Some(proxy.to_string()))
            .unwrap_or_else(|| proxy.to_string());

        warn!("Proxy support for browser mode is simplified");
        // For browser mode with proxy, we would need to configure the browser with proxy settings
        // This is a simplified implementation.
        // FIXME (xiaming.cxm): to be implemented.
        self.search_browser(query, limit).await
    }

    /// Clean up resources
    pub async fn cleanup(&mut self) -> Result<()> {
        info!("Cleaning up SearchEngine resources");
        // The fetcher will handle its own cleanup
        Ok(())
    }

    /// Explicitly shut down browser and driver resources
    pub async fn shutdown(&mut self) {
        self.fetcher.shutdown().await;
    }
}

impl Default for SearchEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SearchEngine {
    fn drop(&mut self) {
        info!("SearchEngine dropping - cleanup will be handled by WebFetcher");
        // The fetcher will handle its own cleanup
    }
}
