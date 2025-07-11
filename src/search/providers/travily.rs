use super::super::api::SearchApiProvider;
use super::super::types::{SearchEngineType, SearchResult};
use super::{ApiSearchProvider, WebSearchProvider};
use crate::{Result, error::TarziError};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::{Value, json};
use tracing::info;

pub struct TravilySearchProvider {
    api_key: String,
    client: Client,
}

impl TravilySearchProvider {
    pub fn new(api_key: String, client: Client) -> Self {
        Self { api_key, client }
    }

    pub fn new_with_proxy(api_key: String, proxy_url: &str) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .proxy(reqwest::Proxy::http(proxy_url)?)
            .build()
            .map_err(|e| TarziError::Network(format!("Failed to create proxy client: {e}")))?;

        Ok(Self { api_key, client })
    }
}

#[async_trait]
impl WebSearchProvider for TravilySearchProvider {
    async fn search(&mut self, _query: &str, _limit: usize) -> Result<Vec<SearchResult>> {
        // Travily doesn't support web search, only API
        Err(TarziError::Config(
            "Travily only supports API search mode".to_string(),
        ))
    }

    fn get_provider_name(&self) -> &str {
        "Travily Search (Web - Not Supported)"
    }

    fn get_query_pattern(&self) -> &str {
        "" // No web query pattern for Travily
    }

    fn is_healthy(&self) -> bool {
        false // Web provider is not available for Travily
    }

    fn get_engine_type(&self) -> SearchEngineType {
        SearchEngineType::Travily
    }
}

#[async_trait]
impl ApiSearchProvider for TravilySearchProvider {
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let query_pattern = SearchEngineType::Travily
            .get_query_pattern_for_mode(super::super::types::SearchMode::ApiQuery);
        let search_url = query_pattern;
        info!("Travily API search: {}", search_url);

        let response = self
            .client
            .post(&search_url)
            .header("api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&json!({
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

        let data: Value = response.json().await.map_err(|e| {
            TarziError::Network(format!("Failed to parse Travily API response: {e}"))
        })?;

        self.parse_travily_response(data)
    }

    fn get_provider_name(&self) -> &str {
        "Travily Search API"
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
}

// Legacy trait implementation for backward compatibility
#[async_trait]
impl SearchApiProvider for TravilySearchProvider {
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        // Use the API search method for legacy compatibility
        let query_pattern = SearchEngineType::Travily
            .get_query_pattern_for_mode(super::super::types::SearchMode::ApiQuery);
        let search_url = query_pattern;
        info!("Travily API search: {}", search_url);

        let response = self
            .client
            .post(&search_url)
            .header("api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&json!({
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

        let data: Value = response.json().await.map_err(|e| {
            TarziError::Network(format!("Failed to parse Travily API response: {e}"))
        })?;

        self.parse_travily_response(data)
    }

    fn get_provider_name(&self) -> &str {
        "Travily Search API"
    }

    fn is_healthy(&self) -> bool {
        true // API provider is always available if configured
    }
}

impl TravilySearchProvider {
    fn parse_travily_response(&self, data: Value) -> Result<Vec<SearchResult>> {
        let mut results = Vec::new();

        if let Some(results_array) = data.get("results").and_then(|r| r.as_array()) {
            for (index, result) in results_array.iter().enumerate() {
                if let (Some(title), Some(url), Some(content)) = (
                    result.get("title").and_then(|t| t.as_str()),
                    result.get("url").and_then(|u| u.as_str()),
                    result.get("content").and_then(|c| c.as_str()),
                ) {
                    results.push(SearchResult {
                        title: title.to_string(),
                        url: url.to_string(),
                        snippet: content.to_string(),
                        rank: index + 1,
                    });
                }
            }
        }

        Ok(results)
    }
}
