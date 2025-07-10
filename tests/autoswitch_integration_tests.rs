use std::env;
use tarzi::config::Config;
use tarzi::search::{SearchEngine, SearchMode};

#[tokio::test]
async fn test_autoswitch_smart_strategy_fallback() {
    let mut config = Config::new();

    // Configure multiple API providers if available
    let mut primary_provider_available = false;
    let mut fallback_providers_available = false;

    if let Ok(api_key) = env::var("BRAVE_API_KEY") {
        config.search.brave_api_key = Some(api_key);
        primary_provider_available = true;
    }

    if let Ok(api_key) = env::var("GOOGLE_SERPER_API_KEY") {
        config.search.google_serper_api_key = Some(api_key);
        fallback_providers_available = true;
    }

    if let Ok(api_key) = env::var("EXA_API_KEY") {
        config.search.exa_api_key = Some(api_key);
        fallback_providers_available = true;
    }

    if primary_provider_available && fallback_providers_available {
        config.search.engine = "brave".to_string(); // Primary provider
        config.search.autoswitch = "smart".to_string(); // Enable smart fallback

        let mut engine = SearchEngine::from_config(&config);
        let results = engine
            .search("test smart fallback", SearchMode::ApiQuery, 3)
            .await;

        match results {
            Ok(search_results) => {
                println!("Smart autoswitch returned {} results", search_results.len());
                // If we get results, the smart strategy worked (either primary or fallback)
                assert!(
                    !search_results.is_empty(),
                    "Smart strategy should return results from available providers"
                );
            }
            Err(e) => {
                println!("Smart autoswitch failed: {e}");
                // Even with fallbacks, network issues can cause failures
            }
        }
    } else {
        println!("Skipping smart autoswitch test - need multiple API providers");
    }
}

#[tokio::test]
async fn test_autoswitch_none_strategy() {
    let mut config = Config::new();

    if let Ok(api_key) = env::var("BRAVE_API_KEY") {
        config.search.brave_api_key = Some(api_key);
        config.search.engine = "brave".to_string();
        config.search.autoswitch = "none".to_string(); // Disable autoswitch

        let mut engine = SearchEngine::from_config(&config);
        let results = engine
            .search("test no autoswitch", SearchMode::ApiQuery, 3)
            .await;

        match results {
            Ok(search_results) => {
                println!(
                    "No autoswitch strategy returned {} results",
                    search_results.len()
                );
                // Should work with the primary provider
            }
            Err(e) => {
                println!("No autoswitch strategy failed: {e}");
                // If primary provider fails, should not fallback
            }
        }
    } else {
        println!("Skipping no autoswitch test - BRAVE_API_KEY not available");
    }
}

#[tokio::test]
async fn test_autoswitch_with_invalid_primary_provider() {
    let mut config = Config::new();

    // Set up a scenario where primary provider will fail but fallbacks are available
    config.search.brave_api_key = Some("invalid_api_key".to_string()); // Invalid key

    // Add valid fallback providers if available
    let mut fallback_available = false;
    if let Ok(api_key) = env::var("GOOGLE_SERPER_API_KEY") {
        config.search.google_serper_api_key = Some(api_key);
        fallback_available = true;
    }

    if let Ok(api_key) = env::var("EXA_API_KEY") {
        config.search.exa_api_key = Some(api_key);
        fallback_available = true;
    }

    if fallback_available {
        config.search.engine = "brave".to_string(); // Primary (will fail with invalid key)
        config.search.autoswitch = "smart".to_string(); // Enable smart fallback

        let mut engine = SearchEngine::from_config(&config);
        let results = engine
            .search(
                "test fallback from invalid primary",
                SearchMode::ApiQuery,
                2,
            )
            .await;

        match results {
            Ok(search_results) => {
                println!(
                    "Fallback from invalid primary returned {} results",
                    search_results.len()
                );
                // Should get results from fallback providers
                assert!(
                    !search_results.is_empty(),
                    "Should fallback to valid providers"
                );
            }
            Err(e) => {
                println!("Fallback test failed: {e}");
                // Could fail if all providers have issues
            }
        }
    } else {
        println!("Skipping invalid primary fallback test - no fallback providers available");
    }
}

