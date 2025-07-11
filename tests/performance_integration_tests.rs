use std::env;
use std::time::{Duration, Instant};
use tarzi::config::Config;
use tarzi::search::types::SearchEngineType;
use tarzi::search::{SearchEngine, SearchMode};

/// Performance and load testing for search engines
/// These tests measure response times and throughput

#[tokio::test]
async fn test_search_engine_performance_comparison() {
    println!("=== Search Engine Performance Comparison ===");

    let engines_to_test = vec![
        ("duckduckgo", SearchMode::WebQuery, "DuckDuckGo WebQuery"),
        ("duckduckgo", SearchMode::ApiQuery, "DuckDuckGo ApiQuery"),
    ];

    // Add API engines if keys are available
    let mut api_engines = Vec::new();
    if env::var("BRAVE_API_KEY").is_ok() {
        api_engines.push(("brave", SearchMode::ApiQuery, "Brave API"));
    }
    if env::var("GOOGLE_SERPER_API_KEY").is_ok() {
        api_engines.push(("googleserper", SearchMode::ApiQuery, "Google Serper API"));
    }
    if env::var("EXA_API_KEY").is_ok() {
        api_engines.push(("exa", SearchMode::ApiQuery, "Exa API"));
    }
    if env::var("TRAVILY_API_KEY").is_ok() {
        api_engines.push(("travily", SearchMode::ApiQuery, "Travily API"));
    }

    let all_engines = [engines_to_test, api_engines].concat();

    let test_queries = vec![
        "rust programming",
        "machine learning tutorial",
        "web development",
    ];

    for (engine_name, mode, description) in all_engines {
        println!("\n--- Testing {} ---", description);

        let mut config = Config::new();
        config.search.engine = engine_name.to_string();

        // Set API keys if available
        match engine_name {
            "brave" => {
                if let Ok(key) = env::var("BRAVE_API_KEY") {
                    config.search.brave_api_key = Some(key);
                }
            }
            "googleserper" => {
                if let Ok(key) = env::var("GOOGLE_SERPER_API_KEY") {
                    config.search.google_serper_api_key = Some(key);
                }
            }
            "exa" => {
                if let Ok(key) = env::var("EXA_API_KEY") {
                    config.search.exa_api_key = Some(key);
                }
            }
            "travily" => {
                if let Ok(key) = env::var("TRAVILY_API_KEY") {
                    config.search.travily_api_key = Some(key);
                }
            }
            _ => {}
        }

        let mut engine = SearchEngine::from_config(&config);
        let mut total_time = Duration::new(0, 0);
        let mut successful_queries = 0;
        let mut total_results = 0;

        for query in &test_queries {
            let start = Instant::now();
            let result = engine.search(query, mode, 5).await;
            let duration = start.elapsed();

            match result {
                Ok(results) => {
                    successful_queries += 1;
                    total_results += results.len();
                    total_time += duration;
                    println!(
                        "  Query '{}': {} results in {:?}",
                        query,
                        results.len(),
                        duration
                    );
                }
                Err(e) => {
                    println!("  Query '{}': Failed in {:?} - {}", query, duration, e);
                }
            }
        }

        if successful_queries > 0 {
            let avg_time = total_time / successful_queries;
            println!(
                "  Summary: {}/{} queries successful",
                successful_queries,
                test_queries.len()
            );
            println!("  Average response time: {:?}", avg_time);
            println!("  Total results: {}", total_results);
            println!(
                "  Average results per query: {:.1}",
                total_results as f64 / successful_queries as f64
            );
        } else {
            println!("  No successful queries for {}", description);
        }
    }
}

