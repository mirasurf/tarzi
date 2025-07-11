use super::{SearchEngine, SearchEngineType, SearchMode};
use std::str::FromStr;

#[test]
fn test_searchengine_from_config() {
    use crate::config::Config;
    let mut config = Config::new();
    config.search.brave_api_key = Some("test-brave-api-key-123".to_string());
    let engine = SearchEngine::from_config(&config);
    assert_eq!(*engine.engine_type(), SearchEngineType::DuckDuckGo);
    assert_eq!(engine.query_pattern(), "https://duckduckgo.com/?q={query}");
    assert_eq!(engine.user_agent(), crate::constants::DEFAULT_USER_AGENT);
}

#[test]
fn test_search_engine_type_parsing() {
    assert_eq!(
        SearchEngineType::from_str("bing").unwrap(),
        SearchEngineType::Bing
    );
    assert_eq!(
        SearchEngineType::from_str("google").unwrap(),
        SearchEngineType::Google
    );
    assert_eq!(
        SearchEngineType::from_str("duckduckgo").unwrap(),
        SearchEngineType::DuckDuckGo
    );
    assert_eq!(
        SearchEngineType::from_str("brave").unwrap(),
        SearchEngineType::BraveSearch
    );
    assert_eq!(
        SearchEngineType::from_str("baidu").unwrap(),
        SearchEngineType::Baidu
    );

    // Test invalid engine (should return error)
    assert!(SearchEngineType::from_str("custom-engine").is_err());
}

#[test]
fn test_query_patterns() {
    assert_eq!(
        SearchEngineType::Bing.get_query_pattern(),
        "https://www.bing.com/search?q={query}"
    );
    assert_eq!(
        SearchEngineType::Google.get_query_pattern(),
        "https://www.google.com/search?q={query}"
    );
    assert_eq!(
        SearchEngineType::DuckDuckGo.get_query_pattern(),
        "https://duckduckgo.com/?q={query}"
    );
    assert_eq!(
        SearchEngineType::BraveSearch.get_query_pattern(),
        "https://search.brave.com/search?q={query}"
    );
    assert_eq!(
        SearchEngineType::Baidu.get_query_pattern(),
        "https://www.baidu.com/s?wd={query}"
    );
}

#[test]
fn test_custom_query_pattern() {
    use crate::config::Config;
    let mut config = Config::new();
    config.search.engine = "google".to_string();
    config.search.query_pattern =
        "https://custom-search.com/search?query={query}&lang=en".to_string();

    let engine = SearchEngine::from_config(&config);
    assert_eq!(*engine.engine_type(), SearchEngineType::Google);
    assert_eq!(
        engine.query_pattern(),
        "https://custom-search.com/search?query={query}&lang=en"
    );
}

#[test]
fn test_custom_user_agent() {
    use crate::config::Config;
    let mut config = Config::new();
    config.fetcher.user_agent = "Custom User Agent 1.0".to_string();

    let engine = SearchEngine::from_config(&config);
    assert_eq!(engine.user_agent(), "Custom User Agent 1.0");
}

#[test]
fn test_searchengine_new() {
    let engine = SearchEngine::new();
    assert_eq!(*engine.engine_type(), SearchEngineType::DuckDuckGo);
    assert_eq!(engine.query_pattern(), "https://duckduckgo.com/?q={query}");
    assert_eq!(engine.user_agent(), crate::constants::DEFAULT_USER_AGENT);
}

#[test]
fn test_searchengine_default() {
    let engine1 = SearchEngine::new();
    let engine2 = SearchEngine::default();
    assert_eq!(engine1.engine_type(), engine2.engine_type());
    assert_eq!(engine1.query_pattern(), engine2.query_pattern());
    assert_eq!(engine1.user_agent(), engine2.user_agent());
}

