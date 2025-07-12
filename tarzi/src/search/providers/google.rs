use super::super::types::{SearchEngineType, SearchResult};
use crate::Result;
use crate::fetcher::{FetchMode, WebFetcher};
use async_trait::async_trait;
use tracing::info;

#[derive(Debug)]
pub struct GoogleSearchProvider {
    fetcher: WebFetcher,
}

impl GoogleSearchProvider {
    pub fn new_web(fetcher: WebFetcher) -> Self {
        Self { fetcher }
    }
}

#[async_trait]
impl super::SearchProvider for GoogleSearchProvider {
    type Config = crate::fetcher::WebFetcher;

    fn new(config: Self::Config) -> Self {
        Self { fetcher: config }
    }

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

    fn is_healthy(&self) -> bool {
        true // Web provider is always available
    }

    fn get_engine_type(&self) -> SearchEngineType {
        SearchEngineType::Google
    }

    fn supported_modes(&self) -> Vec<super::super::types::SearchMode> {
        vec![super::super::types::SearchMode::WebQuery]
    }
}