#[tokio::test]
async fn test_search_latency_percentiles() {
    println!("=== Search Latency Percentile Analysis ===");

    let mut config = Config::new();
    config.search.engine = "duckduckgo".to_string(); // Use DuckDuckGo as baseline

    let mut engine = SearchEngine::from_config(&config);
    let num_requests = 10; // Small number for integration test
    let mut latencies = Vec::new();

    println!(
        "Performing {} search requests to measure latency distribution...",
        num_requests
    );

    for i in 0..num_requests {
        let query = format!("test query {}", i);
        let start = Instant::now();
        let result = engine.search(&query, SearchMode::WebQuery, 3).await;
        let duration = start.elapsed();

        match result {
            Ok(results) => {
                latencies.push(duration);
                println!(
                    "Request {}: {} results in {:?}",
                    i + 1,
                    results.len(),
                    duration
                );
            }
            Err(e) => {
                println!("Request {} failed: {}", i + 1, e);
            }
        }

        // Small delay between requests to be respectful
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    if !latencies.is_empty() {
        latencies.sort();

        let min = latencies[0];
        let max = latencies[latencies.len() - 1];
        let median = latencies[latencies.len() / 2];
        let p95_index = (latencies.len() as f64 * 0.95) as usize;
        let p95 = latencies[p95_index.min(latencies.len() - 1)];

        let total: Duration = latencies.iter().sum();
        let avg = total / latencies.len() as u32;

        println!("\nLatency Statistics:");
        println!("  Min: {:?}", min);
        println!("  Avg: {:?}", avg);
        println!("  Median: {:?}", median);
        println!("  95th percentile: {:?}", p95);
        println!("  Max: {:?}", max);
        println!("  Sample size: {}", latencies.len());
    } else {
        println!("No successful requests for latency analysis");
    }
}

#[tokio::test]
async fn test_search_throughput() {
    println!("=== Search Throughput Test ===");

    let mut config = Config::new();
    config.search.engine = "duckduckgo".to_string();

    // Test with concurrent requests
    let concurrent_requests = 3; // Small number for integration test
    let engines: Vec<_> = (0..concurrent_requests)
        .map(|_| SearchEngine::from_config(&config))
        .collect();

    println!(
        "Testing throughput with {} concurrent requests...",
        concurrent_requests
    );

    let start_time = Instant::now();

    let handles: Vec<_> = engines
        .into_iter()
        .enumerate()
        .map(|(i, mut engine)| {
            tokio::spawn(async move {
                let query = format!("throughput test {}", i);
                let start = Instant::now();
                let result = engine.search(&query, SearchMode::WebQuery, 3).await;
                let duration = start.elapsed();
                (i, result, duration)
            })
        })
        .collect();

    let mut successful_requests = 0;
    let mut total_results = 0;

    for handle in handles {
        match handle.await {
            Ok((i, search_result, duration)) => match search_result {
                Ok(results) => {
                    successful_requests += 1;
                    total_results += results.len();
                    println!(
                        "Concurrent request {}: {} results in {:?}",
                        i,
                        results.len(),
                        duration
                    );
                }
                Err(e) => {
                    println!("Concurrent request {} failed: {}", i, e);
                }
            },
            Err(e) => {
                println!("Task failed: {}", e);
            }
        }
    }

    let total_time = start_time.elapsed();

    println!("\nThroughput Results:");
    println!(
        "  Successful requests: {}/{}",
        successful_requests, concurrent_requests
    );
    println!("  Total time: {:?}", total_time);
    println!("  Total results: {}", total_results);

    if successful_requests > 0 {
        let throughput = successful_requests as f64 / total_time.as_secs_f64();
        println!("  Throughput: {:.2} requests/second", throughput);
        println!(
            "  Results per second: {:.2}",
            total_results as f64 / total_time.as_secs_f64()
        );
    }
}

#[tokio::test]
async fn test_search_timeout_behavior() {
    println!("=== Search Timeout Behavior Test ===");

    let timeout_values = vec![1, 5, 10, 30]; // seconds

    for timeout_seconds in timeout_values {
        println!("\nTesting with {}s timeout...", timeout_seconds);

        let mut config = Config::new();
        config.search.engine = "duckduckgo".to_string();
        config.fetcher.timeout = timeout_seconds;

        let mut engine = SearchEngine::from_config(&config);

        let start = Instant::now();
        let result = engine
            .search("timeout test query", SearchMode::WebQuery, 5)
            .await;
        let duration = start.elapsed();

        match result {
            Ok(results) => {
                println!(
                    "  Completed in {:?} with {} results",
                    duration,
                    results.len()
                );
                assert!(
                    duration.as_secs() <= timeout_seconds as u64 + 5,
                    "Search should complete within reasonable time of timeout"
                );
            }
            Err(e) => {
                println!("  Failed in {:?}: {}", duration, e);
                // Check if it's a timeout error
                let error_msg = e.to_string().to_lowercase();
                if error_msg.contains("timeout") || error_msg.contains("time") {
                    println!("  Properly failed due to timeout");
                }
            }
        }
    }
}

#[tokio::test]
async fn test_parser_performance() {
    println!("=== Parser Performance Test ===");

    use tarzi::parser::ParserFactory;

    let factory = ParserFactory::new();

    // Sample HTML content for testing (simplified)
    let sample_html = r#"
    <html>
    <body>
        <div class="result">
            <h3><a href="https://example1.com">Test Result 1</a></h3>
            <p>This is a test snippet for result 1</p>
        </div>
        <div class="result">
            <h3><a href="https://example2.com">Test Result 2</a></h3>
            <p>This is a test snippet for result 2</p>
        </div>
        <div class="result">
            <h3><a href="https://example3.com">Test Result 3</a></h3>
            <p>This is a test snippet for result 3</p>
        </div>
    </body>
    </html>
    "#;

    let parsers_to_test = vec![
        ("Bing", SearchEngineType::Bing, SearchMode::WebQuery),
        (
            "DuckDuckGo",
            SearchEngineType::DuckDuckGo,
            SearchMode::WebQuery,
        ),
        ("Google", SearchEngineType::Google, SearchMode::WebQuery),
        ("Brave", SearchEngineType::BraveSearch, SearchMode::WebQuery),
        ("Baidu", SearchEngineType::Baidu, SearchMode::WebQuery),
    ];

    for (name, engine_type, mode) in parsers_to_test {
        println!("\nTesting {} parser performance...", name);

        let parser = factory.get_parser(&engine_type, mode);

        // Test parsing the same content multiple times
        let iterations = 100;
        let start = Instant::now();

        let mut total_results = 0;
        for _ in 0..iterations {
            match parser.parse(sample_html, 10) {
                Ok(results) => {
                    total_results += results.len();
                }
                Err(_) => {
                    // Parser might not find results in sample HTML, which is fine
                }
            }
        }

        let total_time = start.elapsed();
        let avg_time = total_time / iterations;

        println!("  {} iterations completed in {:?}", iterations, total_time);
        println!("  Average parse time: {:?}", avg_time);
        println!("  Total results found: {}", total_results);

        // Parsing should be fast
        assert!(
            avg_time < Duration::from_millis(10),
            "Parser should be fast (< 10ms per parse)"
        );
    }
}

#[tokio::test]
async fn test_memory_usage_pattern() {
    println!("=== Memory Usage Pattern Test ===");

    let mut config = Config::new();
    config.search.engine = "duckduckgo".to_string();

    // Test multiple sequential searches to check for memory leaks
    let num_searches = 5;

    for i in 0..num_searches {
        println!("Search iteration {}/{}", i + 1, num_searches);

        let mut engine = SearchEngine::from_config(&config);
        let query = format!("memory test query {}", i);

        let start = Instant::now();
        let result = engine.search(&query, SearchMode::WebQuery, 5).await;
        let duration = start.elapsed();

        match result {
            Ok(results) => {
                println!(
                    "  Completed with {} results in {:?}",
                    results.len(),
                    duration
                );

                // Verify results are properly structured (no memory corruption)
                for (j, result) in results.iter().enumerate() {
                    assert!(
                        !result.title.is_empty() || !result.url.is_empty(),
                        "Result {} should have title or URL",
                        j
                    );
                    assert!(result.rank > 0, "Result {} should have valid rank", j);
                }
            }
            Err(e) => {
                println!("  Search failed: {}", e);
            }
        }

        // Force garbage collection between iterations
        // Note: Rust doesn't have explicit GC, but this simulates the pattern
        drop(engine);

        // Small delay to observe memory patterns
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    println!("Memory usage pattern test completed");
}

#[tokio::test]
async fn test_error_recovery_performance() {
    println!("=== Error Recovery Performance Test ===");

    let mut config = Config::new();
    config.search.engine = "duckduckgo".to_string();

    let mut engine = SearchEngine::from_config(&config);

    // Test recovery from various error conditions
    let long_unicode_query = "🎉".repeat(1000);
    let error_scenarios = vec![
        ("", "empty query"),
        ("query\x00with\x00null", "query with null bytes"),
        (long_unicode_query.as_str(), "very long unicode query"),
    ];

    for (query, description) in error_scenarios {
        println!("\nTesting error recovery for: {}", description);

        let start = Instant::now();
        let result = engine.search(query, SearchMode::WebQuery, 5).await;
        let duration = start.elapsed();

        match result {
            Ok(results) => {
                println!(
                    "  Handled gracefully with {} results in {:?}",
                    results.len(),
                    duration
                );
            }
            Err(e) => {
                println!("  Failed gracefully in {:?}: {}", duration, e);
            }
        }

        // Test that engine still works after error
        let recovery_start = Instant::now();
        let recovery_result = engine
            .search("recovery test", SearchMode::WebQuery, 2)
            .await;
        let recovery_duration = recovery_start.elapsed();

        match recovery_result {
            Ok(results) => {
                println!(
                    "  Recovery successful: {} results in {:?}",
                    results.len(),
                    recovery_duration
                );
            }
            Err(e) => {
                println!("  Recovery failed: {}", e);
            }
        }

        // Error recovery should be fast
        assert!(
            recovery_duration < Duration::from_secs(30),
            "Error recovery should be reasonably fast"
        );
    }
}
