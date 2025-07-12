use super::super::types::{SearchEngineType, SearchResult};
use crate::fetcher::{FetchMode, WebFetcher};
use crate::{Result, error::TarziError};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;
use tracing::info;

#[derive(Debug)]
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
}

/// Configuration for BraveSearch provider
#[derive(Debug)]
pub enum BraveSearchConfig {
    Web { fetcher: Box<WebFetcher> },
    Api { api_key: String, client: Client },
}

#[async_trait]
impl super::SearchProvider for BraveSearchProvider {
    type Config = BraveSearchConfig;

    fn new(config: Self::Config) -> Self {
        match config {
            BraveSearchConfig::Web { fetcher } => Self {
                api_key: None,
                client: None,
                fetcher: *fetcher,
            },
            BraveSearchConfig::Api { api_key, client } => Self {
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

            let data: Value = response.json().await.map_err(|e| {
                TarziError::Network(format!("Failed to parse Brave API response: {e}"))
            })?;

            self.parse_brave_response(data)
        } else {
            // Web search
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
    }

    fn is_healthy(&self) -> bool {
        // Both API and web providers are always available
        true
    }

    fn get_engine_type(&self) -> SearchEngineType {
        SearchEngineType::BraveSearch
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
