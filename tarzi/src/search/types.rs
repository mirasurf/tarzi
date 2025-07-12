use crate::constants::{
    BAIDU_API_KEY_FIELD, BAIDU_API_PATTERN, BAIDU_QUERY_PATTERN, BING_QUERY_PATTERN,
    BRAVE_API_KEY_FIELD, BRAVE_API_PATTERN, BRAVE_QUERY_PATTERN, DUCKDUCKGO_API_PATTERN,
    DUCKDUCKGO_QUERY_PATTERN, EMPTY_PATTERN, EXA_API_KEY_FIELD, EXA_API_PATTERN, EXA_QUERY_PATTERN,
    GOOGLE_QUERY_PATTERN, SEARCH_ENGINE_BAIDU, SEARCH_ENGINE_BING, SEARCH_ENGINE_BRAVE,
    SEARCH_ENGINE_DUCKDUCKGO, SEARCH_ENGINE_EXA, SEARCH_ENGINE_GOOGLE, SEARCH_ENGINE_TRAVILY,
    SEARCH_MODE_APIQUERY, SEARCH_MODE_WEBQUERY, TRAVILY_API_KEY_FIELD, TRAVILY_API_PATTERN,
};
use crate::error::TarziError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SearchMode {
    WebQuery,
    ApiQuery,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SearchEngineType {
    Bing,
    DuckDuckGo,
    Google,
    BraveSearch,
    Baidu,
    Exa,
    Travily,
}

impl FromStr for SearchEngineType {
    type Err = TarziError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            SEARCH_ENGINE_BING => Ok(SearchEngineType::Bing),
            SEARCH_ENGINE_DUCKDUCKGO => Ok(SearchEngineType::DuckDuckGo),
            SEARCH_ENGINE_GOOGLE => Ok(SearchEngineType::Google),
            SEARCH_ENGINE_BRAVE => Ok(SearchEngineType::BraveSearch),
            SEARCH_ENGINE_BAIDU => Ok(SearchEngineType::Baidu),
            SEARCH_ENGINE_EXA => Ok(SearchEngineType::Exa),
            SEARCH_ENGINE_TRAVILY => Ok(SearchEngineType::Travily),
            _ => Err(TarziError::InvalidEngine(s.to_string())),
        }
    }
}

impl SearchEngineType {
    pub fn get_query_pattern_for_mode(&self, mode: SearchMode) -> String {
        match (self, mode) {
            (SearchEngineType::Bing, SearchMode::WebQuery) => BING_QUERY_PATTERN.to_string(),
            (SearchEngineType::Bing, SearchMode::ApiQuery) => EMPTY_PATTERN.to_string(), // No API
            (SearchEngineType::DuckDuckGo, SearchMode::WebQuery) => {
                DUCKDUCKGO_QUERY_PATTERN.to_string()
            }
            (SearchEngineType::DuckDuckGo, SearchMode::ApiQuery) => {
                DUCKDUCKGO_API_PATTERN.to_string()
            }
            (SearchEngineType::Google, SearchMode::WebQuery) => GOOGLE_QUERY_PATTERN.to_string(),
            (SearchEngineType::Google, SearchMode::ApiQuery) => {
                EMPTY_PATTERN.to_string() // No API for Google (Serper support removed)
            }
            (SearchEngineType::BraveSearch, SearchMode::WebQuery) => {
                BRAVE_QUERY_PATTERN.to_string()
            }
            (SearchEngineType::BraveSearch, SearchMode::ApiQuery) => BRAVE_API_PATTERN.to_string(),
            (SearchEngineType::Baidu, SearchMode::WebQuery) => BAIDU_QUERY_PATTERN.to_string(),
            (SearchEngineType::Baidu, SearchMode::ApiQuery) => BAIDU_API_PATTERN.to_string(),
            (SearchEngineType::Exa, SearchMode::WebQuery) => EXA_QUERY_PATTERN.to_string(),
            (SearchEngineType::Exa, SearchMode::ApiQuery) => EXA_API_PATTERN.to_string(),
            (SearchEngineType::Travily, SearchMode::WebQuery) => EMPTY_PATTERN.to_string(), // No webquery
            (SearchEngineType::Travily, SearchMode::ApiQuery) => TRAVILY_API_PATTERN.to_string(),
        }
    }

    pub fn get_query_pattern(&self) -> String {
        self.get_query_pattern_for_mode(SearchMode::WebQuery)
    }

    /// Check if this engine supports web query mode
    pub fn supports_web_query(&self) -> bool {
        match self {
            SearchEngineType::Bing => true,
            SearchEngineType::DuckDuckGo => true,
            SearchEngineType::Google => true,
            SearchEngineType::BraveSearch => true,
            SearchEngineType::Baidu => true,
            SearchEngineType::Exa => true,
            SearchEngineType::Travily => false, // Travily only supports API
        }
    }

    /// Check if this engine supports API query mode
    pub fn supports_api_query(&self) -> bool {
        match self {
            SearchEngineType::Bing => false, // Bing doesn't have a public API
            SearchEngineType::DuckDuckGo => true,
            SearchEngineType::Google => true,
            SearchEngineType::BraveSearch => true,
            SearchEngineType::Baidu => true,
            SearchEngineType::Exa => true,
            SearchEngineType::Travily => true,
        }
    }

    /// Check if this engine requires an API key for API query mode
    pub fn requires_api_key(&self) -> bool {
        match self {
            SearchEngineType::Bing => false,       // No API support
            SearchEngineType::DuckDuckGo => false, // No API key required
            SearchEngineType::Google => true,
            SearchEngineType::BraveSearch => true,
            SearchEngineType::Baidu => true,
            SearchEngineType::Exa => true,
            SearchEngineType::Travily => true,
        }
    }

    /// Get the API key field name for this engine type
    pub fn get_api_key_field(&self) -> Option<&'static str> {
        match self {
            SearchEngineType::Bing => None,
            SearchEngineType::DuckDuckGo => None,
            SearchEngineType::Google => None,
            SearchEngineType::BraveSearch => Some(BRAVE_API_KEY_FIELD),
            SearchEngineType::Baidu => Some(BAIDU_API_KEY_FIELD),
            SearchEngineType::Exa => Some(EXA_API_KEY_FIELD),
            SearchEngineType::Travily => Some(TRAVILY_API_KEY_FIELD),
        }
    }

    pub fn is_api_based(&self) -> bool {
        matches!(self, SearchEngineType::Exa | SearchEngineType::Travily)
    }
}

impl FromStr for SearchMode {
    type Err = TarziError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            SEARCH_MODE_WEBQUERY => Ok(SearchMode::WebQuery),
            SEARCH_MODE_APIQUERY => Ok(SearchMode::ApiQuery),
            _ => Err(TarziError::InvalidMode(s.to_string())),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
    pub rank: usize,
}
