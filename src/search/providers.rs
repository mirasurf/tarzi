use super::types::{SearchEngineType, SearchResult};
use crate::fetcher::WebFetcher;
use crate::search::parser::ParserFactory;
use crate::Result;
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
                    .fetch_url(&search_url, crate::fetcher::FetchMode::BrowserHeadless)
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
impl_search_provider!(SougouWeixinProvider, SearchEngineType::SougouWeixin);

/// Provider variant enum for different search engines
#[derive(Debug)]
pub enum ProviderVariant {
    Google(GoogleSearchProvider),
    Bing(BingSearchProvider),
    DuckDuckGo(DuckDuckGoProvider),
    BraveSearch(BraveSearchProvider),
    Baidu(BaiduSearchProvider),
    SougouWeixin(SougouWeixinProvider),
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
            SearchEngineType::SougouWeixin => Ok(ProviderVariant::SougouWeixin(
                SougouWeixinProvider::new_web(*config.fetcher),
            )),
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
            ProviderVariant::SougouWeixin(_) => SearchEngineType::SougouWeixin,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fetcher::WebFetcher;

    #[test]
    fn test_google_search_provider() {
        let fetcher = WebFetcher::new();
        let provider = GoogleSearchProvider::new_web(fetcher);

        assert_eq!(provider.get_engine_type(), SearchEngineType::Google);
        assert!(provider.is_healthy());
    }

    #[test]
    fn test_bing_search_provider() {
        let fetcher = WebFetcher::new();
        let provider = BingSearchProvider::new_web(fetcher);

        assert_eq!(provider.get_engine_type(), SearchEngineType::Bing);
        assert!(provider.is_healthy());
    }

    #[test]
    fn test_duckduckgo_provider() {
        let fetcher = WebFetcher::new();
        let provider = DuckDuckGoProvider::new_web(fetcher);

        assert_eq!(provider.get_engine_type(), SearchEngineType::DuckDuckGo);
        assert!(provider.is_healthy());
    }

    #[test]
    fn test_brave_search_provider() {
        let fetcher = WebFetcher::new();
        let provider = BraveSearchProvider::new_web(fetcher);

        assert_eq!(provider.get_engine_type(), SearchEngineType::BraveSearch);
        assert!(provider.is_healthy());
    }

    #[test]
    fn test_baidu_search_provider() {
        let fetcher = WebFetcher::new();
        let provider = BaiduSearchProvider::new_web(fetcher);

        assert_eq!(provider.get_engine_type(), SearchEngineType::Baidu);
        assert!(provider.is_healthy());
    }

    #[test]
    fn test_provider_variant_from_engine_type() {
        let fetcher = WebFetcher::new();
        let config = ProviderConfig {
            fetcher: Box::new(fetcher),
        };

        // Test Google provider creation
        let google_variant =
            ProviderVariant::from_engine_type(SearchEngineType::Google, config).unwrap();
        assert_eq!(google_variant.engine_type(), SearchEngineType::Google);

        // Test other engine types
        let fetcher = WebFetcher::new();
        let config = ProviderConfig {
            fetcher: Box::new(fetcher),
        };
        let bing_variant =
            ProviderVariant::from_engine_type(SearchEngineType::Bing, config).unwrap();
        assert_eq!(bing_variant.engine_type(), SearchEngineType::Bing);

        let fetcher = WebFetcher::new();
        let config = ProviderConfig {
            fetcher: Box::new(fetcher),
        };
        let duckduckgo_variant =
            ProviderVariant::from_engine_type(SearchEngineType::DuckDuckGo, config).unwrap();
        assert_eq!(
            duckduckgo_variant.engine_type(),
            SearchEngineType::DuckDuckGo
        );

        let fetcher = WebFetcher::new();
        let config = ProviderConfig {
            fetcher: Box::new(fetcher),
        };
        let brave_variant =
            ProviderVariant::from_engine_type(SearchEngineType::BraveSearch, config).unwrap();
        assert_eq!(brave_variant.engine_type(), SearchEngineType::BraveSearch);

        let fetcher = WebFetcher::new();
        let config = ProviderConfig {
            fetcher: Box::new(fetcher),
        };
        let baidu_variant =
            ProviderVariant::from_engine_type(SearchEngineType::Baidu, config).unwrap();
        assert_eq!(baidu_variant.engine_type(), SearchEngineType::Baidu);
    }

    #[test]
    fn test_provider_variant_engine_type_matching() {
        // Test that ProviderVariant correctly returns engine types
        let google_provider =
            ProviderVariant::Google(GoogleSearchProvider::new_web(WebFetcher::new()));
        assert_eq!(google_provider.engine_type(), SearchEngineType::Google);

        let bing_provider = ProviderVariant::Bing(BingSearchProvider::new_web(WebFetcher::new()));
        assert_eq!(bing_provider.engine_type(), SearchEngineType::Bing);

        let duckduckgo_provider =
            ProviderVariant::DuckDuckGo(DuckDuckGoProvider::new_web(WebFetcher::new()));
        assert_eq!(
            duckduckgo_provider.engine_type(),
            SearchEngineType::DuckDuckGo
        );

        let brave_provider =
            ProviderVariant::BraveSearch(BraveSearchProvider::new_web(WebFetcher::new()));
        assert_eq!(brave_provider.engine_type(), SearchEngineType::BraveSearch);

        let baidu_provider =
            ProviderVariant::Baidu(BaiduSearchProvider::new_web(WebFetcher::new()));
        assert_eq!(baidu_provider.engine_type(), SearchEngineType::Baidu);
    }

    #[test]
    fn test_provider_config_creation() {
        let fetcher = WebFetcher::new();
        let config = ProviderConfig {
            fetcher: Box::new(fetcher),
        };

        // Test that config can be created and used
        let _variant = ProviderVariant::from_engine_type(SearchEngineType::Google, config);
        // This test passes if no panic occurs during creation
    }

    #[test]
    fn test_all_engine_types_supported() {
        let engine_types = vec![
            SearchEngineType::Google,
            SearchEngineType::Bing,
            SearchEngineType::DuckDuckGo,
            SearchEngineType::BraveSearch,
            SearchEngineType::Baidu,
        ];

        for engine_type in engine_types {
            let config = ProviderConfig {
                fetcher: Box::new(WebFetcher::new()),
            };
            let variant = ProviderVariant::from_engine_type(engine_type, config);
            assert!(
                variant.is_ok(),
                "Engine type {engine_type:?} should be supported"
            );

            if let Ok(provider) = variant {
                assert_eq!(provider.engine_type(), engine_type);
            }
        }
    }

    #[test]
    fn test_search_provider_trait_functionality() {
        let fetcher = WebFetcher::new();

        // Test SearchProvider trait through concrete implementation
        let google_provider = GoogleSearchProvider::new(fetcher);
        assert_eq!(google_provider.get_engine_type(), SearchEngineType::Google);
        assert!(google_provider.is_healthy());

        // Test that provider can be created with SearchProvider::new
        let fetcher2 = WebFetcher::new();
        let bing_provider = BingSearchProvider::new(fetcher2);
        assert_eq!(bing_provider.get_engine_type(), SearchEngineType::Bing);
        assert!(bing_provider.is_healthy());
    }
}
