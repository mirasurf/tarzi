use super::super::types::{SearchEngineType, SearchResult};
use crate::fetcher::{FetchMode, WebFetcher};
use crate::{Result, error::TarziError};
use async_trait::async_trait;
use reqwest::Client;
use tracing::info;

#[derive(Debug)]
pub struct ExaSearchProvider {
    api_key: Option<String>,
    client: Option<Client>,
    fetcher: WebFetcher,
}

impl ExaSearchProvider {
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
}

/// Configuration for Exa provider
#[derive(Debug)]
pub enum ExaConfig {
    Web { fetcher: Box<WebFetcher> },
    Api { api_key: String, client: Client },
}

#[async_trait]
impl super::SearchProvider for ExaSearchProvider {
    type Config = ExaConfig;

    fn new(config: Self::Config) -> Self {
        match config {
            ExaConfig::Web { fetcher } => Self {
                api_key: None,
                client: None,
                fetcher: *fetcher,
            },
            ExaConfig::Api { api_key, client } => Self {
                api_key: Some(api_key),
                client: Some(client),
                fetcher: WebFetcher::new(),
            },
        }
    }

    async fn search(&mut self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        // Use web search by default, API search if configured
        if self.api_key.is_some() && self.client.is_some() {
            // API search
            let api_key = self
                .api_key
                .as_ref()
                .ok_or_else(|| TarziError::Config("Exa API key not configured".to_string()))?;

            let client = self
                .client
                .as_ref()
                .ok_or_else(|| TarziError::Config("HTTP client not configured".to_string()))?;

            let query_pattern = SearchEngineType::Exa
                .get_query_pattern_for_mode(super::super::types::SearchMode::ApiQuery);
            let search_url = query_pattern;
            info!("Exa API search: {}", search_url);

            let response = client
                .post(&search_url)
                .header("Authorization", &format!("Bearer {api_key}"))
                .json(&serde_json::json!({
                    "query": query,
                    "limit": limit
                }))
                .send()
                .await
                .map_err(|e| TarziError::Network(format!("Exa API request failed: {e}")))?;

            if !response.status().is_success() {
                return Err(TarziError::Network(format!(
                    "Exa API returned error status: {}",
                    response.status()
                )));
            }

            let data: serde_json::Value = response.json().await.map_err(|e| {
                TarziError::Network(format!("Failed to parse Exa API response: {e}"))
            })?;

            // Use the Exa parser to extract results
            let parser = super::super::parser::ParserFactory::new().get_parser(
                &SearchEngineType::Exa,
                super::super::types::SearchMode::ApiQuery,
            );
            parser.parse(&serde_json::to_string(&data)?, limit)
        } else {
            // Web search
            let query_pattern = SearchEngineType::Exa
                .get_query_pattern_for_mode(super::super::types::SearchMode::WebQuery);
            let search_url = query_pattern.replace("{query}", &urlencoding::encode(query));
            info!("Exa web search: {}", search_url);

            let search_page_content = self
                .fetcher
                .fetch_raw(&search_url, FetchMode::BrowserHeadless)
                .await?;

            // Use the Exa parser to extract results
            let parser = super::super::parser::ParserFactory::new().get_parser(
                &SearchEngineType::Exa,
                super::super::types::SearchMode::WebQuery,
            );
            parser.parse(&search_page_content, limit)
        }
    }

    fn is_healthy(&self) -> bool {
        // Both API and web providers are always available
        true
    }

    fn get_engine_type(&self) -> SearchEngineType {
        SearchEngineType::Exa
    }

    fn requires_api_key(&self) -> bool {
        self.api_key.is_some()
    }

    fn supported_modes(&self) -> Vec<super::super::types::SearchMode> {
        vec![
            super::super::types::SearchMode::WebQuery,
            super::super::types::SearchMode::ApiQuery,
        ]
    }
}
