use crate::error::TarziError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SearchMode {
    WebQuery,
    ApiQuery,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SearchEngineType {
    Bing,
    DuckDuckGo,
    Google,
    BraveSearch,
    Baidu,
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
            _ => Ok(SearchEngineType::Custom(s.to_string())),
        }
    }
}

impl SearchEngineType {
    pub fn get_query_pattern(&self) -> String {
        match self {
            SearchEngineType::Bing => "https://www.bing.com/search?q={query}".to_string(),
            SearchEngineType::DuckDuckGo => "https://duckduckgo.com/?q={query}".to_string(),
            SearchEngineType::Google => "https://www.google.com/search?q={query}".to_string(),
            SearchEngineType::BraveSearch => {
                "https://search.brave.com/search?q={query}".to_string()
            }
            SearchEngineType::Baidu => "https://www.baidu.com/s?wd={query}".to_string(),
            SearchEngineType::Custom(_) => "{query}".to_string(), // Default pattern for custom engines
        }
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
