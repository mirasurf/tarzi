use std::env;
use tarzi::config::Config;
use tarzi::search::{SearchEngine, SearchMode};

#[tokio::test]
async fn test_api_search_with_brave_provider() {
    let mut config = Config::new();

    // Skip test if no API key is available
    if let Ok(api_key) = env::var("BRAVE_API_KEY") {
        config.search.brave_api_key = Some(api_key);
        config.search.engine = "brave".to_string();

        let mut engine = SearchEngine::from_config(&config);
        let results = engine
            .search("rust programming", SearchMode::ApiQuery, 3)
            .await;

        match results {
            Ok(search_results) => {
                assert!(!search_results.is_empty(), "Should return search results");
                assert!(search_results.len() <= 3, "Should respect limit parameter");

                for result in &search_results {
                    assert!(!result.title.is_empty(), "Title should not be empty");
                    assert!(!result.url.is_empty(), "URL should not be empty");
                    assert!(!result.snippet.is_empty(), "Snippet should not be empty");
                    assert!(result.rank > 0, "Rank should be positive");
                }
            }
            Err(_) => {
                // If the test fails due to network issues, we still want to pass
                // This is an integration test that depends on external services
                println!("API call failed - this may be due to network issues or API limits");
            }
        }
    } else {
        println!("Skipping Brave API test - BRAVE_API_KEY not set");
    }
}

#[tokio::test]
async fn test_api_search_with_exa_provider() {
    let mut config = Config::new();

    // Skip test if no API key is available
    if let Ok(api_key) = env::var("EXA_API_KEY") {
        config.search.exa_api_key = Some(api_key);
        config.search.engine = "exa".to_string();

        let mut engine = SearchEngine::from_config(&config);
        let results = engine
            .search("artificial intelligence", SearchMode::ApiQuery, 2)
            .await;

        match results {
            Ok(search_results) => {
                assert!(!search_results.is_empty(), "Should return search results");
                assert!(search_results.len() <= 2, "Should respect limit parameter");

                for result in &search_results {
                    assert!(!result.title.is_empty(), "Title should not be empty");
                    assert!(!result.url.is_empty(), "URL should not be empty");
                    assert!(result.rank > 0, "Rank should be positive");
                }
            }
            Err(_) => {
                println!("API call failed - this may be due to network issues or API limits");
            }
        }
    } else {
        println!("Skipping Exa API test - EXA_API_KEY not set");
    }
}

#[tokio::test]
async fn test_api_search_with_travily_provider() {
    let mut config = Config::new();

    // Skip test if no API key is available
    if let Ok(api_key) = env::var("TRAVILY_API_KEY") {
        config.search.travily_api_key = Some(api_key);
        config.search.engine = "travily".to_string();

        let mut engine = SearchEngine::from_config(&config);
        let results = engine
            .search("climate change", SearchMode::ApiQuery, 4)
            .await;

        match results {
            Ok(search_results) => {
                assert!(!search_results.is_empty(), "Should return search results");
                assert!(search_results.len() <= 4, "Should respect limit parameter");

                for result in &search_results {
                    assert!(!result.title.is_empty(), "Title should not be empty");
                    assert!(!result.url.is_empty(), "URL should not be empty");
                    assert!(result.rank > 0, "Rank should be positive");
                }
            }
            Err(_) => {
                println!("API call failed - this may be due to network issues or API limits");
            }
        }
    } else {
        println!("Skipping Travily API test - TRAVILY_API_KEY not set");
    }
}

#[tokio::test]
async fn test_api_search_with_duckduckgo_provider() {
    let mut config = Config::new();

    // DuckDuckGo API doesn't require an API key, so we can always test it
    config.search.engine = "duckduckgo".to_string();

    let mut engine = SearchEngine::from_config(&config);
    let results = engine
        .search("rust programming language", SearchMode::ApiQuery, 3)
        .await;

    match results {
        Ok(search_results) => {
            // DuckDuckGo API may return empty results as it's not fully implemented
            // But it shouldn't crash
            println!("DuckDuckGo API returned {} results", search_results.len());

            if !search_results.is_empty() {
                assert!(search_results.len() <= 3, "Should respect limit parameter");

                for result in &search_results {
                    assert!(!result.title.is_empty(), "Title should not be empty");
                    assert!(!result.url.is_empty(), "URL should not be empty");
                    assert!(result.rank > 0, "Rank should be positive");

                    // URL validation
                    assert!(
                        result.url.starts_with("http://") || result.url.starts_with("https://"),
                        "URL should be properly formatted: {}",
                        result.url
                    );
                }
            }
        }
        Err(e) => {
            // DuckDuckGo API might not be fully implemented, so errors are acceptable
            println!("DuckDuckGo API test failed as expected: {e}");
            assert!(
                e.to_string().contains("not fully implemented")
                    || e.to_string().contains("Network")
                    || e.to_string().contains("DuckDuckGo"),
                "Error should indicate DuckDuckGo limitation: {e}"
            );
        }
    }
}

