use super::super::types::{SearchEngineType, SearchResult};
use crate::Result;
use crate::fetcher::{FetchMode, WebFetcher};
use async_trait::async_trait;
use reqwest::Client;
use tracing::info;

#[derive(Debug)]
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
}

/// Configuration for DuckDuckGo provider
#[derive(Debug)]
pub enum DuckDuckGoConfig {
    Web { fetcher: Box<WebFetcher> },
    Api { client: Client },
}

#[async_trait]
impl super::SearchProvider for DuckDuckGoProvider {
    type Config = DuckDuckGoConfig;

    fn new(config: Self::Config) -> Self {
        match config {
            DuckDuckGoConfig::Web { fetcher } => Self { fetcher: *fetcher },
            DuckDuckGoConfig::Api { .. } => Self {
                fetcher: WebFetcher::new(),
            },
        }
    }

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

    fn is_healthy(&self) -> bool {
        true // Web provider is always available
    }

    fn get_engine_type(&self) -> SearchEngineType {
        SearchEngineType::DuckDuckGo
    }

    fn supported_modes(&self) -> Vec<super::super::types::SearchMode> {
        vec![
            super::super::types::SearchMode::WebQuery,
            super::super::types::SearchMode::ApiQuery,
        ]
    }
}
