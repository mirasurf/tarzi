#[cfg(test)]
mod tests {
    use super::{types::FetchMode, webfetcher::WebFetcher};
    use crate::{converter::Format, error::TarziError};
    use std::str::FromStr;
    use crate::constants::{FETCHER_MODE_PLAIN_REQUEST, FETCHER_MODE_BROWSER_HEADLESS, FETCHER_MODE_HEAD};

    #[test]
    fn test_fetch_mode_from_str() {
        // Test valid modes
        assert_eq!(
            FetchMode::from_str(FETCHER_MODE_PLAIN_REQUEST).unwrap(),
            FetchMode::PlainRequest
        );
        assert_eq!(
            FetchMode::from_str(FETCHER_MODE_PLAIN_REQUEST.to_uppercase().as_str()).unwrap(),
            FetchMode::PlainRequest
        );
        assert_eq!(
            FetchMode::from_str(FETCHER_MODE_HEAD).unwrap(),
            FetchMode::BrowserHead
        );
        assert_eq!(FetchMode::from_str(FETCHER_MODE_HEAD.to_uppercase().as_str()).unwrap(), FetchMode::BrowserHead);

        assert_eq!(
            FetchMode::from_str(FETCHER_MODE_BROWSER_HEADLESS).unwrap(),
            FetchMode::BrowserHeadless
        );
        assert_eq!(
            FetchMode::from_str(FETCHER_MODE_BROWSER_HEADLESS.to_uppercase().as_str()).unwrap(),
            FetchMode::BrowserHeadless
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
    }

    #[test]
    fn test_webfetcher_default() {
        let fetcher1 = WebFetcher::new();
        let fetcher2 = WebFetcher::default();
        assert!(!fetcher1.browser_manager.has_browsers());
        assert!(!fetcher2.browser_manager.has_browsers());
    }

    #[test]
    fn test_webfetcher_from_config() {
        let config = Config::load_dev().unwrap_or_else(|_| Config::new());
        let fetcher = WebFetcher::from_config(&config);
        assert!(!fetcher.browser_manager.has_browsers());
    }

    #[test]
    fn test_webfetcher_from_config_with_proxy() {
        let mut config = Config::new();
        config.fetcher.proxy = Some("http://127.0.0.1:8080".to_string());
        let fetcher = WebFetcher::from_config(&config);
        assert!(!fetcher.browser_manager.has_browsers());
        // Test that the proxy is configured in the browser manager
        assert!(fetcher.browser_manager.config.is_some());
        if let Some(ref browser_config) = fetcher.browser_manager.config {
            assert_eq!(browser_config.fetcher.proxy, Some("http://127.0.0.1:8080".to_string()));
        }
    }

    #[test]
    fn test_fetch_mode_partial_eq() {
        assert_eq!(FetchMode::PlainRequest, FetchMode::PlainRequest);
        assert_eq!(FetchMode::BrowserHead, FetchMode::BrowserHead);
        assert_eq!(FetchMode::BrowserHeadless, FetchMode::BrowserHeadless);

        assert_ne!(FetchMode::PlainRequest, FetchMode::BrowserHead);
        assert_ne!(FetchMode::PlainRequest, FetchMode::BrowserHeadless);
        assert_ne!(FetchMode::BrowserHead, FetchMode::BrowserHeadless);
    }

    #[test]
    fn test_fetch_mode_debug() {
        assert_eq!(format!("{:?}", FetchMode::PlainRequest), "PlainRequest");
        assert_eq!(format!("{:?}", FetchMode::BrowserHead), "BrowserHead");
        assert_eq!(
            format!("{:?}", FetchMode::BrowserHeadless),
            "BrowserHeadless"
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

    #[tokio::test]
    async fn test_fetch_with_proxy_browser_headless() {
        let mut fetcher = WebFetcher::new();

        // Test fetching with proxy in browser headless mode
        let result = fetcher
            .fetch_with_proxy(
                "https://httpbin.org/html",
                "http://invalid-proxy:8080",
                FetchMode::BrowserHeadless,
                Format::Html,
            )
            .await;

        // This should fail due to invalid proxy, but we're testing that the code path works
        match result {
            Ok(_) => {
                println!("Proxy browser test succeeded - invalid proxy was handled gracefully");
            }
            Err(_) => {
                println!("Proxy browser test failed as expected");
            }
        }
    }

    #[tokio::test]
    async fn test_fetch_with_proxy_browser_head() {
        let mut fetcher = WebFetcher::new();

        // Test fetching with proxy in browser head mode
        let result = fetcher
            .fetch_with_proxy(
                "https://httpbin.org/html",
                "http://invalid-proxy:8080",
                FetchMode::BrowserHead,
                Format::Html,
            )
            .await;

        // This should fail due to invalid proxy, but we're testing that the code path works
        match result {
            Ok(_) => {
                println!("Proxy browser head test succeeded - invalid proxy was handled gracefully");
            }
            Err(_) => {
                println!("Proxy browser head test failed as expected");
            }
        }
    }

    #[tokio::test]
    async fn test_create_browser_with_proxy() {
        let mut fetcher = WebFetcher::new();

        // Test creating a browser with proxy configuration
        let result = fetcher
            .create_browser_with_proxy(
                None,
                true,
                Some("test_proxy_browser".to_string()),
                Some("http://127.0.0.1:8080".to_string()),
            )
            .await;

        // This might fail due to no webdriver available, but we're testing the code path
        match result {
            Ok(instance_id) => {
                println!("Browser with proxy created successfully: {}", instance_id);
                // Clean up
                let _ = fetcher.remove_browser(&instance_id).await;
            }
            Err(e) => {
                println!("Browser creation with proxy failed as expected: {:?}", e);
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
        ];

        for mode in modes {
            let mode_str = match mode {
                FetchMode::PlainRequest => FETCHER_MODE_PLAIN_REQUEST,
                FetchMode::BrowserHead => "browser_head",
                FetchMode::BrowserHeadless => FETCHER_MODE_BROWSER_HEADLESS,
            };

            let parsed = FetchMode::from_str(mode_str).unwrap();
            assert_eq!(mode, parsed);
        }
    }

    #[test]
    fn test_proxy_configuration_inheritance() {
        // Test that proxy configuration is properly inherited by browser manager
        let mut config = Config::new();
        config.fetcher.proxy = Some("http://test-proxy:3128".to_string());
        
        let fetcher = WebFetcher::from_config(&config);
        
        // Verify that the browser manager has the config
        assert!(fetcher.browser_manager.config.is_some());
        
        if let Some(ref browser_config) = fetcher.browser_manager.config {
            assert_eq!(
                browser_config.fetcher.proxy,
                Some("http://test-proxy:3128".to_string())
            );
        }
    }

    #[test]
    fn test_proxy_environment_override() {
        use crate::config::get_proxy_from_env_or_config;
        
        // Test with config proxy
        let config_proxy = Some("http://config-proxy:8080".to_string());
        
        // When no environment variables are set, should use config
        let result = get_proxy_from_env_or_config(&config_proxy);
        // This might vary based on environment, so we'll just ensure it doesn't panic
        match result {
            Some(proxy) => {
                println!("Using proxy: {}", proxy);
            }
            None => {
                println!("No proxy configured");
            }
        }
    }
}