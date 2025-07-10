use super::types::{SearchEngineType, SearchResult};
use crate::{Result, error::TarziError};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::{Value, json};
use std::collections::HashMap;
use tracing::{info, warn};

#[async_trait]
pub trait SearchApiProvider: Send + Sync {
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>>;
    fn get_provider_name(&self) -> &str;
    fn is_healthy(&self) -> bool;
}

pub struct ApiSearchManager {
    providers: HashMap<SearchEngineType, Box<dyn SearchApiProvider>>,
    autoswitch_strategy: AutoSwitchStrategy,
    fallback_order: Vec<SearchEngineType>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AutoSwitchStrategy {
    Smart,
    None,
}

impl From<&str> for AutoSwitchStrategy {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "smart" => AutoSwitchStrategy::Smart,
            "none" => AutoSwitchStrategy::None,
            _ => AutoSwitchStrategy::Smart,
        }
    }
}

impl ApiSearchManager {
    pub fn new(autoswitch_strategy: AutoSwitchStrategy) -> Self {
        let fallback_order = vec![
            SearchEngineType::GoogleSerper,
            SearchEngineType::BraveSearch,
            SearchEngineType::Exa,
            SearchEngineType::Travily,
            SearchEngineType::DuckDuckGo,
        ];

        Self {
            providers: HashMap::new(),
            autoswitch_strategy,
            fallback_order,
        }
    }

    pub fn register_provider(
        &mut self,
        engine_type: SearchEngineType,
        provider: Box<dyn SearchApiProvider>,
    ) {
        info!("Registering API provider for {:?}", engine_type);
        self.providers.insert(engine_type, provider);
    }

    pub async fn search(
        &self,
        engine_type: &SearchEngineType,
        query: &str,
        limit: usize,
    ) -> Result<Vec<SearchResult>> {
        match self.autoswitch_strategy {
            AutoSwitchStrategy::None => self.search_with_provider(engine_type, query, limit).await,
            AutoSwitchStrategy::Smart => {
                self.search_with_smart_fallback(engine_type, query, limit)
                    .await
            }
        }
    }

    async fn search_with_provider(
        &self,
        engine_type: &SearchEngineType,
        query: &str,
        limit: usize,
    ) -> Result<Vec<SearchResult>> {
        if let Some(provider) = self.providers.get(engine_type) {
            info!(
                "Using provider: {} for search",
                provider.get_provider_name()
            );
            provider.search(query, limit).await
        } else {
            Err(TarziError::Config(format!(
                "No provider registered for {engine_type:?}"
            )))
        }
    }

    async fn search_with_smart_fallback(
        &self,
        primary_engine: &SearchEngineType,
        query: &str,
        limit: usize,
    ) -> Result<Vec<SearchResult>> {
        // Try the primary engine first
        if let Ok(results) = self
            .search_with_provider(primary_engine, query, limit)
            .await
        {
            return Ok(results);
        }

        warn!(
            "Primary engine {:?} failed, attempting fallback",
            primary_engine
        );

        // Try fallback providers in order
        for engine_type in &self.fallback_order {
            if engine_type == primary_engine {
                continue; // Skip the primary engine we already tried
            }

            if self.providers.contains_key(engine_type) {
                info!("Trying fallback provider: {:?}", engine_type);
                match self.search_with_provider(engine_type, query, limit).await {
                    Ok(results) => {
                        info!("Fallback provider {:?} succeeded", engine_type);
                        return Ok(results);
                    }
                    Err(e) => {
                        warn!("Fallback provider {:?} failed: {}", engine_type, e);
                        continue;
                    }
                }
            }
        }

        Err(TarziError::Config(
            "All search providers failed".to_string(),
        ))
    }
}

// Brave Search API Provider
pub struct BraveSearchProvider {
    api_key: String,
    client: Client,
}

impl BraveSearchProvider {
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
impl SearchApiProvider for BraveSearchProvider {
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let url = "https://api.search.brave.com/res/v1/web/search";
        let params = json!({
            "q": query,
            "count": limit,
        });

        let response = self
            .client
            .get(url)
            .header("X-Subscription-Token", &self.api_key)
            .query(&params)
            .send()
            .await
            .map_err(|e| TarziError::Network(format!("Brave API request failed: {e}")))?;

        if !response.status().is_success() {
            return Err(TarziError::Network(format!(
                "Brave API returned status: {}",
                response.status()
            )));
        }

        let data: Value = response
            .json()
            .await
            .map_err(|e| TarziError::Parse(format!("Failed to parse Brave API response: {e}")))?;

        self.parse_brave_response(data)
    }

    fn get_provider_name(&self) -> &str {
        "Brave Search API"
    }