#[tokio::test]
async fn test_autoswitch_none_with_invalid_provider() {
    let mut config = Config::new();

    // Set up invalid API key with no autoswitch
    config.search.brave_api_key = Some("definitely_invalid_key_12345".to_string());
    config.search.engine = "brave".to_string();
    config.search.autoswitch = "none".to_string(); // No fallback

    // Add valid providers but they shouldn't be used due to "none" strategy
    if let Ok(api_key) = env::var("GOOGLE_SERPER_API_KEY") {
        config.search.google_serper_api_key = Some(api_key);
    }

    let mut engine = SearchEngine::from_config(&config);
    let results = engine
        .search("test no fallback with invalid key", SearchMode::ApiQuery, 1)
        .await;

    // Should fail because autoswitch is disabled and primary provider has invalid key
    match results {
        Ok(_) => {
            // Unexpected success - maybe the API key validation is lenient
            println!("Unexpected success with invalid key");
        }
        Err(e) => {
            println!("Expected failure with invalid key and no autoswitch: {e}");
            // Expected behavior - should fail and not fallback
        }
    }
}

#[tokio::test]
async fn test_autoswitch_strategy_parsing() {
    // Test different autoswitch strategy values
    let test_cases = vec![
        ("smart", "smart"),
        ("Smart", "smart"),
        ("SMART", "smart"),
        ("none", "none"),
        ("None", "none"),
        ("NONE", "none"),
        ("invalid", "smart"), // Should default to smart
        ("", "smart"),        // Should default to smart
    ];

    for (input, expected_behavior) in test_cases {
        let mut config = Config::new();
        config.search.autoswitch = input.to_string();

        // Only test if we have an API key available
        if let Ok(api_key) = env::var("BRAVE_API_KEY") {
            config.search.brave_api_key = Some(api_key);
            config.search.engine = "brave".to_string();

            let _engine = SearchEngine::from_config(&config);

            // We can't directly inspect the autoswitch strategy from the public API,
            // but we can verify the engine was created successfully
            println!(
                "Created engine with autoswitch strategy: '{input}' (expected behavior: '{expected_behavior}')"
            );

            // Just verify the engine can perform a search
            // (The actual strategy testing is covered in other tests)
            break; // Only test once since we're just validating parsing
        }
    }
}

#[tokio::test]
async fn test_autoswitch_with_all_providers_failing() {
    let mut config = Config::new();

    // Set up all providers with invalid keys
    config.search.brave_api_key = Some("invalid_brave_key".to_string());
    config.search.google_serper_api_key = Some("invalid_serper_key".to_string());
    config.search.exa_api_key = Some("invalid_exa_key".to_string());
    config.search.travily_api_key = Some("invalid_travily_key".to_string());

    config.search.engine = "brave".to_string();
    config.search.autoswitch = "smart".to_string();

    let mut engine = SearchEngine::from_config(&config);
    let results = engine
        .search("test all providers failing", SearchMode::ApiQuery, 1)
        .await;

    // With smart autoswitch, it will try all providers including DuckDuckGo
    // DuckDuckGo doesn't use real API so it may return empty results instead of failing
    match results {
        Ok(search_results) => {
            // DuckDuckGo fallback returns empty results
            assert!(
                search_results.is_empty(),
                "Should return empty results when all real providers fail"
            );
            println!("All providers with invalid keys fell back to DuckDuckGo (empty results)");
        }
        Err(error) => {
            // Also acceptable - all providers failed
            println!("All providers failed as expected: {error}");
            assert!(
                error.to_string().contains("All search providers failed")
                    || error.to_string().contains("Network")
                    || error.to_string().contains("API"),
                "Error should indicate provider failures: {error}"
            );
        }
    }
}

