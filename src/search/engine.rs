use super::parser::ParserFactory;
use super::types::{SearchEngineType, SearchMode, SearchResult};
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

        // Determine the search mode
        let mode = SearchMode::from_str(&config.search.mode).unwrap_or(SearchMode::WebQuery);

        // Use custom query pattern if provided, otherwise use the default for the engine type and mode
        let query_pattern = if config.search.query_pattern != DEFAULT_QUERY_PATTERN {
            // If a custom query pattern is explicitly set in config, use it
            config.search.query_pattern.clone()
        } else {
            // Otherwise use the engine-specific pattern
            engine_type.get_query_pattern_for_mode(mode)
        };

        Self {
            fetcher,
            engine_type,
            query_pattern,
            user_agent: config.fetcher.user_agent.clone(),
            parser_factory: ParserFactory::new(),
        }
    }

    pub async fn search(
        &mut self,
        query: &str,
        mode: SearchMode,
        limit: usize,
    ) -> Result<Vec<SearchResult>> {
        // Validate that the engine supports the requested mode
        match mode {
            SearchMode::WebQuery => {
                if !self.engine_type.supports_web_query() {
                    return Err(TarziError::Config(format!(
                        "Engine {:?} does not support web query mode",
                        self.engine_type
                    )));
                }
                self.search_browser(query, limit).await
            }
            SearchMode::ApiQuery => {
                if !self.engine_type.supports_api_query() {
                    return Err(TarziError::Config(format!(
                        "Engine {:?} does not support API query mode",
                        self.engine_type
                    )));
                }

                self.search_api(query, limit).await
            }
        }
    }

    async fn search_browser(&mut self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        // Use the query pattern from config to build the search URL
        let search_url = self
            .query_pattern
            .replace("{query}", &urlencoding::encode(query));

        // For webquery mode, default to browser_headless but allow config to override
        let fetch_mode = FetchMode::BrowserHeadless;
        let search_page_content = self.fetcher.fetch_raw(&search_url, fetch_mode).await?;

        // Extract search results from the HTML content using webquery parser
        let results = self.extract_search_results_from_html(
            &search_page_content,
            limit,
            SearchMode::WebQuery,
        )?;

        Ok(results)
    }

    fn extract_search_results_from_html(
        &self,
        html: &str,
        limit: usize,
        mode: SearchMode,
    ) -> Result<Vec<SearchResult>> {
        // Get the appropriate parser for the current engine type and mode
        let parser = self.parser_factory.get_parser(&self.engine_type, mode);

        // Use the parser to extract results
        let results = parser.parse(html, limit)?;

        Ok(results)
    }

    async fn search_api(&mut self, _query: &str, _limit: usize) -> Result<Vec<SearchResult>> {
        // For now, API search is not implemented in the simplified version
        Err(TarziError::Config(
            "API search not implemented in simplified version".to_string(),
        ))
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
        // Enforce fetcher mode constraints based on search mode
        let effective_fetch_mode = match mode {
            SearchMode::ApiQuery => {
                // For apiquery mode, always use plain_request regardless of what's passed
                FetchMode::PlainRequest
            }
            SearchMode::WebQuery => {
                // For webquery mode, use the provided fetch_mode or default to browser_headless
                if matches!(fetch_mode, FetchMode::PlainRequest) {
                    FetchMode::PlainRequest
                } else {
                    FetchMode::BrowserHeadless
                }
            }
        };

        // First, perform the search
        let search_results = self.search(query, mode, limit).await?;

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
                let _proxy_client = match reqwest::Proxy::http(&effective_proxy) {
                    Ok(proxy_config) => {
                        match Client::builder()
                            .timeout(crate::constants::DEFAULT_TIMEOUT)
                            .user_agent(&self.user_agent)
                            .proxy(proxy_config)
                            .build()
                        {
                            Ok(client) => client,
                            Err(e) => {
                                warn!(
                                    "Failed to create HTTP client with proxy '{}': {}. Falling back to no proxy.",
                                    effective_proxy, e
                                );
                                return Err(TarziError::Config(format!(
                                    "Failed to create proxy client: {e}"
                                )));
                            }
                        }
                    }
                    Err(e) => {
                        warn!(
                            "Invalid proxy URL '{}': {}. Falling back to no proxy.",
                            effective_proxy, e
                        );
                        return Err(TarziError::Config(format!("Invalid proxy URL: {e}")));
                    }
                };

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
