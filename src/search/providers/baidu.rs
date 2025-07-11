use super::super::types::{SearchEngineType, SearchResult};
use super::{ApiSearchProvider, WebSearchProvider};
use crate::fetcher::{FetchMode, WebFetcher};
use crate::{Result, error::TarziError};
use async_trait::async_trait;
use reqwest::Client;

use tracing::info;

pub struct BaiduSearchProvider {
    api_key: Option<String>,
    client: Option<Client>,
    fetcher: WebFetcher,
}

impl BaiduSearchProvider {
    pub fn new_web(fetcher: WebFetcher) -> Self {
        Self {
            api_key: None,
            client: None,
            fetcher,
        }
    }

    pub fn new_api(api_key: String, client: Client) -> Self {
        Self {
            api_key: Some(api_key),
            client: Some(client),
            fetcher: WebFetcher::new(),
        }
    }

    pub fn new_api_with_proxy(api_key: String, proxy_url: &str) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .proxy(reqwest::Proxy::http(proxy_url)?)
            .build()
            .map_err(|e| TarziError::Network(format!("Failed to create proxy client: {e}")))?;

        Ok(Self {
            api_key: Some(api_key),
            client: Some(client),
            fetcher: WebFetcher::new(),
        })
    }
}

#[async_trait]
impl WebSearchProvider for BaiduSearchProvider {
    async fn search(&mut self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let query_pattern = SearchEngineType::Baidu
            .get_query_pattern_for_mode(super::super::types::SearchMode::WebQuery);
        let search_url = query_pattern.replace("{query}", &urlencoding::encode(query));
        info!("Baidu web search: {}", search_url);

        let search_page_content = self
            .fetcher
            .fetch_raw(&search_url, FetchMode::BrowserHeadless)
            .await?;

        // Use the Baidu parser to extract results
        let parser = super::super::parser::ParserFactory::new().get_parser(
            &SearchEngineType::Baidu,
            super::super::types::SearchMode::WebQuery,
        );
        parser.parse(&search_page_content, limit)
    }

    fn get_provider_name(&self) -> &str {
        "Baidu Search (Web)"
    }

    fn get_query_pattern(&self) -> &str {
        "https://www.baidu.com/s?wd={query}"
    }

    fn is_healthy(&self) -> bool {
        true // Web provider is always available
    }

    fn get_engine_type(&self) -> SearchEngineType {
        SearchEngineType::Baidu
    }
}

#[async_trait]
impl ApiSearchProvider for BaiduSearchProvider {
    async fn search(&self, _query: &str, _limit: usize) -> Result<Vec<SearchResult>> {
        // Note: Baidu API implementation is not fully implemented yet
        // This is a placeholder for future implementation
        Err(TarziError::Config(
            "Baidu API provider is not fully implemented yet".to_string(),
        ))
    }

    fn get_provider_name(&self) -> &str {
        "Baidu Search API"
    }

    fn is_healthy(&self) -> bool {
        self.api_key.is_some() && self.client.is_some()
    }

    fn get_engine_type(&self) -> SearchEngineType {
        SearchEngineType::Baidu
    }

    fn requires_api_key(&self) -> bool {
        true
    }
}