    fn is_healthy(&self) -> bool {
        !self.api_key.is_empty()
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

// Google Serper API Provider
pub struct GoogleSerperProvider {
    api_key: String,
    client: Client,
}

impl GoogleSerperProvider {
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
impl SearchApiProvider for GoogleSerperProvider {
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let url = "https://google.serper.dev/search";
        let payload = json!({
            "q": query,
            "num": limit,
        });

        let response = self
            .client
            .post(url)
            .header("X-API-KEY", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| TarziError::Network(format!("Google Serper API request failed: {e}")))?;

        if !response.status().is_success() {
            return Err(TarziError::Network(format!(
                "Google Serper API returned status: {}",
                response.status()
            )));
        }

        let data: Value = response.json().await.map_err(|e| {
            TarziError::Parse(format!("Failed to parse Google Serper API response: {e}"))
        })?;

        self.parse_serper_response(data)
    }

    fn get_provider_name(&self) -> &str {
        "Google Serper API"
    }

    fn is_healthy(&self) -> bool {
        !self.api_key.is_empty()
    }
}

impl GoogleSerperProvider {
    fn parse_serper_response(&self, data: Value) -> Result<Vec<SearchResult>> {
        let mut results = Vec::new();

        if let Some(organic_results) = data.get("organic").and_then(|o| o.as_array()) {
            for (index, result) in organic_results.iter().enumerate() {
                if let (Some(title), Some(link), Some(snippet)) = (
                    result.get("title").and_then(|t| t.as_str()),
                    result.get("link").and_then(|l| l.as_str()),
                    result.get("snippet").and_then(|s| s.as_str()),
                ) {
                    results.push(SearchResult {
                        title: title.to_string(),
                        url: link.to_string(),
                        snippet: snippet.to_string(),
                        rank: index + 1,
                    });
                }
            }
        }

        Ok(results)
    }
}

// Exa Search API Provider
pub struct ExaSearchProvider {
    api_key: String,
    client: Client,
}

impl ExaSearchProvider {
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
impl SearchApiProvider for ExaSearchProvider {
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let url = "https://api.exa.ai/search";
        let payload = json!({
            "query": query,
            "numResults": limit,
            "includeDomains": [],
            "excludeDomains": [],
            "useAutoprompt": true,
        });

        let response = self
            .client
            .post(url)
            .header("x-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| TarziError::Network(format!("Exa API request failed: {e}")))?;

        if !response.status().is_success() {
            return Err(TarziError::Network(format!(
                "Exa API returned status: {}",
                response.status()
            )));
        }

        let data: Value = response
            .json()
            .await
            .map_err(|e| TarziError::Parse(format!("Failed to parse Exa API response: {e}")))?;

        self.parse_exa_response(data)
    }

    fn get_provider_name(&self) -> &str {
        "Exa Search API"
    }

    fn is_healthy(&self) -> bool {
        !self.api_key.is_empty()
    }
}

impl ExaSearchProvider {
    fn parse_exa_response(&self, data: Value) -> Result<Vec<SearchResult>> {
        let mut results = Vec::new();

        if let Some(exa_results) = data.get("results").and_then(|r| r.as_array()) {
            for (index, result) in exa_results.iter().enumerate() {
                if let (Some(title), Some(url)) = (
                    result.get("title").and_then(|t| t.as_str()),
                    result.get("url").and_then(|u| u.as_str()),
                ) {
                    let snippet = result
                        .get("text")
                        .and_then(|t| t.as_str())
                        .unwrap_or("")
                        .chars()
                        .take(200)
                        .collect::<String>();

                    results.push(SearchResult {
                        title: title.to_string(),
                        url: url.to_string(),
                        snippet,
                        rank: index + 1,
                    });
                }
            }
        }

        Ok(results)
    }
}

// Travily API Provider
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
impl SearchApiProvider for TravilySearchProvider {
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let url = "https://api.tavily.com/search";
        let payload = json!({
            "api_key": self.api_key,
            "query": query,
            "search_depth": "basic",
            "include_answer": false,
            "include_images": false,
            "include_raw_content": false,
            "max_results": limit,
        });

        let response = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| TarziError::Network(format!("Travily API request failed: {e}")))?;

        if !response.status().is_success() {
            return Err(TarziError::Network(format!(
                "Travily API returned status: {}",
                response.status()
            )));
        }

        let data: Value = response.json().await.map_err(|e| {
            TarziError::Parse(format!("Failed to parse Travily API response: {e}"))
        })?;

        self.parse_travily_response(data)
    }

    fn get_provider_name(&self) -> &str {
        "Travily Search API"
    }

    fn is_healthy(&self) -> bool {
        !self.api_key.is_empty()
    }
}

impl TravilySearchProvider {
    fn parse_travily_response(&self, data: Value) -> Result<Vec<SearchResult>> {
        let mut results = Vec::new();

        if let Some(travily_results) = data.get("results").and_then(|r| r.as_array()) {
            for (index, result) in travily_results.iter().enumerate() {
                if let (Some(title), Some(url), Some(content)) = (
                    result.get("title").and_then(|t| t.as_str()),
                    result.get("url").and_then(|u| u.as_str()),
                    result.get("content").and_then(|c| c.as_str()),
                ) {
                    results.push(SearchResult {
                        title: title.to_string(),
                        url: url.to_string(),
                        snippet: content.chars().take(200).collect::<String>(),
                        rank: index + 1,
                    });
                }
            }
        }

        Ok(results)
    }
}

// DuckDuckGo API Provider (Note: DuckDuckGo doesn't have an official API, this is a placeholder)
pub struct DuckDuckGoProvider;

impl DuckDuckGoProvider {
    pub fn new(_client: Client) -> Self {
        Self
    }

    pub fn new_with_proxy(_proxy_url: &str) -> Result<Self> {
        // DuckDuckGo provider doesn't use HTTP client currently
        // but we provide the method for consistency
        Ok(Self)
    }
}

#[async_trait]
impl SearchApiProvider for DuckDuckGoProvider {
    async fn search(&self, _query: &str, _limit: usize) -> Result<Vec<SearchResult>> {
        // Note: DuckDuckGo doesn't have an official search API
        // This is a placeholder implementation that could use their instant answer API
        // or scrape results (which would require additional implementation)

        warn!("DuckDuckGo API provider is not fully implemented (no official API available)");

        // For now, return empty results or implement web scraping
        Ok(vec![])
    }

    fn get_provider_name(&self) -> &str {
        "DuckDuckGo API (Limited)"
    }

    fn is_healthy(&self) -> bool {
        true // No API key required, but functionality is limited
    }
}
