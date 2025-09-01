use super::types::{SearchEngineType, SearchResult};
use crate::Result;
use crate::fetcher::WebFetcher;
use crate::search::parser::ParserFactory;
use async_trait::async_trait;

/// Provider configuration for web search only
#[derive(Debug)]
pub struct ProviderConfig {
    pub fetcher: Box<WebFetcher>,
}

/// Unified interface for all search providers
#[async_trait]
pub trait SearchProvider: Send + Sync {
    /// Associated type for the provider's configuration
    type Config;

    /// Create a new provider instance with the given configuration
    fn new(config: Self::Config) -> Self
    where
        Self: Sized;

    /// Perform a search using the provider
    async fn search(&mut self, query: &str, limit: usize) -> Result<Vec<SearchResult>>;

    /// Check if the provider is healthy/available
    fn is_healthy(&self) -> bool;

    /// Get the search engine type this provider represents
    fn get_engine_type(&self) -> SearchEngineType;
}

/// Macro to generate search provider implementations
macro_rules! impl_search_provider {
    ($provider_name:ident, $engine_type:expr) => {
        #[derive(Debug)]
        pub struct $provider_name {
            fetcher: WebFetcher,
        }

        impl $provider_name {
            pub fn new_web(fetcher: WebFetcher) -> Self {
                Self { fetcher }
            }
        }

        #[async_trait]
        impl SearchProvider for $provider_name {
            type Config = crate::fetcher::WebFetcher;

            fn new(config: Self::Config) -> Self {
                Self { fetcher: config }
            }

            async fn search(&mut self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
                let query_pattern = $engine_type.get_query_pattern();
                let search_url = query_pattern.replace("{query}", &urlencoding::encode(query));
                tracing::info!("{} web search: {}", stringify!($provider_name), search_url);

                let search_page_content = self
                    .fetcher
                    .fetch_raw(&search_url, crate::fetcher::FetchMode::BrowserHeadless)
                    .await?;

                // Use the parser to extract results
                let parser = ParserFactory::new().get_parser(&$engine_type);
                parser.parse(&search_page_content, limit)
            }

            fn is_healthy(&self) -> bool {
                true // Web provider is always available
            }

            fn get_engine_type(&self) -> SearchEngineType {
                $engine_type
            }
        }
    };
}

// Generate all provider implementations
impl_search_provider!(GoogleSearchProvider, SearchEngineType::Google);
impl_search_provider!(BingSearchProvider, SearchEngineType::Bing);
impl_search_provider!(DuckDuckGoProvider, SearchEngineType::DuckDuckGo);
impl_search_provider!(BraveSearchProvider, SearchEngineType::BraveSearch);
impl_search_provider!(BaiduSearchProvider, SearchEngineType::Baidu);

/// Provider variant enum for different search engines
#[derive(Debug)]
pub enum ProviderVariant {
    Google(GoogleSearchProvider),
    Bing(BingSearchProvider),
    DuckDuckGo(DuckDuckGoProvider),
    BraveSearch(BraveSearchProvider),
    Baidu(BaiduSearchProvider),
}

impl ProviderVariant {
    /// Create a provider variant from engine type and configuration
    pub fn from_engine_type(engine_type: SearchEngineType, config: ProviderConfig) -> Result<Self> {
        match engine_type {
            SearchEngineType::Google => Ok(ProviderVariant::Google(GoogleSearchProvider::new_web(
                *config.fetcher,
            ))),
            SearchEngineType::Bing => Ok(ProviderVariant::Bing(BingSearchProvider::new_web(
                *config.fetcher,
            ))),
            SearchEngineType::DuckDuckGo => Ok(ProviderVariant::DuckDuckGo(
                DuckDuckGoProvider::new_web(*config.fetcher),
            )),
            SearchEngineType::BraveSearch => Ok(ProviderVariant::BraveSearch(
                BraveSearchProvider::new_web(*config.fetcher),
            )),
            SearchEngineType::Baidu => Ok(ProviderVariant::Baidu(BaiduSearchProvider::new_web(
                *config.fetcher,
            ))),
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
        }
    }
}