#[test]
fn test_search_mode_parsing() {
    assert_eq!(
        SearchMode::from_str("webquery").unwrap(),
        SearchMode::WebQuery
    );
    assert_eq!(
        SearchMode::from_str("apiquery").unwrap(),
        SearchMode::ApiQuery
    );
    assert_eq!(
        SearchMode::from_str("WEBQUERY").unwrap(),
        SearchMode::WebQuery
    );
    assert_eq!(
        SearchMode::from_str("APIQUERY").unwrap(),
        SearchMode::ApiQuery
    );

    // Test invalid modes
    assert!(SearchMode::from_str("invalid").is_err());
    assert!(SearchMode::from_str("").is_err());
    assert!(SearchMode::from_str("web").is_err());
    assert!(SearchMode::from_str("api").is_err());
}

#[test]
fn test_search_mode_partial_eq() {
    assert_eq!(SearchMode::WebQuery, SearchMode::WebQuery);
    assert_eq!(SearchMode::ApiQuery, SearchMode::ApiQuery);
    assert_ne!(SearchMode::WebQuery, SearchMode::ApiQuery);
    assert_ne!(SearchMode::ApiQuery, SearchMode::WebQuery);
}

#[test]
fn test_search_mode_debug() {
    assert_eq!(format!("{:?}", SearchMode::WebQuery), "WebQuery");
    assert_eq!(format!("{:?}", SearchMode::ApiQuery), "ApiQuery");
}

#[test]
fn test_search_mode_clone() {
    let mode1 = SearchMode::WebQuery;
    let mode2 = mode1;
    assert_eq!(mode1, mode2);

    let mode3 = SearchMode::ApiQuery;
    let mode4 = mode3;
    assert_eq!(mode3, mode4);
}

#[test]
fn test_engine_capabilities() {
    // Test DuckDuckGo capabilities
    assert!(SearchEngineType::DuckDuckGo.supports_web_query());
    assert!(SearchEngineType::DuckDuckGo.supports_api_query());
    assert!(!SearchEngineType::DuckDuckGo.requires_api_key());

    // Test Google capabilities
    assert!(SearchEngineType::Google.supports_web_query());
    assert!(SearchEngineType::Google.supports_api_query());
    assert!(SearchEngineType::Google.requires_api_key());

    // Test Bing capabilities
    assert!(SearchEngineType::Bing.supports_web_query());
    assert!(!SearchEngineType::Bing.supports_api_query()); // Bing doesn't have public API
    assert!(!SearchEngineType::Bing.requires_api_key());

    // Test Brave capabilities
    assert!(SearchEngineType::BraveSearch.supports_web_query());
    assert!(SearchEngineType::BraveSearch.supports_api_query());
    assert!(SearchEngineType::BraveSearch.requires_api_key());

    // Test Baidu capabilities
    assert!(SearchEngineType::Baidu.supports_web_query());
    assert!(SearchEngineType::Baidu.supports_api_query());
    assert!(SearchEngineType::Baidu.requires_api_key());

    // Test API-only engines
    assert!(SearchEngineType::Exa.supports_web_query());
    assert!(SearchEngineType::Exa.supports_api_query());
    assert!(SearchEngineType::Exa.requires_api_key());

    assert!(!SearchEngineType::Travily.supports_web_query());
    assert!(SearchEngineType::Travily.supports_api_query());
    assert!(SearchEngineType::Travily.requires_api_key());

    assert!(SearchEngineType::Google.supports_web_query());
    assert!(SearchEngineType::Google.supports_api_query());
    assert!(SearchEngineType::Google.requires_api_key());
}

#[test]
fn test_api_key_fields() {
    assert_eq!(SearchEngineType::Bing.get_api_key_field(), None);
    assert_eq!(SearchEngineType::DuckDuckGo.get_api_key_field(), None);
    assert_eq!(
        SearchEngineType::BraveSearch.get_api_key_field(),
        Some("brave_api_key")
    );
    assert_eq!(
        SearchEngineType::Baidu.get_api_key_field(),
        Some("baidu_api_key")
    );
    assert_eq!(
        SearchEngineType::Exa.get_api_key_field(),
        Some("exa_api_key")
    );
    assert_eq!(
        SearchEngineType::Travily.get_api_key_field(),
        Some("travily_api_key")
    );
}

