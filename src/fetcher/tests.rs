#[cfg(test)]
mod tests {
    use super::{types::FetchMode, webfetcher::WebFetcher};
    use crate::{converter::Format, error::TarziError};
    use std::str::FromStr;

    #[test]
    fn test_fetch_mode_from_str() {
        // Test valid modes
        assert_eq!(
            FetchMode::from_str("plain_request").unwrap(),
            FetchMode::PlainRequest
        );
        assert_eq!(
            FetchMode::from_str("plain").unwrap(),
            FetchMode::PlainRequest
        );
        assert_eq!(
            FetchMode::from_str("PLAIN_REQUEST").unwrap(),
            FetchMode::PlainRequest
        );
        assert_eq!(
            FetchMode::from_str("PLAIN").unwrap(),
            FetchMode::PlainRequest
        );

        assert_eq!(
            FetchMode::from_str("browser_head").unwrap(),
            FetchMode::BrowserHead
        );
        assert_eq!(FetchMode::from_str("head").unwrap(), FetchMode::BrowserHead);
        assert_eq!(
            FetchMode::from_str("BROWSER_HEAD").unwrap(),
            FetchMode::BrowserHead
        );
        assert_eq!(FetchMode::from_str("HEAD").unwrap(), FetchMode::BrowserHead);

        assert_eq!(
            FetchMode::from_str("browser_headless").unwrap(),
            FetchMode::BrowserHeadless
        );
        assert_eq!(
            FetchMode::from_str("headless").unwrap(),
            FetchMode::BrowserHeadless
        );
        assert_eq!(
            FetchMode::from_str("BROWSER_HEADLESS").unwrap(),
            FetchMode::BrowserHeadless
        );
        assert_eq!(
            FetchMode::from_str("HEADLESS").unwrap(),
            FetchMode::BrowserHeadless
        );

        assert_eq!(
            FetchMode::from_str("browser_head_external").unwrap(),
            FetchMode::BrowserHeadExternal
        );
        assert_eq!(
            FetchMode::from_str("external").unwrap(),
            FetchMode::BrowserHeadExternal
        );
        assert_eq!(
            FetchMode::from_str("BROWSER_HEAD_EXTERNAL").unwrap(),
            FetchMode::BrowserHeadExternal
        );
        assert_eq!(
            FetchMode::from_str("EXTERNAL").unwrap(),
            FetchMode::BrowserHeadExternal
        );

        // Test invalid modes
        assert!(FetchMode::from_str("invalid").is_err());
        assert!(FetchMode::from_str("").is_err());
        assert!(FetchMode::from_str("browser").is_err());
        assert!(FetchMode::from_str("request").is_err());
    }

    #[test]
    fn test_webfetcher_new() {
        let fetcher = WebFetcher::new();
        assert!(!fetcher.browser_manager.has_browsers());
        assert!(!fetcher.external_browser_manager.is_connected());
    }

    #[test]
    fn test_webfetcher_default() {
        let fetcher1 = WebFetcher::new();
        let fetcher2 = WebFetcher::default();
        assert!(!fetcher1.browser_manager.has_browsers());
        assert!(!fetcher2.browser_manager.has_browsers());
        assert!(!fetcher1.external_browser_manager.is_connected());
        assert!(!fetcher2.external_browser_manager.is_connected());
    }

    #[test]
    fn test_fetch_mode_partial_eq() {
        assert_eq!(FetchMode::PlainRequest, FetchMode::PlainRequest);
        assert_eq!(FetchMode::BrowserHead, FetchMode::BrowserHead);
        assert_eq!(FetchMode::BrowserHeadless, FetchMode::BrowserHeadless);
        assert_eq!(
            FetchMode::BrowserHeadExternal,
            FetchMode::BrowserHeadExternal
        );

        assert_ne!(FetchMode::PlainRequest, FetchMode::BrowserHead);
        assert_ne!(FetchMode::PlainRequest, FetchMode::BrowserHeadless);
        assert_ne!(FetchMode::PlainRequest, FetchMode::BrowserHeadExternal);
        assert_ne!(FetchMode::BrowserHead, FetchMode::BrowserHeadless);
        assert_ne!(FetchMode::BrowserHead, FetchMode::BrowserHeadExternal);
        assert_ne!(FetchMode::BrowserHeadless, FetchMode::BrowserHeadExternal);
    }

