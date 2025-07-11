use super::super::api::SearchApiProvider;
use super::super::types::{SearchEngineType, SearchResult};
use super::{ApiSearchProvider, WebSearchProvider};
use crate::fetcher::{FetchMode, WebFetcher};
use crate::{Result, error::TarziError};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;
use tracing::info;

pub struct BraveSearchProvider {
    api_key: Option<String>,
    client: Option<Client>,
    fetcher: WebFetcher,
}

impl BraveSearchProvider {
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
impl WebSearchProvider for BraveSearchProvider {
    async fn search(&mut self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let query_pattern = SearchEngineType::BraveSearch
            .get_query_pattern_for_mode(super::super::types::SearchMode::WebQuery);
        let search_url = query_pattern.replace("{query}", &urlencoding::encode(query));
        info!("Brave web search: {}", search_url);

        let search_page_content = self
            .fetcher
            .fetch_raw(&search_url, FetchMode::BrowserHeadless)
            .await?;

        // Use the Brave parser to extract results
        let parser = super::super::parser::ParserFactory::new().get_parser(
            &SearchEngineType::BraveSearch,
            super::super::types::SearchMode::WebQuery,
        );
        parser.parse(&search_page_content, limit)
    }

    fn get_provider_name(&self) -> &str {
        "Brave Search (Web)"
    }

    fn get_query_pattern(&self) -> &str {
        "https://search.brave.com/search?q={query}"
    }

    fn is_healthy(&self) -> bool {
        true // Web provider is always available
    }

    fn get_engine_type(&self) -> SearchEngineType {
        SearchEngineType::BraveSearch
    }
}

#[async_trait]
impl ApiSearchProvider for BraveSearchProvider {
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let api_key = self
            .api_key
            .as_ref()
            .ok_or_else(|| TarziError::Config("Brave API key not configured".to_string()))?;

        let client = self
            .client
            .as_ref()
            .ok_or_else(|| TarziError::Config("HTTP client not configured".to_string()))?;

        let query_pattern = SearchEngineType::BraveSearch
            .get_query_pattern_for_mode(super::super::types::SearchMode::ApiQuery);
        let search_url = query_pattern;
        info!("Brave API search: {}", search_url);

        let response = client
            .get(&search_url)
            .header("X-Subscription-Token", api_key)
            .query(&[("q", query), ("count", &limit.to_string())])
            .send()
            .await
            .map_err(|e| TarziError::Network(format!("Brave API request failed: {e}")))?;

        if !response.status().is_success() {
            return Err(TarziError::Network(format!(
                "Brave API returned error status: {}",
                response.status()
            )));
        }

        let data: Value = response
            .json()
            .await
            .map_err(|e| TarziError::Network(format!("Failed to parse Brave API response: {e}")))?;

        self.parse_brave_response(data)
    }

    fn get_provider_name(&self) -> &str {
        "Brave Search API"
    }

    fn is_healthy(&self) -> bool {
        self.api_key.is_some() && self.client.is_some()
    }

    fn get_engine_type(&self) -> SearchEngineType {
        SearchEngineType::BraveSearch
    }

    fn requires_api_key(&self) -> bool {
        true
    }
}

// Legacy trait implementation for backward compatibility
#[async_trait]
impl SearchApiProvider for BraveSearchProvider {
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        // Use the API search method for legacy compatibility
        let api_key = self
            .api_key
            .as_ref()
            .ok_or_else(|| TarziError::Config("Brave API key not configured".to_string()))?;

        let client = self
            .client
            .as_ref()
            .ok_or_else(|| TarziError::Config("HTTP client not configured".to_string()))?;

        let query_pattern = SearchEngineType::BraveSearch
            .get_query_pattern_for_mode(super::super::types::SearchMode::ApiQuery);
        let search_url = query_pattern;
        info!("Brave API search: {}", search_url);

        let response = client
            .get(&search_url)
            .header("X-Subscription-Token", api_key)
            .query(&[("q", query), ("count", &limit.to_string())])
            .send()
            .await
            .map_err(|e| TarziError::Network(format!("Brave API request failed: {e}")))?;

        if !response.status().is_success() {
            return Err(TarziError::Network(format!(
                "Brave API returned error status: {}",
                response.status()
            )));
        }

        let data: Value = response
            .json()
            .await
            .map_err(|e| TarziError::Network(format!("Failed to parse Brave API response: {e}")))?;

        self.parse_brave_response(data)
    }

    fn get_provider_name(&self) -> &str {
        "Brave Search API"
    }

    fn is_healthy(&self) -> bool {
        self.api_key.is_some() && self.client.is_some()
    }
}

impl BraveSearchProvider {
    fn parse_brave_response(&self, data: Value) -> Result<Vec<SearchResult>> {
        let mut results = Vec::new();

        if let Some(web_results) = data
            .get("web")
            .and_then(|w| w.get("results"))
            .and_then(|r| r.as_array())
        {
            for (index, result) in web_results.iter().enumerate() {
                if let (Some(title), Some(url), Some(description)) = (
                    result.get("title").and_then(|t| t.as_str()),
                    result.get("url").and_then(|u| u.as_str()),
                    result.get("description").and_then(|d| d.as_str()),
                ) {
                    results.push(SearchResult {
                        title: title.to_string(),
                        url: url.to_string(),
                        snippet: description.to_string(),
                        rank: index + 1,
                    });
                }
            }
        }

        Ok(results)
    }
}
