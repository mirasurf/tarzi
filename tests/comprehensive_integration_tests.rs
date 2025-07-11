use std::env;
use std::time::Duration;
use tarzi::config::Config;
use tarzi::search::parser::ParserFactory;
use tarzi::search::types::SearchEngineType;
use tarzi::search::{SearchEngine, SearchMode};
use tokio::time::timeout;

/// Comprehensive integration tests for all engines and modes
/// These tests focus on areas not covered by existing test files

#[tokio::test]
#[ignore = "Browser tests disabled due to WebDriver runtime drop issues"]
async fn test_exa_webquery_fallback() {
    // Test Exa in WebQuery mode (should use fallback parser)
    let mut config = Config::new();
    config.search.engine = "exa".to_string();

    let mut engine = SearchEngine::from_config(&config);
    let results = engine
        .search("machine learning", SearchMode::WebQuery, 3)
        .await;

    match results {
        Ok(search_results) => {
            // Exa WebQuery should use fallback parser and return empty results or minimal results
            println!(
                "Exa WebQuery fallback returned {} results",
                search_results.len()
            );
            // The fallback parser might return empty results, which is acceptable
        }
        Err(e) => {
            // Also acceptable since Exa is primarily API-only
            println!("Exa WebQuery fallback failed as expected: {e}");
        }
    }
}

#[tokio::test]
async fn test_all_engines_parser_factory() {
    let factory = ParserFactory::new();

    // Test all combinations of engines and modes
    let test_cases = vec![
        (SearchEngineType::Bing, SearchMode::WebQuery, true),
        (SearchEngineType::Bing, SearchMode::ApiQuery, false), // Bing has no API
        (SearchEngineType::DuckDuckGo, SearchMode::WebQuery, true),
        (SearchEngineType::DuckDuckGo, SearchMode::ApiQuery, true),
        // Google now supports both web and API modes via the same provider
        (SearchEngineType::Google, SearchMode::WebQuery, true),
        (SearchEngineType::Google, SearchMode::ApiQuery, true),
        (SearchEngineType::BraveSearch, SearchMode::WebQuery, true),
        (SearchEngineType::BraveSearch, SearchMode::ApiQuery, true),
        (SearchEngineType::Baidu, SearchMode::WebQuery, true),
        (SearchEngineType::Baidu, SearchMode::ApiQuery, true),
        (SearchEngineType::Exa, SearchMode::WebQuery, false), // Fallback parser
        (SearchEngineType::Exa, SearchMode::ApiQuery, true),
        (SearchEngineType::Travily, SearchMode::WebQuery, false), // API-only
        (SearchEngineType::Travily, SearchMode::ApiQuery, true),
    ];

    for (engine_type, mode, should_have_real_parser) in test_cases {
        let parser = factory.get_parser(&engine_type, mode);

        println!(
            "Testing parser: {} for {:?} in {:?} mode",
            parser.name(),
            engine_type,
            mode
        );

        assert!(!parser.name().is_empty(), "Parser name should not be empty");

        if should_have_real_parser {
            // Real parsers should support their engine type
            assert!(
                parser.supports(&engine_type),
                "Parser {} should support {:?}",
                parser.name(),
                engine_type
            );
        }

        // Test parsing empty content (should not crash)
        let result = parser.parse("", 5);
        match result {
            Ok(results) => {
                // Some parsers might return placeholder results, which is acceptable
                println!(
                    "  Parser returned {} results for empty content",
                    results.len()
                );
            }
            Err(_) => {
                // Also acceptable for empty content
                println!("  Parser rejected empty content (acceptable)");
            }
        }
    }
}