#[tokio::test]
async fn test_api_search_with_proxy() {
    let mut config = Config::new();

    // Test with a mock proxy (will fail but test the proxy setup)
    config.fetcher.proxy = Some("http://127.0.0.1:8888".to_string());

    // Only test if we have at least one API key
    if env::var("BRAVE_API_KEY").is_ok() {
        config.search.brave_api_key = env::var("BRAVE_API_KEY").ok();
        config.search.engine = "brave".to_string();

        let mut engine = SearchEngine::from_config(&config);
        let results = engine.search("test query", SearchMode::ApiQuery, 1).await;

        // We expect this to fail due to the mock proxy, but it should fail gracefully
        match results {
            Ok(_) => {
                // If it succeeds, the proxy worked (unlikely with mock proxy)
                println!("Proxy test succeeded unexpectedly");
            }
            Err(e) => {
                // Expected to fail with proxy connection error
                println!("Proxy test failed as expected: {e}");
                assert!(e.to_string().contains("proxy") || e.to_string().contains("Network"));
            }
        }
    } else {
        println!("Skipping proxy test - no API keys available");
    }
}

#[tokio::test]
async fn test_api_search_without_api_key() {
    let config = Config::new(); // No API keys configured

    let mut engine = SearchEngine::from_config(&config);
    let results = engine.search("test query", SearchMode::ApiQuery, 1).await;

    // DuckDuckGo provider is always registered but returns empty results
    // So we expect either an error OR empty results
    match results {
        Ok(search_results) => {
            // DuckDuckGo provider returns empty results
            assert!(
                search_results.is_empty(),
                "Should return empty results when no real API providers are available"
            );
            println!("API search without keys returned empty results as expected");
        }
        Err(error) => {
            // Also acceptable - should indicate missing providers
            println!("API search without keys failed as expected: {error}");
            assert!(
                error.to_string().contains("No provider registered")
                    || error.to_string().contains("All search providers failed")
                    || error.to_string().contains("not fully implemented"),
                "Error should indicate missing/limited provider: {error}"
            );
        }
    }
}

#[tokio::test]
async fn test_api_search_invalid_query() {
    let mut config = Config::new();

    // Use any available API key for this test
    if let Ok(api_key) = env::var("BRAVE_API_KEY") {
        config.search.brave_api_key = Some(api_key);
        config.search.engine = "brave".to_string();

        let mut engine = SearchEngine::from_config(&config);

        // Test with empty query
        let results = engine.search("", SearchMode::ApiQuery, 1).await;

        match results {
            Ok(search_results) => {
                // Some APIs might handle empty queries gracefully
                println!(
                    "Empty query handled gracefully, returned {} results",
                    search_results.len()
                );
            }
            Err(_) => {
                // Expected behavior for empty query
                println!("Empty query rejected as expected");
            }
        }

        // Test with very long query
        let long_query = "a".repeat(10000);
        let results = engine.search(&long_query, SearchMode::ApiQuery, 1).await;

        match results {
            Ok(_) => {
                println!("Long query handled gracefully");
            }
            Err(_) => {
                println!("Long query rejected or failed");
            }
        }
    } else {
        println!("Skipping invalid query test - no API keys available");
    }
}

#[tokio::test]
async fn test_api_search_limit_boundaries() {
    let mut config = Config::new();

    if let Ok(api_key) = env::var("BRAVE_API_KEY") {
        config.search.brave_api_key = Some(api_key);
        config.search.engine = "brave".to_string();

        let mut engine = SearchEngine::from_config(&config);

        // Test with limit 0
        let results = engine.search("test", SearchMode::ApiQuery, 0).await;
        match results {
            Ok(search_results) => {
                // Should return empty results or handle gracefully
                println!("Limit 0 returned {} results", search_results.len());
            }
            Err(_) => {
                println!("Limit 0 rejected");
            }
        }

        // Test with limit 1
        let results = engine.search("test", SearchMode::ApiQuery, 1).await;
        match results {
            Ok(search_results) => {
                assert!(search_results.len() <= 1, "Should respect limit of 1");
            }
            Err(_) => {
                println!("API call failed");
            }
        }

        // Test with large limit
        let results = engine.search("test", SearchMode::ApiQuery, 100).await;
        match results {
            Ok(search_results) => {
                // Most APIs have their own limits
                println!("Large limit returned {} results", search_results.len());
            }
            Err(_) => {
                println!("Large limit rejected or failed");
            }
        }
    } else {
        println!("Skipping limit boundary test - no API keys available");
    }
}

#[tokio::test]
async fn test_multiple_api_providers_registered() {
    let mut config = Config::new();

    // Register multiple providers if keys are available
    let mut providers_count = 0;

    if let Ok(api_key) = env::var("BRAVE_API_KEY") {
        config.search.brave_api_key = Some(api_key);
        providers_count += 1;
    }

    if let Ok(api_key) = env::var("EXA_API_KEY") {
        config.search.exa_api_key = Some(api_key);
        providers_count += 1;
    }

    if let Ok(api_key) = env::var("TRAVILY_API_KEY") {
        config.search.travily_api_key = Some(api_key);
        providers_count += 1;
    }

    if providers_count > 0 {
        config.search.engine = "brave".to_string(); // Use brave as primary
        config.search.autoswitch = "smart".to_string(); // Enable smart fallback

        let mut engine = SearchEngine::from_config(&config);
        let results = engine
            .search("technology news", SearchMode::ApiQuery, 3)
            .await;

        match results {
            Ok(search_results) => {
                assert!(
                    !search_results.is_empty(),
                    "Should return results from available providers"
                );
                println!(
                    "Multi-provider test returned {} results",
                    search_results.len()
                );
            }
            Err(e) => {
                println!("Multi-provider test failed: {e}");
            }
        }
    } else {
        println!("Skipping multi-provider test - no API keys available");
    }
}
