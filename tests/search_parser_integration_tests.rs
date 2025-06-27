use std::time::Duration;
use tarzi::search::parser::{BingParser, SearchResultParser};
use tarzi::search::types::SearchEngineType;
use tarzi::utils::is_webdriver_available;
use thirtyfour::prelude::*;
use thirtyfour::{By, DesiredCapabilities, Key, WebDriver};

/// Integration tests for search parsers
/// These tests require internet access and a running WebDriver server

/// Perform a real Bing search using WebDriver and return the HTML
async fn perform_bing_search(query: &str) -> Result<String, Box<dyn std::error::Error>> {
    let webdriver_url = std::env::var("TARZI_WEBDRIVER_URL")
        .unwrap_or_else(|_| "http://localhost:4444".to_string());

    // Setup Chrome capabilities for head browser
    let mut caps = DesiredCapabilities::chrome();
    caps.add_arg("--disable-blink-features=AutomationControlled")?;
    caps.add_arg("--disable-web-security")?;
    caps.add_arg("--disable-features=VizDisplayCompositor")?;

    // Connect to WebDriver
    let driver = WebDriver::new(&webdriver_url, caps).await?;

    let result = async {
        // Navigate to Bing
        driver.goto("https://www.bing.com").await?;
        println!("Navigated to Bing homepage");

        // Wait a moment for the page to load
        tokio::time::sleep(Duration::from_millis(1000)).await;

        // Find search box and enter query
        let search_box = driver.find(By::Name("q")).await?;
        search_box.clear().await?;
        search_box.send_keys(query).await?;
        search_box.send_keys(Key::Enter).await?;
        println!("Submitted search query: '{}'", query);

        // Wait for search results to load
        let mut search_results_loaded = false;
        for _ in 0..20 {
            tokio::time::sleep(Duration::from_millis(500)).await;
            match driver.find_all(By::Css("li.b_algo")).await {
                Ok(elements) if !elements.is_empty() => {
                    println!("Search results loaded successfully");
                    search_results_loaded = true;
                    break;
                }
                _ => continue,
            }
        }

        if !search_results_loaded {
            println!("Warning: Search results did not load within timeout, trying to get page source anyway");
        }

        // Additional wait to ensure all results are loaded
        tokio::time::sleep(Duration::from_millis(2000)).await;

        // Get page source
        let page_source = driver.source().await?;
        println!("Retrieved page source, length: {} characters", page_source.len());

        Ok::<String, Box<dyn std::error::Error>>(page_source)
    }.await;

    // Always quit the driver
    if let Err(e) = driver.quit().await {
        eprintln!("Warning: Failed to quit WebDriver: {}", e);
    }

    result
}