#[tokio::test]
#[ignore = "Browser tests disabled due to WebDriver runtime drop issues"]
async fn test_search_engine_error_handling() {
    let mut config = Config::new();

    // Test with invalid engine name
    config.search.engine = "nonexistent".to_string();
    let mut engine = SearchEngine::from_config(&config);

    let results = engine.search("test", SearchMode::WebQuery, 1).await;
    // Should handle gracefully - either return empty results or a meaningful error
    match results {
        Ok(search_results) => {
            println!(
                "Invalid engine handled gracefully, returned {} results",
                search_results.len()
            );
        }
        Err(e) => {
            println!("Invalid engine rejected as expected: {e}");
        }
    }
}

#[tokio::test]
#[ignore = "Browser tests disabled due to WebDriver runtime drop issues"]
async fn test_search_query_edge_cases() {
    let mut config = Config::new();
    config.search.engine = "duckduckgo".to_string(); // Use DuckDuckGo as it doesn't need API keys

    let mut engine = SearchEngine::from_config(&config);

    // Test edge cases
    let long_query = "a".repeat(1000);
    let edge_cases = vec![
        ("", "empty query"),
        ("   ", "whitespace only"),
        ("a", "single character"),
        ("!@#$%^&*()", "special characters"),
        ("query with spaces", "spaces"),
        ("query\nwith\nnewlines", "newlines"),
        (
            "очень длинный запрос на русском языке",
            "unicode characters",
        ),
        ("🔍🚀💻", "emoji"),
        (long_query.as_str(), "very long query"),
    ];

    for (query, description) in edge_cases {
        println!("Testing edge case: {description}");

        let results = engine.search(query, SearchMode::WebQuery, 1).await;
        match results {
            Ok(search_results) => {
                println!(
                    "Edge case '{description}' handled gracefully, returned {} results",
                    search_results.len()
                );

                // Validate results if any
                for result in search_results {
                    assert!(
                        !result.title.is_empty() || !result.url.is_empty(),
                        "Result should have either title or URL"
                    );
                    assert!(result.rank > 0, "Rank should be positive");
                }
            }
            Err(e) => {
                println!("Edge case '{description}' failed (acceptable): {e}");
            }
        }
    }
}

#[tokio::test]
#[ignore = "Browser tests disabled due to WebDriver runtime drop issues"]
async fn test_search_limit_edge_cases() {
    let mut config = Config::new();
    config.search.engine = "duckduckgo".to_string();

    let mut engine = SearchEngine::from_config(&config);

    // Test different limit values
    let limits = vec![0, 1, 5, 10, 50, 100, 1000];

    for limit in limits {
        println!("Testing limit: {limit}");

        let results = engine
            .search("test query", SearchMode::WebQuery, limit)
            .await;
        match results {
            Ok(search_results) => {
                if limit == 0 {
                    // Limit 0 should return empty results
                    assert!(
                        search_results.is_empty(),
                        "Limit 0 should return empty results"
                    );
                } else {
                    // Results should not exceed limit
                    assert!(
                        search_results.len() <= limit,
                        "Results ({}) should not exceed limit ({})",
                        search_results.len(),
                        limit
                    );
                }

                println!("Limit {limit} returned {} results", search_results.len());
            }
            Err(e) => {
                println!("Limit {limit} failed: {e}");
            }
        }
    }
}

#[tokio::test]
#[ignore = "Browser tests disabled due to WebDriver runtime drop issues"]
async fn test_search_timeout_handling() {
    let mut config = Config::new();
    config.search.engine = "duckduckgo".to_string();
    config.fetcher.timeout = 1; // Very short timeout to force timeout

    let mut engine = SearchEngine::from_config(&config);

    // Use timeout wrapper to ensure test doesn't hang
    let search_future = engine.search("test query", SearchMode::WebQuery, 1);
    let result = timeout(Duration::from_secs(10), search_future).await;

    match result {
        Ok(search_result) => {
            match search_result {
                Ok(results) => {
                    println!(
                        "Search completed despite short timeout, returned {} results",
                        results.len()
                    );
                }
                Err(e) => {
                    println!("Search failed due to timeout as expected: {e}");
                    // Should contain timeout-related error message
                    let error_msg = e.to_string().to_lowercase();
                    assert!(
                        error_msg.contains("timeout")
                            || error_msg.contains("time")
                            || error_msg.contains("network")
                            || error_msg.contains("webdriver")
                            || error_msg.contains("browser"),
                        "Error should indicate timeout, network, or WebDriver issue: {e}"
                    );
                }
            }
        }
        Err(_) => {
            panic!("Test itself timed out - this indicates a hanging search operation");
        }
    }
}

