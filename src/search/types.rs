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
    GoogleSerper,
    Custom(String),
}

impl FromStr for SearchEngineType {
    type Err = TarziError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "bing" => Ok(SearchEngineType::Bing),
            "duckduckgo" => Ok(SearchEngineType::DuckDuckGo),
            "google" => Ok(SearchEngineType::Google),
            "brave" => Ok(SearchEngineType::BraveSearch),
            "baidu" => Ok(SearchEngineType::Baidu),
            "exa" => Ok(SearchEngineType::Exa),
            "travily" => Ok(SearchEngineType::Travily),
            "google_serper" => Ok(SearchEngineType::GoogleSerper),
            _ => Ok(SearchEngineType::Custom(s.to_string())),
        }
    }
}

impl SearchEngineType {
    pub fn get_query_pattern_for_mode(&self, mode: SearchMode) -> String {
        match (self, mode) {
            (SearchEngineType::Bing, SearchMode::WebQuery) => {
                "https://www.bing.com/search?q={query}".to_string()
            }
            (SearchEngineType::Bing, SearchMode::ApiQuery) => "".to_string(), // No API
            (SearchEngineType::DuckDuckGo, SearchMode::WebQuery) => {
                "https://duckduckgo.com/?q={query}".to_string()
            }
            (SearchEngineType::DuckDuckGo, SearchMode::ApiQuery) => {
                "https://api.duckduckgo.com/?q={query}&format=json".to_string()
            }
            (SearchEngineType::Google, SearchMode::WebQuery) => {
                "https://www.google.com/search?q={query}".to_string()
            }
            (SearchEngineType::Google, SearchMode::ApiQuery) => {
                "https://google.serper.dev/search".to_string()
            } // Use Serper for Google API
            (SearchEngineType::BraveSearch, SearchMode::WebQuery) => {
                "https://search.brave.com/search?q={query}".to_string()
            }
            (SearchEngineType::BraveSearch, SearchMode::ApiQuery) => {
                "https://api.search.brave.com/res/v1/web/search".to_string()
            }
            (SearchEngineType::Baidu, SearchMode::WebQuery) => {
                "https://www.baidu.com/s?wd={query}".to_string()
            }
            (SearchEngineType::Baidu, SearchMode::ApiQuery) => {
                "https://api.baidu.com/search".to_string()
            }
            (SearchEngineType::Exa, SearchMode::WebQuery) => {
                "https://exa.ai/search?q={query}".to_string()
            }
            (SearchEngineType::Exa, SearchMode::ApiQuery) => {
                "https://api.exa.ai/search".to_string()
            }
            (SearchEngineType::Travily, SearchMode::WebQuery) => "".to_string(), // No webquery
            (SearchEngineType::Travily, SearchMode::ApiQuery) => {
                "https://api.tavily.com/search".to_string()
            }
            (SearchEngineType::GoogleSerper, SearchMode::WebQuery) => {
                "https://www.google.com/search?q={query}".to_string()
            }
            (SearchEngineType::GoogleSerper, SearchMode::ApiQuery) => {
                "https://google.serper.dev/search".to_string()
            }
            (SearchEngineType::Custom(_), _) => "{query}".to_string(),
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
            SearchEngineType::GoogleSerper => true,
            SearchEngineType::Travily => false, // Travily only supports API
            SearchEngineType::Custom(_) => true, // Custom engines assumed to support web query
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
            SearchEngineType::GoogleSerper => true,
            SearchEngineType::Custom(_) => true, // Custom engines assumed to support API
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
            SearchEngineType::GoogleSerper => true,
            SearchEngineType::Custom(_) => true, // Custom engines assumed to require API key
        }
    }

    /// Get the API key field name for this engine type
    pub fn get_api_key_field(&self) -> Option<&'static str> {
        match self {
            SearchEngineType::Bing => None,
            SearchEngineType::DuckDuckGo => None,
            SearchEngineType::Google => Some("google_serper_api_key"), // Use Serper for Google API
            SearchEngineType::BraveSearch => Some("brave_api_key"),
            SearchEngineType::Baidu => Some("baidu_api_key"), // Note: baidu_api_key not in config yet
            SearchEngineType::Exa => Some("exa_api_key"),
            SearchEngineType::Travily => Some("travily_api_key"),
            SearchEngineType::GoogleSerper => Some("google_serper_api_key"),
            SearchEngineType::Custom(_) => None, // Custom engines don't have predefined API key fields
        }
    }

    pub fn is_api_based(&self) -> bool {
        matches!(
            self,
            SearchEngineType::Exa | SearchEngineType::Travily | SearchEngineType::GoogleSerper
        )
    }
}

impl FromStr for SearchMode {
    type Err = TarziError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "webquery" => Ok(SearchMode::WebQuery),
            "apiquery" => Ok(SearchMode::ApiQuery),
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
