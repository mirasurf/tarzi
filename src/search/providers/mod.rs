use super::types::{SearchEngineType, SearchResult};
use crate::Result;
use async_trait::async_trait;

/// Interface for web-based search providers
#[async_trait]
pub trait WebSearchProvider: Send + Sync {
    /// Perform a web-based search using the provider's query pattern
    async fn search(&mut self, query: &str, limit: usize) -> Result<Vec<SearchResult>>;

    /// Get the provider name
    fn get_provider_name(&self) -> &str;

    /// Get the query pattern for this provider
    fn get_query_pattern(&self) -> &str;

    /// Check if the provider is healthy/available
    fn is_healthy(&self) -> bool;

    /// Get the search engine type this provider represents
    fn get_engine_type(&self) -> SearchEngineType;
}

/// Interface for API-based search providers
#[async_trait]
pub trait ApiSearchProvider: Send + Sync {
    /// Perform an API-based search
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>>;

    /// Get the provider name
    fn get_provider_name(&self) -> &str;

    /// Check if the provider is healthy/available
    fn is_healthy(&self) -> bool;

    /// Get the search engine type this provider represents
    fn get_engine_type(&self) -> SearchEngineType;

    /// Check if this provider requires an API key
    fn requires_api_key(&self) -> bool;
}

// Re-export all provider implementations
pub mod baidu;
pub mod bing;
pub mod brave;
pub mod duckduckgo;
pub mod exa;
pub mod google;
pub mod travily;

// Re-export the traits
pub use baidu::BaiduSearchProvider;
pub use bing::BingSearchProvider;
pub use brave::BraveSearchProvider;
pub use duckduckgo::DuckDuckGoProvider;
pub use exa::ExaSearchProvider;
pub use google::GoogleSearchProvider;
pub use travily::TravilySearchProvider;
