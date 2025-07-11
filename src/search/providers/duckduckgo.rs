use super::super::api::SearchApiProvider;
use super::super::types::{SearchEngineType, SearchResult};
use super::{ApiSearchProvider, WebSearchProvider};
use crate::fetcher::{FetchMode, WebFetcher};
use crate::{Result, error::TarziError};
use async_trait::async_trait;
use reqwest::Client;
use tracing::{info, warn};

pub struct DuckDuckGoProvider {
    fetcher: WebFetcher,
}

impl DuckDuckGoProvider {
    pub fn new_web(fetcher: WebFetcher) -> Self {
        Self { fetcher }
    }

    pub fn new_api(_client: Client) -> Self {
        Self {
            fetcher: WebFetcher::new(),
        }
    }

    pub fn new_api_with_proxy(_proxy_url: &str) -> Result<Self> {
        let _client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| TarziError::Network(format!("Failed to create proxy client: {e}")))?;

        Ok(Self {
            fetcher: WebFetcher::new(),
        })
    }
}

#[async_trait]
impl WebSearchProvider for DuckDuckGoProvider {
    async fn search(&mut self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let query_pattern = SearchEngineType::DuckDuckGo
            .get_query_pattern_for_mode(super::super::types::SearchMode::WebQuery);
        let search_url = query_pattern.replace("{query}", &urlencoding::encode(query));
        info!("DuckDuckGo web search: {}", search_url);

        let search_page_content = self
            .fetcher
            .fetch_raw(&search_url, FetchMode::BrowserHeadless)
            .await?;

        // Use the DuckDuckGo parser to extract results
        let parser = super::super::parser::ParserFactory::new().get_parser(
            &SearchEngineType::DuckDuckGo,
            super::super::types::SearchMode::WebQuery,
        );
        parser.parse(&search_page_content, limit)
    }

    fn get_provider_name(&self) -> &str {
        "DuckDuckGo (Web)"
    }

    fn get_query_pattern(&self) -> &str {
        "https://duckduckgo.com/?q={query}"
    }

    fn is_healthy(&self) -> bool {
        true // Web provider is always available
    }

    fn get_engine_type(&self) -> SearchEngineType {
        SearchEngineType::DuckDuckGo
    }
}

#[async_trait]
impl ApiSearchProvider for DuckDuckGoProvider {
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let query_pattern = SearchEngineType::DuckDuckGo
            .get_query_pattern_for_mode(super::super::types::SearchMode::ApiQuery);
        let search_url = query_pattern.replace("{query}", &urlencoding::encode(query));
        info!("DuckDuckGo API search: {}", search_url);

        // Note: DuckDuckGo API has limited functionality and returns different format
        // For now, we'll use the web search method as a fallback
        warn!("DuckDuckGo API has limited functionality, falling back to web search");

        // Create a new fetcher instance for the API provider
        let mut fetcher = WebFetcher::new();
        let search_page_content = fetcher
            .fetch_raw(&search_url, FetchMode::PlainRequest)
            .await?;

        // Use the DuckDuckGo parser to extract results
        let parser = super::super::parser::ParserFactory::new().get_parser(
            &SearchEngineType::DuckDuckGo,
            super::super::types::SearchMode::ApiQuery,
        );
        parser.parse(&search_page_content, limit)
    }

    fn get_provider_name(&self) -> &str {
        "DuckDuckGo (API)"
    }

    fn is_healthy(&self) -> bool {
        true // API provider is always available
    }

    fn get_engine_type(&self) -> SearchEngineType {
        SearchEngineType::DuckDuckGo
    }

    fn requires_api_key(&self) -> bool {
        false
    }
}

// Legacy trait implementation for backward compatibility
#[async_trait]
impl SearchApiProvider for DuckDuckGoProvider {
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        // Use the API search method for legacy compatibility
        let query_pattern = SearchEngineType::DuckDuckGo
            .get_query_pattern_for_mode(super::super::types::SearchMode::ApiQuery);
        let search_url = query_pattern.replace("{query}", &urlencoding::encode(query));
        info!("DuckDuckGo API search: {}", search_url);

        // Create a new fetcher instance for the legacy trait
        let mut fetcher = WebFetcher::new();
        let search_page_content = fetcher
            .fetch_raw(&search_url, FetchMode::PlainRequest)
            .await?;

        // Use the DuckDuckGo parser to extract results
        let parser = super::super::parser::ParserFactory::new().get_parser(
            &SearchEngineType::DuckDuckGo,
            super::super::types::SearchMode::ApiQuery,
        );
        parser.parse(&search_page_content, limit)
    }

    fn get_provider_name(&self) -> &str {
        "DuckDuckGo (API)"
    }

    fn is_healthy(&self) -> bool {
        true // API provider is always available
    }
}