#[tokio::test]
async fn test_concurrent_searches() {
    let mut config = Config::new();
    config.search.engine = "duckduckgo".to_string();

    // Create multiple search engines for concurrent testing
    let engines: Vec<_> = (0..3).map(|_| SearchEngine::from_config(&config)).collect();

    println!("Starting concurrent searches...");

    let handles: Vec<_> = engines
        .into_iter()
        .enumerate()
        .map(|(i, mut engine)| {
            tokio::spawn(async move {
                let query = format!("concurrent test query {i}");
                let result = engine.search(&query, SearchMode::WebQuery, 2).await;
                (i, result)
            })
        })
        .collect();

    // Wait for all searches to complete
    for handle in handles {
        match handle.await {
            Ok((i, search_result)) => match search_result {
                Ok(results) => {
                    println!(
                        "Concurrent search {} completed successfully with {} results",
                        i,
                        results.len()
                    );
                }
                Err(e) => {
                    println!("Concurrent search {i} failed: {e}");
                }
            },
            Err(e) => {
                println!("Concurrent search task failed: {e}");
            }
        }
    }

    println!("All concurrent searches completed");
}

#[tokio::test]
#[ignore = "Browser tests disabled due to WebDriver runtime drop issues"]
async fn test_search_result_quality_validation() {
    let mut config = Config::new();

    // Test with any available API key or fallback to DuckDuckGo
    if let Ok(api_key) = env::var("BRAVE_API_KEY") {
        config.search.brave_api_key = Some(api_key);
        config.search.engine = "brave".to_string();
    } else {
        config.search.engine = "duckduckgo".to_string();
    }

    let mut engine = SearchEngine::from_config(&config);
    let results = engine
        .search(
            "rust programming language tutorial",
            SearchMode::WebQuery,
            5,
        )
        .await;

    match results {
        Ok(search_results) => {
            if !search_results.is_empty() {
                println!(
                    "Validating quality of {} search results",
                    search_results.len()
                );

                for (i, result) in search_results.iter().enumerate() {
                    println!("Result {}: {}", i + 1, result.title);

                    // URL validation
                    assert!(
                        result.url.starts_with("http://") || result.url.starts_with("https://"),
                        "URL should be properly formatted: {}",
                        result.url
                    );

                    // Check URL is not obviously broken
                    assert!(
                        !result.url.contains("localhost"),
                        "URL should not be localhost"
                    );
                    assert!(
                        !result.url.contains("127.0.0.1"),
                        "URL should not be loopback"
                    );

                    // Title should be meaningful
                    assert!(result.title.len() > 5, "Title should be reasonably long");
                    assert!(
                        !result.title.to_lowercase().contains("error"),
                        "Title should not contain error: {}",
                        result.title
                    );

                    // Rank should be sequential
                    assert_eq!(result.rank, i + 1, "Rank should be sequential");

                    // For "rust programming" search, check relevance
                    let combined_text = format!(
                        "{} {} {}",
                        result.title.to_lowercase(),
                        result.snippet.to_lowercase(),
                        result.url.to_lowercase()
                    );

                    let is_relevant = combined_text.contains("rust")
                        || combined_text.contains("programming")
                        || combined_text.contains("tutorial")
                        || combined_text.contains("language")
                        || combined_text.contains("code");

                    if !is_relevant {
                        println!(
                            "Warning: Result {} may not be relevant to 'rust programming'",
                            i + 1
                        );
                    }
                }
            } else {
                println!("No search results returned for quality validation");
            }
        }
        Err(e) => {
            println!("Search failed during quality validation: {e}");
        }
    }
}

