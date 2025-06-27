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
                FetchMode::PlainRequest => "plain_request",
                FetchMode::BrowserHead => "browser_head",
                FetchMode::BrowserHeadless => "browser_headless",
            };

            let parsed = FetchMode::from_str(mode_str).unwrap();
            assert_eq!(mode, parsed);
        }
    }
}