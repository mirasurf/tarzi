use crate::constants::{
    BAIDU_QUERY_PATTERN, BING_QUERY_PATTERN, BRAVE_QUERY_PATTERN, DUCKDUCKGO_QUERY_PATTERN,
    GOOGLE_QUERY_PATTERN, SEARCH_ENGINE_BAIDU, SEARCH_ENGINE_BING, SEARCH_ENGINE_BRAVE,
    SEARCH_ENGINE_DUCKDUCKGO, SEARCH_ENGINE_GOOGLE, SEARCH_ENGINE_SOUGOU_WEIXIN,
    SOUGOU_WEIXIN_QUERY_PATTERN,
};
use crate::error::TarziError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SearchEngineType {
    Bing,
    DuckDuckGo,
    Google,
    BraveSearch,
    Baidu,
    SougouWeixin,
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
            SEARCH_ENGINE_SOUGOU_WEIXIN => Ok(SearchEngineType::SougouWeixin),
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
            SearchEngineType::SougouWeixin => SOUGOU_WEIXIN_QUERY_PATTERN.to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{
        BAIDU_QUERY_PATTERN, BING_QUERY_PATTERN, BRAVE_QUERY_PATTERN, DUCKDUCKGO_QUERY_PATTERN,
        GOOGLE_QUERY_PATTERN, SEARCH_ENGINE_BAIDU, SEARCH_ENGINE_BING, SEARCH_ENGINE_BRAVE,
        SEARCH_ENGINE_DUCKDUCKGO, SEARCH_ENGINE_GOOGLE, SEARCH_ENGINE_SOUGOU_WEIXIN,
    };

    #[test]
    fn test_search_engine_type_parsing() {
        // Test valid engine types
        assert_eq!(
            SearchEngineType::from_str(SEARCH_ENGINE_DUCKDUCKGO).unwrap(),
            SearchEngineType::DuckDuckGo
        );
        assert_eq!(
            SearchEngineType::from_str(SEARCH_ENGINE_GOOGLE).unwrap(),
            SearchEngineType::Google
        );
        assert_eq!(
            SearchEngineType::from_str(SEARCH_ENGINE_BING).unwrap(),
            SearchEngineType::Bing
        );
        assert_eq!(
            SearchEngineType::from_str(SEARCH_ENGINE_BRAVE).unwrap(),
            SearchEngineType::BraveSearch
        );
        assert_eq!(
            SearchEngineType::from_str(SEARCH_ENGINE_BAIDU).unwrap(),
            SearchEngineType::Baidu
        );
        assert_eq!(
            SearchEngineType::from_str(SEARCH_ENGINE_SOUGOU_WEIXIN).unwrap(),
            SearchEngineType::SougouWeixin
        );

        // Test invalid engine types
        assert!(SearchEngineType::from_str("invalid").is_err());
        assert!(SearchEngineType::from_str("").is_err());
        assert!(SearchEngineType::from_str("web").is_err());
        assert!(SearchEngineType::from_str("api").is_err());
    }

    #[test]
    fn test_search_engine_type_partial_eq() {
        // Test equality
        assert_eq!(SearchEngineType::DuckDuckGo, SearchEngineType::DuckDuckGo);
        assert_eq!(SearchEngineType::Google, SearchEngineType::Google);
        assert_ne!(SearchEngineType::DuckDuckGo, SearchEngineType::Google);
        assert_ne!(SearchEngineType::Google, SearchEngineType::DuckDuckGo);
    }

    #[test]
    fn test_search_engine_type_debug() {
        // Test debug formatting
        assert_eq!(format!("{:?}", SearchEngineType::DuckDuckGo), "DuckDuckGo");
        assert_eq!(format!("{:?}", SearchEngineType::Google), "Google");
    }

    #[test]
    fn test_search_engine_type_clone() {
        // Test cloning
        let engine_type1 = SearchEngineType::DuckDuckGo;
        let engine_type2 = engine_type1;
        assert_eq!(engine_type1, engine_type2);
    }

    #[test]
    fn test_search_engine_type_copy() {
        // Test that SearchEngineType can be copied
        let engine_type1 = SearchEngineType::DuckDuckGo;
        let engine_type2 = engine_type1; // This should work because SearchEngineType is Copy
        assert_eq!(engine_type1, engine_type2);
    }

    #[test]
    fn test_query_patterns() {
        // Test that each engine type returns a valid query pattern
        assert_eq!(
            SearchEngineType::DuckDuckGo.get_query_pattern(),
            DUCKDUCKGO_QUERY_PATTERN
        );
        assert_eq!(
            SearchEngineType::Google.get_query_pattern(),
            GOOGLE_QUERY_PATTERN
        );
        assert_eq!(
            SearchEngineType::Bing.get_query_pattern(),
            BING_QUERY_PATTERN
        );
        assert_eq!(
            SearchEngineType::BraveSearch.get_query_pattern(),
            BRAVE_QUERY_PATTERN
        );
        assert_eq!(
            SearchEngineType::Baidu.get_query_pattern(),
            BAIDU_QUERY_PATTERN
        );
        assert_eq!(
            SearchEngineType::SougouWeixin.get_query_pattern(),
            SOUGOU_WEIXIN_QUERY_PATTERN
        );
    }

    #[test]
    fn test_search_result_creation() {
        let result = SearchResult {
            title: "Test Title".to_string(),
            url: "https://example.com".to_string(),
            snippet: "Test snippet".to_string(),
            rank: 1,
        };

        assert_eq!(result.title, "Test Title");
        assert_eq!(result.url, "https://example.com");
        assert_eq!(result.snippet, "Test snippet");
        assert_eq!(result.rank, 1);
    }
}
