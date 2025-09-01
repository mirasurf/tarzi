use crate::constants::{
    BAIDU_QUERY_PATTERN, BING_QUERY_PATTERN, BRAVE_QUERY_PATTERN, DUCKDUCKGO_QUERY_PATTERN,
    EXA_QUERY_PATTERN, GOOGLE_QUERY_PATTERN, SEARCH_ENGINE_BAIDU, SEARCH_ENGINE_BING,
    SEARCH_ENGINE_BRAVE, SEARCH_ENGINE_DUCKDUCKGO, SEARCH_ENGINE_EXA, SEARCH_ENGINE_GOOGLE,
};
use crate::error::TarziError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SearchEngineType {
    Bing,
    DuckDuckGo,
    Google,
    BraveSearch,
    Baidu,
    Exa,
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
            _ => Err(TarziError::InvalidEngine(s.to_string())),
        }
    }
}

impl SearchEngineType {
    pub fn get_query_pattern(&self) -> String {
        match self {
            SearchEngineType::Bing => BING_QUERY_PATTERN.to_string(),
            SearchEngineType::DuckDuckGo => DUCKDUCKGO_QUERY_PATTERN.to_string(),
            SearchEngineType::Google => GOOGLE_QUERY_PATTERN.to_string(),
            SearchEngineType::BraveSearch => BRAVE_QUERY_PATTERN.to_string(),
            SearchEngineType::Baidu => BAIDU_QUERY_PATTERN.to_string(),
            SearchEngineType::Exa => EXA_QUERY_PATTERN.to_string(),
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