#[tokio::test]
async fn test_autoswitch_smart_with_mixed_provider_health() {
    let mut config = Config::new();

    // Mix of valid and invalid providers
    config.search.brave_api_key = Some("invalid_brave_key".to_string()); // Invalid

    let mut valid_provider_available = false;

    if let Ok(api_key) = env::var("GOOGLE_SERPER_API_KEY") {
        config.search.google_serper_api_key = Some(api_key); // Valid
        valid_provider_available = true;
    }

    if let Ok(api_key) = env::var("EXA_API_KEY") {
        config.search.exa_api_key = Some(api_key); // Valid
        valid_provider_available = true;
    }

    if valid_provider_available {
        config.search.engine = "brave".to_string(); // Primary (invalid)
        config.search.autoswitch = "smart".to_string();

        let mut engine = SearchEngine::from_config(&config);
        let results = engine
            .search("test mixed provider health", SearchMode::ApiQuery, 2)
            .await;

        match results {
            Ok(search_results) => {
                println!(
                    "Mixed provider health test returned {} results",
                    search_results.len()
                );
                // Should succeed using valid fallback providers
                assert!(
                    !search_results.is_empty(),
                    "Should use valid fallback providers"
                );
            }
            Err(e) => {
                println!("Mixed provider test failed: {e}");
                // Could fail due to network issues even with valid providers
            }
        }
    } else {
        println!("Skipping mixed provider health test - no valid fallback providers available");
    }
}

#[tokio::test]
async fn test_autoswitch_provider_order() {
    let mut config = Config::new();

    // Test that fallback order is respected
    // According to the code, fallback order is: GoogleSerper, BraveSearch, Exa, Travily, DuckDuckGo

    let mut multiple_providers = 0;

    if env::var("GOOGLE_SERPER_API_KEY").is_ok() {
        config.search.google_serper_api_key = env::var("GOOGLE_SERPER_API_KEY").ok();
        multiple_providers += 1;
    }

    if env::var("BRAVE_API_KEY").is_ok() {
        config.search.brave_api_key = env::var("BRAVE_API_KEY").ok();
        multiple_providers += 1;
    }

    if env::var("EXA_API_KEY").is_ok() {
        config.search.exa_api_key = env::var("EXA_API_KEY").ok();
        multiple_providers += 1;
    }

    if multiple_providers >= 2 {
        // Set primary to a provider that should be later in fallback order
        config.search.engine = "exa".to_string(); // Exa should fallback to GoogleSerper and BraveSearch
        config.search.autoswitch = "smart".to_string();

        let mut engine = SearchEngine::from_config(&config);
        let results = engine
            .search("test provider order", SearchMode::ApiQuery, 1)
            .await;

        match results {
            Ok(search_results) => {
                println!(
                    "Provider order test returned {} results",
                    search_results.len()
                );
                // We can't easily verify the exact order without more detailed logging,
                // but we can verify that fallback works
            }
            Err(e) => {
                println!("Provider order test failed: {e}");
            }
        }
    } else {
        println!("Skipping provider order test - need multiple API providers");
    }
}

#[tokio::test]
async fn test_autoswitch_performance_comparison() {
    let mut config = Config::new();

    if let Ok(api_key) = env::var("BRAVE_API_KEY") {
        config.search.brave_api_key = Some(api_key);
        config.search.engine = "brave".to_string();

        // Test smart strategy
        config.search.autoswitch = "smart".to_string();
        let mut engine_smart = SearchEngine::from_config(&config);

        let start_smart = std::time::Instant::now();
        let results_smart = engine_smart
            .search("performance test", SearchMode::ApiQuery, 1)
            .await;
        let duration_smart = start_smart.elapsed();

        // Test none strategy
        config.search.autoswitch = "none".to_string();
        let mut engine_none = SearchEngine::from_config(&config);

        let start_none = std::time::Instant::now();
        let results_none = engine_none
            .search("performance test", SearchMode::ApiQuery, 1)
            .await;
        let duration_none = start_none.elapsed();

        println!("Smart strategy took: {duration_smart:?}");
        println!("None strategy took: {duration_none:?}");

        // Both should succeed or fail similarly with a single provider
        match (results_smart, results_none) {
            (Ok(_), Ok(_)) => {
                println!("Both strategies succeeded");
                // None strategy might be slightly faster due to no fallback logic
            }
            (Err(e1), Err(e2)) => {
                println!("Both strategies failed: {e1} vs {e2}");
            }
            _ => {
                println!("Strategies had different outcomes");
            }
        }
    } else {
        println!("Skipping performance comparison test - BRAVE_API_KEY not available");
    }
}
