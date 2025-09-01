use super::{SearchEngine, SearchEngineType};
use crate::constants::*;
use std::str::FromStr;

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
        SearchEngineType::from_str(SEARCH_ENGINE_EXA).unwrap(),
        SearchEngineType::Exa
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
    let engine_type2 = engine_type1.clone();
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
    assert_eq!(SearchEngineType::Exa.get_query_pattern(), EXA_QUERY_PATTERN);
}

#[test]
fn test_search_engine_creation() {
    // Test that SearchEngine can be created
    let engine = SearchEngine::new();
    assert_eq!(engine.engine_type, SearchEngineType::DuckDuckGo);
    assert_eq!(engine.query_pattern, DEFAULT_QUERY_PATTERN);
}

#[test]
fn test_search_engine_from_config() {
    // Test creating SearchEngine from config
    let mut config = crate::config::Config::new();
    config.search.engine = SEARCH_ENGINE_GOOGLE.to_string();
    config.search.query_pattern = "custom pattern".to_string();

    let engine = SearchEngine::from_config(&config);
    assert_eq!(engine.engine_type, SearchEngineType::Google);
    assert_eq!(engine.query_pattern, "custom pattern");
}
