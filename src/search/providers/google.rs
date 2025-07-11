use super::super::api::SearchApiProvider;
use super::super::types::{SearchEngineType, SearchResult};
use super::WebSearchProvider;
use crate::Result;
use crate::fetcher::{FetchMode, WebFetcher};
use async_trait::async_trait;
use tracing::info;

pub struct GoogleSearchProvider {
    fetcher: WebFetcher,
}

impl GoogleSearchProvider {
    pub fn new_web(fetcher: WebFetcher) -> Self {
        Self { fetcher }
    }
}

#[async_trait]
impl WebSearchProvider for GoogleSearchProvider {
    async fn search(&mut self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let query_pattern = SearchEngineType::Google
            .get_query_pattern_for_mode(super::super::types::SearchMode::WebQuery);
        let search_url = query_pattern.replace("{query}", &urlencoding::encode(query));
        info!("Google web search: {}", search_url);

        let search_page_content = self
            .fetcher
            .fetch_raw(&search_url, FetchMode::BrowserHeadless)
            .await?;

        // Use the Google parser to extract results
        let parser = super::super::parser::ParserFactory::new().get_parser(
            &SearchEngineType::Google,
            super::super::types::SearchMode::WebQuery,
        );
        parser.parse(&search_page_content, limit)
    }

    fn get_provider_name(&self) -> &str {
        "Google Search (Web)"
    }

    fn get_query_pattern(&self) -> &str {
        "https://www.google.com/search?q={query}"
    }

    fn is_healthy(&self) -> bool {
        true // Web provider is always available
    }

    fn get_engine_type(&self) -> SearchEngineType {
        SearchEngineType::Google
    }
}

// Legacy trait implementation for backward compatibility
#[async_trait]
impl SearchApiProvider for GoogleSearchProvider {
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        // Use the web search method for legacy compatibility
        let query_pattern = SearchEngineType::Google
            .get_query_pattern_for_mode(super::super::types::SearchMode::WebQuery);
        let search_url = query_pattern.replace("{query}", &urlencoding::encode(query));
        info!("Google web search: {}", search_url);

        // Create a new fetcher instance for the legacy trait
        let mut fetcher = WebFetcher::new();
        let search_page_content = fetcher
            .fetch_raw(&search_url, FetchMode::BrowserHeadless)
            .await?;

        // Use the Google parser to extract results
        let parser = super::super::parser::ParserFactory::new().get_parser(
            &SearchEngineType::Google,
            super::super::types::SearchMode::WebQuery,
        );
        parser.parse(&search_page_content, limit)
    }

    fn get_provider_name(&self) -> &str {
        "Google Search (Web)"
    }

    fn is_healthy(&self) -> bool {
        true // Web provider is always available
    }
}
