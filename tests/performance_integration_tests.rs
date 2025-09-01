use std::time::{Duration, Instant};
use tarzi::config::Config;
use tarzi::search::SearchEngine;
use tarzi::search::types::SearchEngineType;

/// Performance and load testing for search engines
/// These tests focus on measuring response times and throughput

#[tokio::test]
async fn test_parser_performance() {
    println!("=== Testing Parser Performance ===");

    let factory = tarzi::search::parser::ParserFactory::new();

    let test_cases = vec![
        ("Bing", SearchEngineType::Bing),
        ("DuckDuckGo", SearchEngineType::DuckDuckGo),
        ("Google", SearchEngineType::Google),
        ("Brave", SearchEngineType::BraveSearch),
        ("Baidu", SearchEngineType::Baidu),
    ];

    for (name, engine_type) in test_cases {
        println!("\nTesting {name} parser performance...");

        let parser = factory.get_parser(&engine_type);
        let test_html = format!(
            r#"<html><body>
                <h2><a href="https://example1.com">Test Result 1</a></h2>
                <p>Test snippet 1</p>
                <h2><a href="https://example2.com">Test Result 2</a></h2>
                <p>Test snippet 2</p>
                <h2><a href="https://example3.com">Test Result 3</a></h2>
                <p>Test snippet 3</p>
            </body></html>"#
        );

        let iterations = 100;
        let mut total_time = Duration::new(0, 0);

        for _ in 0..iterations {
            let start = Instant::now();
            let _results = parser.parse(&test_html, 5);
            let duration = start.elapsed();
            total_time += duration;
        }

        let avg_time = total_time / iterations;
        println!("  Average parse time: {:?}", avg_time);
        println!(
            "  Total time for {} iterations: {:?}",
            iterations, total_time
        );
    }
}

#[tokio::test]
async fn test_search_throughput() {
    println!("=== Testing Search Throughput ===");

    let mut config = Config::new();
    config.search.engine = "duckduckgo".to_string();

    let mut engine = SearchEngine::from_config(&config);

    let test_queries = vec![
        "rust programming",
        "python tutorial",
        "machine learning",
        "web development",
        "data science",
    ];

    let mut total_time = Duration::new(0, 0);
    let mut successful_queries = 0;

    for query in &test_queries {
        let start = Instant::now();
        let result = engine.search(query, 3).await;
        let duration = start.elapsed();

        match result {
            Ok(results) => {
                successful_queries += 1;
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
            "\n  Summary: {}/{} queries successful",
            successful_queries,
            test_queries.len()
        );
        println!("  Average response time: {:?}", avg_time);
        println!("  Total time: {:?}", total_time);
    } else {
        println!("  No successful queries");
    }
}

#[tokio::test]
async fn test_search_latency_percentiles() {
    println!("=== Testing Search Latency Percentiles ===");

    let mut config = Config::new();
    config.search.engine = "duckduckgo".to_string();

    let mut engine = SearchEngine::from_config(&config);
    let num_requests = 5; // Small number for integration test
    let mut latencies = Vec::new();

    println!(
        "Performing {} search requests to measure latency distribution...",
        num_requests
    );

    for i in 0..num_requests {
        let query = format!("test query {}", i);
        let start = Instant::now();
        let result = engine.search(&query, 2).await;
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
                println!("Request {}: failed in {:?} - {}", i + 1, duration, e);
            }
        }
    }

    if !latencies.is_empty() {
        latencies.sort();
        let p50 = latencies[latencies.len() / 2];
        let p90 = latencies[(latencies.len() * 9) / 10];
        let p95 = latencies[(latencies.len() * 19) / 20];

        println!("\nLatency percentiles:");
        println!("  P50 (median): {:?}", p50);
        println!("  P90: {:?}", p90);
        println!("  P95: {:?}", p95);
        println!("  Min: {:?}", latencies[0]);
        println!("  Max: {:?}", latencies[latencies.len() - 1]);
    }
}