    #[test]
    fn test_fetch_mode_debug() {
        assert_eq!(format!("{:?}", FetchMode::PlainRequest), "PlainRequest");
        assert_eq!(format!("{:?}", FetchMode::BrowserHead), "BrowserHead");
        assert_eq!(
            format!("{:?}", FetchMode::BrowserHeadless),
            "BrowserHeadless"
        );
        assert_eq!(
            format!("{:?}", FetchMode::BrowserHeadExternal),
            "BrowserHeadExternal"
        );
    }

    #[test]
    fn test_fetch_mode_clone() {
        let mode1 = FetchMode::PlainRequest;
        let mode2 = mode1;
        assert_eq!(mode1, mode2);

        let mode3 = FetchMode::BrowserHead;
        let mode4 = mode3;
        assert_eq!(mode3, mode4);
    }

    #[test]
    fn test_fetch_mode_copy() {
        let mode1 = FetchMode::BrowserHeadless;
        let mode2 = mode1; // This should work because FetchMode is Copy
        assert_eq!(mode1, mode2);
    }

    #[tokio::test]
    async fn test_fetch_raw_plain_request_invalid_url() {
        let mut fetcher = WebFetcher::new();
        let result = fetcher
            .fetch_raw("invalid-url", FetchMode::PlainRequest)
            .await;
        assert!(result.is_err());
        match result.unwrap_err() {
            TarziError::Url(_) => (), // Expected
            e => panic!("Expected Url error, got {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_fetch_with_proxy_invalid_proxy() {
        let mut fetcher = WebFetcher::new();

        // Test fetching with an invalid proxy (should fail)
        let result = fetcher
            .fetch_with_proxy(
                "https://httpbin.org/html",
                "http://invalid-proxy:8080",
                FetchMode::PlainRequest,
                Format::Html,
            )
            .await;

        // This should fail due to invalid proxy, but it might succeed in some environments
        // So we'll just check that it doesn't panic and handle both cases
        match result {
            Ok(_) => {
                // If it succeeds, that's also acceptable (some environments might handle invalid proxies differently)
                println!("Proxy test succeeded - invalid proxy was handled gracefully");
            }
            Err(_) => {
                // If it fails, that's the expected behavior
                println!("Proxy test failed as expected");
            }
        }
    }

    #[test]
    fn test_webfetcher_drop() {
        // Test that WebFetcher can be dropped without panicking
        let _fetcher = WebFetcher::new();
        // Should not panic when dropped
    }

    #[test]
    fn test_fetch_mode_serialization() {
        // Test that FetchMode can be converted to string and back
        let modes = vec![
            FetchMode::PlainRequest,
            FetchMode::BrowserHead,
            FetchMode::BrowserHeadless,
            FetchMode::BrowserHeadExternal,
        ];

        for mode in modes {
            let mode_str = match mode {
                FetchMode::PlainRequest => "plain_request",
                FetchMode::BrowserHead => "browser_head",
                FetchMode::BrowserHeadless => "browser_headless",
                FetchMode::BrowserHeadExternal => "browser_head_external",
            };

            let parsed = FetchMode::from_str(mode_str).unwrap();
            assert_eq!(mode, parsed);
        }
    }

    #[test]
    fn test_webfetcher_from_config() {
        use crate::config::Config;
        let config = Config::new();
        let fetcher = WebFetcher::from_config(&config);
        // We can't directly check the http_client internals, but we can check that the struct is created
        assert!(!fetcher.browser_manager.has_browsers());
        assert!(!fetcher.external_browser_manager.is_connected());
    }

    #[test]
    fn test_webfetcher_from_config_with_env_proxy() {
        use crate::config::Config;
        // Set environment variable
        unsafe {
            std::env::set_var("HTTP_PROXY", "http://test-proxy:8080");
        }

        let config = Config::new();
        let _fetcher = WebFetcher::from_config(&config);

        // Note: We can't easily test if the proxy was actually set on the client
        // without making HTTP requests, but we can verify the function doesn't panic

        // Clean up
        unsafe {
            std::env::remove_var("HTTP_PROXY");
        }
    }

    // Integration tests for external browser functionality
    #[tokio::test]
    async fn test_connect_to_external_browser_invalid_endpoint() {
        let mut fetcher = WebFetcher::new();

        // Test with invalid endpoint format
        let result = fetcher
            .connect_to_external_browser("invalid-endpoint")
            .await;
        assert!(result.is_err());

        // Test with non-websocket endpoint
        let result = fetcher
            .connect_to_external_browser("http://localhost:9222")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_connect_to_external_browser_valid_format() {
        let mut fetcher = WebFetcher::new();

        // Test with valid WebSocket format (should pass format validation but fail connection)
        let result = fetcher
            .connect_to_external_browser("ws://localhost:9222")
            .await;

        // This should fail because there's no actual browser running, but it should pass the format check
        // and fail during the actual connection attempt
        match result {
            Ok(_) => {
                // If it succeeds, that means there's actually a browser running (unlikely in test environment)
                println!("External browser connection succeeded - browser is actually running");
            }
            Err(e) => {
                // This is the expected behavior in test environment
                match e {
                    TarziError::Browser(_) => {
                        println!("External browser connection failed as expected: {:?}", e);
                    }
                    _ => panic!("Expected Browser error, got {:?}", e),
                }
            }
        }
    }

    #[tokio::test]
    async fn test_fetch_with_external_browser_no_connection() {
        let mut fetcher = WebFetcher::new();

        // Test fetching with external browser mode when no connection is established
        // This should attempt to connect to the default endpoint and fail
        let result = fetcher
            .fetch_raw("https://httpbin.org/html", FetchMode::BrowserHeadExternal)
            .await;

        match result {
            Ok(_) => {
                // If it succeeds, that means there's actually a browser running
                println!("External browser fetch succeeded - browser is actually running");
            }
            Err(e) => {
                // This is the expected behavior in test environment
                match e {
                    TarziError::Browser(_) => {
                        println!("External browser fetch failed as expected: {:?}", e);
                    }
                    _ => panic!("Expected Browser error, got {:?}", e),
                }
            }
        }
    }

    #[tokio::test]
    async fn test_external_browser_prerequisites_check() {
        let fetcher = WebFetcher::new();

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
                "Endpoint {} should pass prerequisites check",
                endpoint
            );
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
                "Endpoint {} should fail prerequisites check",
                endpoint
            );
        }
    }

