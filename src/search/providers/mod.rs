use super::types::{SearchEngineType, SearchResult};
use crate::Result;
use crate::fetcher::WebFetcher;
use async_trait::async_trait;
use reqwest::Client;

/// Provider configuration for different search modes
#[derive(Debug)]
pub enum ProviderConfig {
    Web { fetcher: Box<WebFetcher> },
    Api { api_key: String, client: Client },
}

/// Unified interface for all search providers
#[async_trait]
pub trait SearchProvider: Send + Sync {
    /// Associated type for the provider's configuration
    type Config;

    /// Create a new provider instance with the given configuration
    fn new(config: Self::Config) -> Self;

    /// Perform a search using the provider
    async fn search(&mut self, query: &str, limit: usize) -> Result<Vec<SearchResult>>;

    /// Check if the provider is healthy/available
    fn is_healthy(&self) -> bool;

    /// Get the search engine type this provider represents
    fn get_engine_type(&self) -> SearchEngineType;

    /// Check if this provider requires an API key
    fn requires_api_key(&self) -> bool {
        false
    }

    /// Get the supported search modes for this provider
    fn supported_modes(&self) -> Vec<super::types::SearchMode> {
        vec![super::types::SearchMode::WebQuery]
    }
}

/// Provider variant enum for different search engines
#[derive(Debug)]
pub enum ProviderVariant {
    Google(GoogleSearchProvider),
    Bing(BingSearchProvider),
    DuckDuckGo(DuckDuckGoProvider),
    BraveSearch(BraveSearchProvider),
    Baidu(BaiduSearchProvider),
    Exa(ExaSearchProvider),
    Travily(TravilySearchProvider),
}

impl ProviderVariant {
    /// Create a provider variant from engine type and configuration
    pub fn from_engine_type(engine_type: SearchEngineType, config: ProviderConfig) -> Result<Self> {
        match engine_type {
            SearchEngineType::Google => {
                if let ProviderConfig::Web { fetcher } = config {
                    Ok(ProviderVariant::Google(GoogleSearchProvider::new_web(
                        *fetcher,
                    )))
                } else {
                    Err(crate::error::TarziError::Config(
                        "Google only supports web search mode".to_string(),
                    ))
                }
            }
            SearchEngineType::Bing => {
                if let ProviderConfig::Web { fetcher } = config {
                    Ok(ProviderVariant::Bing(BingSearchProvider::new_web(*fetcher)))
                } else {
                    Err(crate::error::TarziError::Config(
                        "Bing only supports web search mode".to_string(),
                    ))
                }
            }
            SearchEngineType::DuckDuckGo => match config {
                ProviderConfig::Web { fetcher } => Ok(ProviderVariant::DuckDuckGo(
                    DuckDuckGoProvider::new_web(*fetcher),
                )),
                ProviderConfig::Api { client, .. } => Ok(ProviderVariant::DuckDuckGo(
                    DuckDuckGoProvider::new_api(client),
                )),
            },
            SearchEngineType::BraveSearch => match config {
                ProviderConfig::Web { fetcher } => Ok(ProviderVariant::BraveSearch(
                    BraveSearchProvider::new_web(*fetcher),
                )),
                ProviderConfig::Api { api_key, client } => Ok(ProviderVariant::BraveSearch(
                    BraveSearchProvider::new_api(api_key, client),
                )),
            },
            SearchEngineType::Baidu => match config {
                ProviderConfig::Web { fetcher } => Ok(ProviderVariant::Baidu(
                    BaiduSearchProvider::new_web(*fetcher),
                )),
                ProviderConfig::Api { api_key, client } => Ok(ProviderVariant::Baidu(
                    BaiduSearchProvider::new_api(api_key, client),
                )),
            },
            SearchEngineType::Exa => match config {
                ProviderConfig::Web { fetcher } => {
                    Ok(ProviderVariant::Exa(ExaSearchProvider::new_web(*fetcher)))
                }
                ProviderConfig::Api { api_key, client } => Ok(ProviderVariant::Exa(
                    ExaSearchProvider::new_api(api_key, client),
                )),
            },
            SearchEngineType::Travily => {
                if let ProviderConfig::Api { api_key, client } = config {
                    Ok(ProviderVariant::Travily(TravilySearchProvider::new_api(
                        api_key, client,
                    )))
                } else {
                    Err(crate::error::TarziError::Config(
                        "Travily only supports API search mode".to_string(),
                    ))
                }
            }
        }
    }

    /// Get the engine type for this provider variant
    pub fn engine_type(&self) -> SearchEngineType {
        match self {
            ProviderVariant::Google(_) => SearchEngineType::Google,
            ProviderVariant::Bing(_) => SearchEngineType::Bing,
            ProviderVariant::DuckDuckGo(_) => SearchEngineType::DuckDuckGo,
            ProviderVariant::BraveSearch(_) => SearchEngineType::BraveSearch,
            ProviderVariant::Baidu(_) => SearchEngineType::Baidu,
            ProviderVariant::Exa(_) => SearchEngineType::Exa,
            ProviderVariant::Travily(_) => SearchEngineType::Travily,
        }
    }

    /// Perform a search using this provider variant
    pub async fn search(&mut self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        match self {
            ProviderVariant::Google(provider) => {
                SearchProvider::search(provider, query, limit).await
            }
            ProviderVariant::Bing(provider) => SearchProvider::search(provider, query, limit).await,
            ProviderVariant::DuckDuckGo(provider) => {
                SearchProvider::search(provider, query, limit).await
            }
            ProviderVariant::BraveSearch(provider) => {
                SearchProvider::search(provider, query, limit).await
            }
            ProviderVariant::Baidu(provider) => {
                SearchProvider::search(provider, query, limit).await
            }
            ProviderVariant::Exa(provider) => SearchProvider::search(provider, query, limit).await,
            ProviderVariant::Travily(provider) => {
                SearchProvider::search(provider, query, limit).await
            }
        }
    }

    /// Check if this provider variant is healthy
    pub fn is_healthy(&self) -> bool {
        match self {
            ProviderVariant::Google(provider) => SearchProvider::is_healthy(provider),
            ProviderVariant::Bing(provider) => SearchProvider::is_healthy(provider),
            ProviderVariant::DuckDuckGo(provider) => SearchProvider::is_healthy(provider),
            ProviderVariant::BraveSearch(provider) => SearchProvider::is_healthy(provider),
            ProviderVariant::Baidu(provider) => SearchProvider::is_healthy(provider),
            ProviderVariant::Exa(provider) => SearchProvider::is_healthy(provider),
            ProviderVariant::Travily(provider) => SearchProvider::is_healthy(provider),
        }
    }
}

// Re-export all provider implementations
pub mod baidu;
pub mod bing;
pub mod brave;
pub mod duckduckgo;
pub mod exa;
pub mod google;
pub mod travily;

// Re-export the traits and types
pub use baidu::BaiduSearchProvider;
pub use bing::BingSearchProvider;
pub use brave::BraveSearchProvider;
pub use duckduckgo::DuckDuckGoProvider;
pub use exa::ExaSearchProvider;
pub use google::GoogleSearchProvider;
pub use travily::TravilySearchProvider;