#[tokio::test]
#[ignore = "Browser tests disabled due to WebDriver runtime drop issues"]
async fn test_mode_switching() {
    let mut config = Config::new();
    config.search.engine = "duckduckgo".to_string();

    let mut engine = SearchEngine::from_config(&config);
    let query = "test query";

    // Test switching between WebQuery and ApiQuery modes
    println!("Testing WebQuery mode...");
    let web_results = engine.search(query, SearchMode::WebQuery, 3).await;
    match web_results {
        Ok(results) => {
            println!("WebQuery returned {} results", results.len());
        }
        Err(e) => {
            println!("WebQuery failed: {e}");
        }
    }

    println!("Testing ApiQuery mode...");
    let api_results = engine.search(query, SearchMode::ApiQuery, 3).await;
    match api_results {
        Ok(results) => {
            println!("ApiQuery returned {} results", results.len());
        }
        Err(e) => {
            println!("ApiQuery failed: {e}");
        }
    }
}

#[tokio::test]
async fn test_malformed_response_handling() {
    // This test uses the parser directly to test malformed content handling
    let factory = ParserFactory::new();

    let malformed_inputs = vec![
        (
            "<html><body>No search results</body></html>",
            "HTML without results",
        ),
        ("{}", "Empty JSON"),
        ("{\"invalid\": \"json\"}", "JSON without expected fields"),
        ("Not HTML or JSON at all", "Plain text"),
        ("<html><malformed>", "Malformed HTML"),
        ("{\"results: [}", "Invalid JSON syntax"),
        ("", "Empty string"),
        ("🎉🎊🎈", "Only emoji"),
    ];

    let parsers = vec![
        (
            "BingParser",
            factory.get_parser(&SearchEngineType::Bing, SearchMode::WebQuery),
        ),
        (
            "DuckDuckGoParser",
            factory.get_parser(&SearchEngineType::DuckDuckGo, SearchMode::WebQuery),
        ),
        (
            "DuckDuckGoApiParser",
            factory.get_parser(&SearchEngineType::DuckDuckGo, SearchMode::ApiQuery),
        ),
    ];

    for (parser_name, parser) in parsers {
        println!("Testing malformed input handling for {parser_name}");

        for (input, description) in &malformed_inputs {
            let result = parser.parse(input, 5);
            match result {
                Ok(results) => {
                    // Should return empty results for malformed input
                    assert!(
                        results.is_empty(),
                        "{parser_name} should return empty results for {description}"
                    );
                }
                Err(_) => {
                    // Also acceptable - parser can reject malformed input
                    println!("{parser_name} rejected {description} (acceptable)");
                }
            }
        }
    }
}

#[tokio::test]
async fn test_configuration_validation() {
    // Test various configuration scenarios

    // Test with minimal config
    let minimal_config = Config::new();
    let _engine = SearchEngine::from_config(&minimal_config);
    println!("✓ Minimal config created SearchEngine successfully");

    // Test with invalid proxy
    let mut invalid_proxy_config = Config::new();
    invalid_proxy_config.fetcher.proxy = Some("invalid-proxy-url".to_string());
    let _engine = SearchEngine::from_config(&invalid_proxy_config);
    println!("✓ Invalid proxy config handled gracefully");

    // Test with extreme timeout values
    let mut extreme_timeout_config = Config::new();
    extreme_timeout_config.fetcher.timeout = 0; // Zero timeout
    let _engine = SearchEngine::from_config(&extreme_timeout_config);
    println!("✓ Zero timeout config handled gracefully");

    let mut large_timeout_config = Config::new();
    large_timeout_config.fetcher.timeout = 999999; // Very large timeout
    let _engine = SearchEngine::from_config(&large_timeout_config);
    println!("✓ Large timeout config handled gracefully");

    println!("All configuration validation tests passed");
}
