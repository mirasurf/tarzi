#[cfg(test)]
mod tests {
    use super::super::{SearchEngine, SearchEngineType, SearchMode};
    use std::str::FromStr;

    #[test]
    fn test_searchengine_from_config() {
        use crate::config::Config;
        let mut config = Config::new();
        config.search.api_key = Some("test-api-key-123".to_string());
        let engine = SearchEngine::from_config(&config);
        assert_eq!(*engine.api_key(), Some("test-api-key-123".to_string()));
        assert_eq!(*engine.engine_type(), SearchEngineType::Bing);
        assert_eq!(
            engine.query_pattern(),
            "https://www.bing.com/search?q={query}"
        );
        assert_eq!(engine.user_agent(), "Mozilla/5.0 (compatible; Tarzi/1.0)");
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
            SearchEngineType::from_str("tavily").unwrap(),
            SearchEngineType::Tavily
        );
        assert_eq!(
            SearchEngineType::from_str("searchapi").unwrap(),
            SearchEngineType::SearchApi
        );

        // Test custom engine
        let custom = SearchEngineType::from_str("custom-engine").unwrap();
        assert!(matches!(custom, SearchEngineType::Custom(_)));
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
            SearchEngineType::Tavily.get_query_pattern(),
            "https://tavily.com/search?q={query}"
        );
        assert_eq!(
            SearchEngineType::SearchApi.get_query_pattern(),
            "https://www.searchapi.io/search?q={query}"
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
        assert_eq!(*engine.api_key(), None);
        assert_eq!(*engine.engine_type(), SearchEngineType::Bing);
        assert_eq!(
            engine.query_pattern(),
            "https://www.bing.com/search?q={query}"
        );
        assert_eq!(engine.user_agent(), "Mozilla/5.0 (compatible; Tarzi/1.0)");
    }

    #[test]
    fn test_searchengine_with_api_key() {
        let engine = SearchEngine::new().with_api_key("test-key".to_string());
        assert_eq!(*engine.api_key(), Some("test-key".to_string()));
    }

    #[test]
    fn test_searchengine_default() {
        let engine1 = SearchEngine::new();
        let engine2 = SearchEngine::default();
        assert_eq!(engine1.api_key(), engine2.api_key());
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
    fn test_search_mode_copy() {
        let mode1 = SearchMode::WebQuery;
        let mode2 = mode1; // This should work because SearchMode is Copy
        assert_eq!(mode1, mode2);
    }
}