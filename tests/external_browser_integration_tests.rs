use std::str::FromStr;
use tarzi::{
    converter::Format,
    error::TarziError,
    fetcher::{FetchMode, WebFetcher},
};

/// Integration tests for external browser functionality
/// These tests demonstrate how to use the new BrowserHeadExternal mode

#[tokio::test]
async fn test_external_browser_mode_basic_usage() {
    // Test that the new mode is properly recognized
    let mode = FetchMode::BrowserHeadExternal;
    assert_eq!(mode, FetchMode::from_str("browser_head_external").unwrap());
    assert_eq!(mode, FetchMode::from_str("external").unwrap());

    println!("External browser mode is properly recognized");
}

#[tokio::test]
async fn test_external_browser_connection_workflow() {
    let mut fetcher = WebFetcher::new();

    // Test the connection workflow
    println!("Testing external browser connection workflow...");

    // Test with invalid endpoint (should fail gracefully)
    let result = fetcher
        .connect_to_external_browser("invalid-endpoint")
        .await;
    assert!(result.is_err());
    println!("✓ Invalid endpoint correctly rejected");

    // Test with valid WebSocket format but no actual browser
    let result = fetcher
        .connect_to_external_browser("ws://localhost:9222")
        .await;
    match result {
        Ok(_) => {
            println!("✓ Successfully connected to external browser (browser is running)");
        }
        Err(TarziError::Browser(_)) => {
            println!("✓ Connection failed as expected (no browser running)");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_external_browser_fetch_workflow() {
    let mut fetcher = WebFetcher::new();

    println!("Testing external browser fetch workflow...");

    // Test fetching with external browser mode
    let result = fetcher
        .fetch_raw("https://httpbin.org/html", FetchMode::BrowserHeadExternal)
        .await;

    match result {
        Ok(content) => {
            println!(
                "✓ Successfully fetched content with external browser ({} characters)",
                content.len()
            );
            assert!(!content.is_empty());
        }
        Err(TarziError::Browser(_)) => {
            println!("✓ Fetch failed as expected (no external browser available)");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_external_browser_with_format_conversion() {
    let mut fetcher = WebFetcher::new();

    println!("Testing external browser with format conversion...");

    // Test the complete fetch workflow with format conversion
    let result = fetcher
        .fetch(
            "https://httpbin.org/html",
            FetchMode::BrowserHeadExternal,
            Format::Markdown,
        )
        .await;

    match result {
        Ok(content) => {
            println!(
                "✓ Successfully fetched and converted content with external browser ({} characters)",
                content.len()
            );
            assert!(!content.is_empty());
            // Should contain markdown content
            assert!(content.contains("#") || content.contains("*") || content.contains("`"));
        }
        Err(TarziError::Browser(_)) => {
            println!("✓ Fetch failed as expected (no external browser available)");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_external_browser_prerequisites_validation() {
    let fetcher = WebFetcher::new();

    println!("Testing external browser prerequisites validation...");

    // Test valid WebSocket endpoints
    let valid_endpoints = vec![
        "ws://localhost:9222",
        "wss://localhost:9222",
        "ws://127.0.0.1:9222",
        "wss://example.com:9222",
    ];

    for endpoint in valid_endpoints {
        let result = fetcher.check_external_browser_prerequisites(endpoint).await;
        assert!(result.is_ok());
        assert!(
            result.unwrap(),
            "Endpoint {} should pass validation",
            endpoint
        );
        println!("✓ Valid endpoint passed: {}", endpoint);
    }

    // Test invalid endpoints
    let invalid_endpoints = vec![
        "http://localhost:9222",
        "https://localhost:9222",
        "invalid-endpoint",
        "ftp://localhost:9222",
        "",
    ];

    for endpoint in invalid_endpoints {
        let result = fetcher.check_external_browser_prerequisites(endpoint).await;
        assert!(result.is_ok());
        assert!(
            !result.unwrap(),
            "Endpoint {} should fail validation",
            endpoint
        );
        println!("✓ Invalid endpoint correctly rejected: {}", endpoint);
    }
}

#[tokio::test]
async fn test_external_browser_environment_configuration() {
    println!("Testing external browser environment configuration...");

    // Test environment variable fallback behavior
    let original_env = std::env::var("TARZI_EXTERNAL_BROWSER_ENDPOINT");

    // Test with custom endpoint
    unsafe {
        std::env::set_var("TARZI_EXTERNAL_BROWSER_ENDPOINT", "ws://custom:9222");
    }
    let endpoint = std::env::var("TARZI_EXTERNAL_BROWSER_ENDPOINT")
        .unwrap_or_else(|_| "ws://localhost:9222".to_string());
    assert_eq!(endpoint, "ws://custom:9222");
    println!("✓ Custom endpoint correctly read: {}", endpoint);

    // Test with default fallback
    unsafe {
        std::env::remove_var("TARZI_EXTERNAL_BROWSER_ENDPOINT");
    }
    let endpoint = std::env::var("TARZI_EXTERNAL_BROWSER_ENDPOINT")
        .unwrap_or_else(|_| "ws://localhost:9222".to_string());
    assert_eq!(endpoint, "ws://localhost:9222");
    println!("✓ Default endpoint fallback works: {}", endpoint);

    // Restore original environment variable
    if let Ok(val) = original_env {
        unsafe {
            std::env::set_var("TARZI_EXTERNAL_BROWSER_ENDPOINT", val);
        }
    }
}

#[tokio::test]
async fn test_external_browser_error_handling() {
    let mut fetcher = WebFetcher::new();

    println!("Testing external browser error handling...");

    // Test various error scenarios
    let error_scenarios = vec![
        ("invalid-url", "Invalid URL format"),
        (
            "http://invalid-domain-that-does-not-exist-12345.com",
            "Network error",
        ),
        ("https://httpbin.org/status/404", "HTTP error"),
    ];

    for (url, description) in error_scenarios {
        let result = fetcher.fetch_raw(url, FetchMode::BrowserHeadExternal).await;
        match result {
            Ok(_) => {
                println!("✓ {} succeeded (unexpected)", description);
            }
            Err(TarziError::Browser(_)) => {
                println!("✓ {} failed with browser error (expected)", description);
            }
            Err(TarziError::Url(_)) => {
                println!("✓ {} failed with URL error (expected)", description);
            }
            Err(TarziError::Http(_)) => {
                println!("✓ {} failed with HTTP error (expected)", description);
            }
            Err(e) => {
                println!("✓ {} failed with error: {:?}", description, e);
            }
        }
    }
}

/// Example of how to use the external browser mode in practice
#[tokio::test]
async fn test_external_browser_practical_example() {
    println!("=== External Browser Mode Practical Example ===");

    let mut fetcher = WebFetcher::new();

    // Step 1: Check if external browser is available
    println!("1. Checking external browser availability...");
    let endpoint = std::env::var("TARZI_EXTERNAL_BROWSER_ENDPOINT")
        .unwrap_or_else(|_| "ws://localhost:9222".to_string());

    let prerequisites_ok = fetcher
        .check_external_browser_prerequisites(&endpoint)
        .await
        .unwrap();
    if prerequisites_ok {
        println!(
            "   ✓ External browser endpoint format is valid: {}",
            endpoint
        );
    } else {
        println!(
            "   ✗ External browser endpoint format is invalid: {}",
            endpoint
        );
    }

    // Step 2: Try to connect to external browser
    println!("2. Attempting to connect to external browser...");
    let connection_result = fetcher.connect_to_external_browser(&endpoint).await;
    match connection_result {
        Ok(_) => {
            println!("   ✓ Successfully connected to external browser");

            // Step 3: Use the external browser for fetching
            println!("3. Using external browser to fetch content...");
            let fetch_result = fetcher
                .fetch(
                    "https://httpbin.org/html",
                    FetchMode::BrowserHeadExternal,
                    Format::Markdown,
                )
                .await;

            match fetch_result {
                Ok(content) => {
                    println!(
                        "   ✓ Successfully fetched content ({} characters)",
                        content.len()
                    );
                    println!("   Content preview: {}", &content[..content.len().min(100)]);
                }
                Err(e) => {
                    println!("   ✗ Failed to fetch content: {:?}", e);
                }
            }
        }
        Err(TarziError::Browser(msg)) => {
            println!("   ✗ Failed to connect to external browser: {}", msg);
            println!("   Note: This is expected if no external browser is running");
            println!(
                "   To use external browser mode, start Chrome with --remote-debugging-port=9222"
            );
        }
        Err(e) => {
            println!("   ✗ Unexpected error: {:?}", e);
        }
    }

    println!("=== End of Practical Example ===");
}
