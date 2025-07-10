use super::api::{
    ApiSearchManager, AutoSwitchStrategy, BraveSearchProvider, DuckDuckGoProvider,
    ExaSearchProvider, GoogleSerperProvider, TravilySearchProvider,
};
use super::parser::{ParserFactory, SearchResultParser};
use super::types::{SearchEngineType, SearchMode, SearchResult};
use crate::config::Config;
use crate::{
    Result,
    error::TarziError,
    fetcher::{FetchMode, WebFetcher},
};
use reqwest::Client;
use std::str::FromStr;

use tracing::{error, info, warn};

pub struct SearchEngine {
    fetcher: WebFetcher,
    api_key: Option<String>,
    #[allow(dead_code)]
    engine_type: SearchEngineType,
    query_pattern: String,
    user_agent: String,
    parser_factory: ParserFactory,
    api_manager: ApiSearchManager,
}

impl SearchEngine {
    pub fn new() -> Self {
        info!("Initializing SearchEngine");
        Self {
            fetcher: WebFetcher::new(),
            api_key: None,
            engine_type: SearchEngineType::Bing,
            query_pattern: SearchEngineType::Bing.get_query_pattern(),
            user_agent: crate::constants::DEFAULT_USER_AGENT.to_string(),
            parser_factory: ParserFactory::new(),
            api_manager: ApiSearchManager::new(AutoSwitchStrategy::Smart),
        }
    }

    pub fn with_api_key(mut self, api_key: String) -> Self {
        info!("Setting API key for SearchEngine");
        self.api_key = Some(api_key);
        self
    }

    // Getter methods for testing
    pub fn api_key(&self) -> &Option<String> {
        &self.api_key
    }

    pub fn engine_type(&self) -> &SearchEngineType {
        &self.engine_type
    }