#[tokio::test]
async fn test_bing_parser_real_world_integration() {
    // Skip test if WebDriver is not available
    if !is_webdriver_available().await {
        println!("Skipping real-world integration test: WebDriver not available");
        println!("To run this test, start a WebDriver server (e.g., chromedriver on port 4444)");
        return;
    }

    println!("Starting real-world Bing search integration test...");

    // Perform a real search
    let search_query = "mirasurf";
    let html_content = match perform_bing_search(search_query).await {
        Ok(html) => html,
        Err(e) => {
            eprintln!("Failed to perform Bing search: {}", e);
            panic!("Integration test failed: {}", e);
        }
    };

    // Verify we got actual Bing HTML
    assert!(html_content.contains("bing.com"));
    assert!(html_content.len() > 10000); // Bing pages are typically quite large
    println!("✓ Successfully retrieved Bing search results HTML");

    // Create BingParser and parse the results
    let parser = BingParser::new();
    assert_eq!(parser.name(), "BingParser");
    assert!(parser.supports(&SearchEngineType::Bing));
    println!("✓ BingParser created and validated");

    // Parse the real HTML
    let limit = 5;
    let results = match parser.parse(&html_content, limit) {
        Ok(results) => results,
        Err(e) => {
            eprintln!("Failed to parse Bing HTML: {}", e);
            // Save HTML for debugging if needed
            if std::env::var("TARZI_DEBUG").is_ok() {
                std::fs::write("debug_bing.html", &html_content).ok();
                println!("Debug HTML saved to debug_bing.html");
            }
            panic!("Parser failed: {}", e);
        }
    };

    println!("✓ Successfully parsed {} search results", results.len());

    // Validate the parsed results
    assert!(!results.is_empty(), "Should have found some search results");
    assert!(
        results.len() <= limit,
        "Should not exceed the requested limit"
    );

    // Validate each result structure
    for (i, result) in results.iter().enumerate() {
        println!("Result {}: {} - {}", i + 1, result.title, result.url);

        // Basic validation
        assert!(!result.title.is_empty(), "Title should not be empty");
        assert!(!result.url.is_empty(), "URL should not be empty");
        assert_eq!(result.rank, i + 1, "Rank should be sequential");

        // URL validation
        assert!(
            result.url.starts_with("http://") || result.url.starts_with("https://"),
            "URL should be properly formatted: {}",
            result.url
        );

        // Content validation (for "rust programming language" search)
        let lower_title = result.title.to_lowercase();
        let lower_snippet = result.snippet.to_lowercase();
        let contains_rust = lower_title.contains("rust")
            || lower_snippet.contains("rust")
            || lower_title.contains("programming")
            || lower_snippet.contains("programming");

        if !contains_rust {
            println!(
                "Warning: Result {} may not be relevant to search query",
                i + 1
            );
        }
    }

    println!("✓ All results validated successfully");

    // Test with different limits
    let small_limit = 2;
    let small_results = parser.parse(&html_content, small_limit).unwrap();
    assert!(small_results.len() <= small_limit);
    assert!(small_results.len() <= results.len());
    println!("✓ Parser correctly handles different limits");

    // Test with empty HTML
    let empty_results = parser.parse("", 5).unwrap();
    assert!(empty_results.is_empty());
    println!("✓ Parser correctly handles empty HTML");

    println!("🎉 Real-world Bing parser integration test completed successfully!");
}

#[tokio::test]
async fn test_bing_parser_with_different_queries() {
    // Skip test if WebDriver is not available
    if !is_webdriver_available().await {
        println!("Skipping multi-query integration test: WebDriver not available");
        return;
    }

    let test_queries = vec![
        "tokio async rust",
        "web scraping",
        "machine learning python",
    ];

    let parser = BingParser::new();

    for query in test_queries {
        println!("Testing query: '{}'", query);

        match perform_bing_search(query).await {
            Ok(html) => {
                let results = parser.parse(&html, 3).unwrap();
                println!("  Found {} results for '{}'", results.len(), query);

                // Validate results
                for result in results {
                    assert!(!result.title.is_empty());
                    assert!(!result.url.is_empty());
                    assert!(result.url.starts_with("http"));
                }
            }
            Err(e) => {
                println!("  Warning: Search failed for '{}': {}", query, e);
            }
        }

        // Add delay between searches to be respectful
        tokio::time::sleep(Duration::from_millis(2000)).await;
    }

    println!("✓ Multi-query test completed");
}

#[tokio::test]
async fn test_bing_parser_performance() {
    // Skip test if WebDriver is not available
    if !is_webdriver_available().await {
        println!("Skipping performance test: WebDriver not available");
        return;
    }

    println!("Testing BingParser performance...");

    // Get HTML once
    let html = match perform_bing_search("performance test").await {
        Ok(html) => html,
        Err(e) => {
            println!("Skipping performance test: {}", e);
            return;
        }
    };

    let parser = BingParser::new();

    // Test parsing performance
    let start_time = std::time::Instant::now();
    let iterations = 10;

    for _ in 0..iterations {
        let _results = parser.parse(&html, 10).unwrap();
    }

    let elapsed = start_time.elapsed();
    let avg_time = elapsed / iterations;

    println!("Average parsing time: {:?}", avg_time);
    assert!(
        avg_time < Duration::from_millis(500),
        "Parsing should be reasonably fast"
    );

    println!("✓ Performance test completed");
}