#[test]
fn test_engine_mode_validation() {
    use crate::config::Config;

    // Test that Bing doesn't support API query
    let mut config = Config::new();
    config.search.engine = "bing".to_string();
    config.search.mode = "apiquery".to_string();

    let engine = SearchEngine::from_config(&config);
    // Test the validation logic directly
    assert!(!engine.engine_type().supports_api_query());
    assert!(engine.engine_type().supports_web_query());

    // Test that Travily doesn't support web query
    let mut config2 = Config::new();
    config2.search.engine = "travily".to_string();
    config2.search.mode = "webquery".to_string();

    let engine2 = SearchEngine::from_config(&config2);
    assert!(!engine2.engine_type().supports_web_query());
    assert!(engine2.engine_type().supports_api_query());

    // Test that Google supports both modes
    let mut config3 = Config::new();
    config3.search.engine = "google".to_string();
    config3.search.mode = "webquery".to_string();

    let engine3 = SearchEngine::from_config(&config3);
    assert!(engine3.engine_type().supports_web_query());
    assert!(engine3.engine_type().supports_api_query());
}

#[test]
fn test_search_mode_copy() {
    let mode1 = SearchMode::WebQuery;
    let mode2 = mode1; // This should work because SearchMode is Copy
    assert_eq!(mode1, mode2);
}

#[test]
fn test_query_patterns_for_modes() {
    use crate::search::SearchMode;

    // Test DuckDuckGo patterns
    assert_eq!(
        SearchEngineType::DuckDuckGo.get_query_pattern_for_mode(SearchMode::WebQuery),
        "https://duckduckgo.com/?q={query}"
    );
    assert_eq!(
        SearchEngineType::DuckDuckGo.get_query_pattern_for_mode(SearchMode::ApiQuery),
        "https://api.duckduckgo.com/?q={query}&format=json"
    );

    // Test Google patterns
    assert_eq!(
        SearchEngineType::Google.get_query_pattern_for_mode(SearchMode::WebQuery),
        "https://www.google.com/search?q={query}"
    );
    assert_eq!(
        SearchEngineType::Google.get_query_pattern_for_mode(SearchMode::ApiQuery),
        ""
    );

    // Test Bing patterns
    assert_eq!(
        SearchEngineType::Bing.get_query_pattern_for_mode(SearchMode::WebQuery),
        "https://www.bing.com/search?q={query}"
    );
    assert_eq!(
        SearchEngineType::Bing.get_query_pattern_for_mode(SearchMode::ApiQuery),
        ""
    );

    // Test Brave patterns
    assert_eq!(
        SearchEngineType::BraveSearch.get_query_pattern_for_mode(SearchMode::WebQuery),
        "https://search.brave.com/search?q={query}"
    );
    assert_eq!(
        SearchEngineType::BraveSearch.get_query_pattern_for_mode(SearchMode::ApiQuery),
        "https://api.search.brave.com/res/v1/web/search"
    );

    // Test Baidu patterns
    assert_eq!(
        SearchEngineType::Baidu.get_query_pattern_for_mode(SearchMode::WebQuery),
        "https://www.baidu.com/s?wd={query}"
    );
    assert_eq!(
        SearchEngineType::Baidu.get_query_pattern_for_mode(SearchMode::ApiQuery),
        "https://api.baidu.com/search"
    );

    // Test API-only engines
    assert_eq!(
        SearchEngineType::Exa.get_query_pattern_for_mode(SearchMode::WebQuery),
        "https://exa.ai/search?q={query}"
    );
    assert_eq!(
        SearchEngineType::Exa.get_query_pattern_for_mode(SearchMode::ApiQuery),
        "https://api.exa.ai/search"
    );

    assert_eq!(
        SearchEngineType::Travily.get_query_pattern_for_mode(SearchMode::WebQuery),
        ""
    );
    assert_eq!(
        SearchEngineType::Travily.get_query_pattern_for_mode(SearchMode::ApiQuery),
        "https://api.tavily.com/search"
    );

    assert_eq!(
        SearchEngineType::Google.get_query_pattern_for_mode(SearchMode::WebQuery),
        "https://www.google.com/search?q={query}"
    );
    assert_eq!(
        SearchEngineType::Google.get_query_pattern_for_mode(SearchMode::ApiQuery),
        ""
    );
}