    pub fn query_pattern(&self) -> &str {
        &self.query_pattern
    }

    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }

    /// Register a custom parser for a specific engine
    pub fn register_custom_parser(&mut self, name: String, parser: Box<dyn SearchResultParser>) {
        info!("Registering custom parser: {}", name);
        self.parser_factory.register_custom_parser(name, parser);
    }

    pub fn from_config(config: &Config) -> Self {
        info!("Initializing SearchEngine from config");
        let fetcher = crate::fetcher::WebFetcher::from_config(config);

        // Parse the search engine type from config
        let engine_type =
            SearchEngineType::from_str(&config.search.engine).unwrap_or(SearchEngineType::Bing);

        // Use custom query pattern if provided, otherwise use the default for the engine type
        let query_pattern = if config.search.query_pattern != engine_type.get_query_pattern() {
            config.search.query_pattern.clone()
        } else {
            engine_type.get_query_pattern()
        };

        // Initialize API manager with autoswitch strategy
        let autoswitch_strategy = AutoSwitchStrategy::from(config.search.autoswitch.as_str());
        let mut api_manager = ApiSearchManager::new(autoswitch_strategy);

        // Check for proxy configuration
        let proxy_url = crate::config::get_proxy_from_env_or_config(&config.fetcher.proxy);

        // Initialize HTTP client for API providers (with or without proxy)
        let client = if let Some(ref proxy) = proxy_url {
            info!("Creating API client with proxy: {}", proxy);
            Client::builder()
                .timeout(std::time::Duration::from_secs(config.general.timeout))
                .user_agent(&config.fetcher.user_agent)
                .proxy(reqwest::Proxy::http(proxy).expect("Invalid proxy URL"))
                .build()
                .expect("Failed to create HTTP client with proxy for API providers")
        } else {
            Client::builder()
                .timeout(std::time::Duration::from_secs(config.general.timeout))
                .user_agent(&config.fetcher.user_agent)
                .build()
                .expect("Failed to create HTTP client for API providers")
        };

        // Register API providers based on available API keys
        if let Some(ref brave_key) = config.search.brave_api_key {
            info!(
                "Registering Brave Search API provider{}",
                if proxy_url.is_some() {
                    " with proxy"
                } else {
                    ""
                }
            );
            let provider = Box::new(BraveSearchProvider::new(brave_key.clone(), client.clone()));
            api_manager.register_provider(SearchEngineType::BraveSearch, provider);
        }

        if let Some(ref serper_key) = config.search.google_serper_api_key {
            info!(
                "Registering Google Serper API provider{}",
                if proxy_url.is_some() {
                    " with proxy"
                } else {
                    ""
                }
            );
            let provider = Box::new(GoogleSerperProvider::new(
                serper_key.clone(),
                client.clone(),
            ));
            api_manager.register_provider(SearchEngineType::GoogleSerper, provider);
        }

        if let Some(ref exa_key) = config.search.exa_api_key {
            info!(
                "Registering Exa Search API provider{}",
                if proxy_url.is_some() {
                    " with proxy"
                } else {
                    ""
                }
            );
            let provider = Box::new(ExaSearchProvider::new(exa_key.clone(), client.clone()));
            api_manager.register_provider(SearchEngineType::Exa, provider);
        }

        if let Some(ref travily_key) = config.search.travily_api_key {
            info!(
                "Registering Travily Search API provider{}",
                if proxy_url.is_some() {
                    " with proxy"
                } else {
                    ""
                }
            );
            let provider = Box::new(TravilySearchProvider::new(
                travily_key.clone(),
                client.clone(),
            ));
            api_manager.register_provider(SearchEngineType::Travily, provider);
        }

        // DuckDuckGo doesn't require an API key but has limited functionality
        info!("Registering DuckDuckGo API provider (limited functionality)");
        let provider = Box::new(DuckDuckGoProvider::new(client.clone()));
        api_manager.register_provider(SearchEngineType::DuckDuckGo, provider);

        Self {
            fetcher,
            api_key: None, // Deprecated: specific API keys are now managed per provider
            engine_type,
            query_pattern,
            user_agent: config.fetcher.user_agent.clone(),
            parser_factory: ParserFactory::new(),
            api_manager,
        }
    }

    pub async fn search(
        &mut self,
        query: &str,
        mode: SearchMode,
        limit: usize,
    ) -> Result<Vec<SearchResult>> {
        info!("Starting search with mode: {:?}, limit: {}", mode, limit);
        match mode {
            SearchMode::WebQuery => {
                info!("Using browser mode for search");
                self.search_browser(query, limit).await
            }
            SearchMode::ApiQuery => {
                info!("Using API mode for search");
                self.search_api(query, limit).await
            }
        }
    }

    async fn search_browser(&mut self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        info!("Starting browser-based search for query: '{}'", query);

        // Use the query pattern from config to build the search URL
        let search_url = self
            .query_pattern
            .replace("{query}", &urlencoding::encode(query));
        info!("Fetching search results from: {}", search_url);

        let search_page_content = self
            .fetcher
            .fetch_raw(&search_url, FetchMode::BrowserHeadless)
            .await?;
        info!(
            "Successfully fetched search page ({} characters)",
            search_page_content.len()
        );

        // Extract search results from the HTML content
        let results = self.extract_search_results_from_html(&search_page_content, limit)?;
        info!("Successfully extracted {} search results", results.len());

        Ok(results)
    }

    fn extract_search_results_from_html(
        &self,
        html: &str,
        limit: usize,
    ) -> Result<Vec<SearchResult>> {
        info!("Extracting search results from HTML content using parser factory");

        // Get the appropriate parser for the current engine type
        let parser = self.parser_factory.get_parser(&self.engine_type);

        info!(
            "Using parser: {} for engine type: {:?}",
            parser.name(),
            self.engine_type
        );

        // Use the parser to extract results
        let results = parser.parse(html, limit)?;

        info!(
            "Successfully extracted {} search results using {}",
            results.len(),
            parser.name()
        );
        Ok(results)
    }

    async fn search_api(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        info!("Starting API-based search for query: '{}'", query);

        // Use the API manager to perform the search with automatic provider switching
        let results = self
            .api_manager
            .search(&self.engine_type, query, limit)
            .await?;

        info!("API search completed, returning {} results", results.len());
        Ok(results)
    }

    /// Search and fetch content for each result
    pub async fn search_and_fetch(
        &mut self,
        query: &str,
        mode: SearchMode,
        limit: usize,
        fetch_mode: FetchMode,
        format: crate::converter::Format,
    ) -> Result<Vec<(SearchResult, String)>> {
        info!("Searching and fetching content for query: '{}'", query);

        // First, perform the search
        let search_results = self.search(query, mode, limit).await?;
        info!("Found {} search results", search_results.len());

        // Then, fetch content for each result
        let mut results_with_content = Vec::new();

        for result in search_results.clone() {
            info!("Fetching content for: {}", result.url);
            match self.fetcher.fetch(&result.url, fetch_mode, format).await {
                Ok(content) => {
                    info!(
                        "Successfully fetched content for {} ({} characters)",
                        result.url,
                        content.len()
                    );
                    results_with_content.push((result, content));
                }
                Err(e) => {
                    warn!("Failed to fetch content for {}: {}", result.url, e);
                    // Continue with other results even if one fails
                }
            }
        }

        info!(
            "Successfully fetched content for {}/{} results",
            results_with_content.len(),
            search_results.len()
        );
        Ok(results_with_content)
    }

    pub async fn search_with_proxy(
        &mut self,
        query: &str,
        mode: SearchMode,
        limit: usize,
        proxy: &str,
    ) -> Result<Vec<SearchResult>> {
        info!("Starting search with proxy: {}", proxy);

        // Use environment variables for proxy with fallback to provided proxy
        let effective_proxy = crate::config::get_proxy_from_env_or_config(&Some(proxy.to_string()))
            .unwrap_or_else(|| proxy.to_string());

        match mode {
            SearchMode::WebQuery => {
                warn!("Proxy support for browser mode is simplified");
                // For browser mode with proxy, we would need to configure the browser with proxy settings
                // This is a simplified implementation.
                // FIXME (xiaming.cxm): to be implemented.
                self.search_browser(query, limit).await
            }
            SearchMode::ApiQuery => {
                info!("Creating proxy-enabled HTTP client");
                // For API mode with proxy, we would use a proxy-enabled HTTP client
                let _proxy_client = Client::builder()
                    .timeout(crate::constants::DEFAULT_TIMEOUT)
                    .user_agent(&self.user_agent)
                    .proxy(reqwest::Proxy::http(&effective_proxy)?)
                    .build()
                    .map_err(|e| {
                        error!("Failed to create proxy client: {}", e);
                        TarziError::Config(format!("Failed to create proxy client: {e}"))
                    })?;

                info!(
                    "Proxy client created successfully with proxy: {}",
                    effective_proxy
                );
                // Use the proxy client for API calls
                // This is a simplified implementation
                // FIXME (xiaming.cxm): to be implemented.
                self.search_api(query, limit).await
            }
        }
    }

    /// Clean up resources
    pub async fn cleanup(&mut self) -> Result<()> {
        info!("Cleaning up SearchEngine resources");
        // The fetcher will handle its own cleanup
        Ok(())
    }
}

impl Default for SearchEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SearchEngine {
    fn drop(&mut self) {
        info!("Cleaning up SearchEngine resources");
        // The fetcher will handle its own cleanup
    }
}