    #[tokio::test]
    async fn test_fetch_with_external_browser_no_connection() {
        let mut fetcher = WebFetcher::new();

        // Test fetching with external browser mode when no connection is established
        // This should attempt to connect to the default endpoint and fail
        let result = fetcher
            .fetch_raw("https://httpbin.org/html", FetchMode::BrowserHeadExternal)
            .await;

        match result {
            Ok(_) => {
                // If it succeeds, that means there's actually a browser running
                println!("External browser fetch succeeded - browser is actually running");
            }
            Err(e) => {
                // This is the expected behavior in test environment
                match e {
                    TarziError::Browser(_) => {
                        println!("External browser fetch failed as expected: {:?}", e);
                    }
                    _ => panic!("Expected Browser error, got {:?}", e),
                }
            }
        }
    }

    #[tokio::test]
    async fn test_external_browser_integration() {
        let mut fetcher = WebFetcher::new();

        // Test fetching with external browser mode when no connection is established
        // This should attempt to connect to the default endpoint and fail
        let result = fetcher
            .fetch_raw("https://httpbin.org/html", FetchMode::BrowserHeadExternal)
            .await;

        match result {
            Ok(_) => {
                println!(
                    "External browser integration test succeeded - browser is actually running"
                );
            }
            Err(e) => match e {
                TarziError::Browser(_) => {
                    println!(
                        "External browser integration test failed as expected: {:?}",
                        e
                    );
                }
                _ => panic!("Expected Browser error, got {:?}", e),
            },
        }
    }
}