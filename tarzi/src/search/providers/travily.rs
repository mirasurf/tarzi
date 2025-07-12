use super::super::types::{SearchEngineType, SearchResult};
use crate::{Result, error::TarziError};
use async_trait::async_trait;
use reqwest::Client;
use tracing::info;

#[derive(Debug)]
pub struct TravilySearchProvider {
    api_key: String,
    client: Client,
}

impl TravilySearchProvider {
    pub fn new_api(api_key: String, client: Client) -> Self {
        Self { api_key, client }
    }
}

/// Configuration for Travily provider
#[derive(Debug)]
pub enum TravilyConfig {
    Api { api_key: String, client: Client },
}

#[async_trait]
impl super::SearchProvider for TravilySearchProvider {
    type Config = TravilyConfig;

    fn new(config: Self::Config) -> Self {
        match config {
            TravilyConfig::Api { api_key, client } => Self { api_key, client },
        }
    }

    async fn search(&mut self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let query_pattern = SearchEngineType::Travily
            .get_query_pattern_for_mode(super::super::types::SearchMode::ApiQuery);
        let search_url = query_pattern;
        info!("Travily API search: {}", search_url);

        let response = self
            .client
            .post(&search_url)
            .header("api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "query": query,
                "search_depth": "basic",
                "include_answer": false,
                "include_raw_content": false,
                "include_images": false,
                "max_results": limit
            }))
            .send()
            .await
            .map_err(|e| TarziError::Network(format!("Travily API request failed: {e}")))?;

        if !response.status().is_success() {
            return Err(TarziError::Network(format!(
                "Travily API returned error status: {}",
                response.status()
            )));
        }

        let data: serde_json::Value = response.json().await.map_err(|e| {
            TarziError::Network(format!("Failed to parse Travily API response: {e}"))
        })?;

        // Use the Travily parser to extract results
        let parser = super::super::parser::ParserFactory::new().get_parser(
            &SearchEngineType::Travily,
            super::super::types::SearchMode::ApiQuery,
        );
        parser.parse(&serde_json::to_string(&data)?, limit)
    }

    fn is_healthy(&self) -> bool {
        true // API provider is always available if configured
    }

    fn get_engine_type(&self) -> SearchEngineType {
        SearchEngineType::Travily
    }

    fn requires_api_key(&self) -> bool {
        true
    }

    fn supported_modes(&self) -> Vec<super::super::types::SearchMode> {
        vec![super::super::types::SearchMode::ApiQuery]
    }
}
