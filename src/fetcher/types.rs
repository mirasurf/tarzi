use crate::constants::{
    FETCHER_MODE_BROWSER_HEADLESS, FETCHER_MODE_HEAD, FETCHER_MODE_PLAIN_REQUEST,
};
use crate::error::TarziError;

/// Different modes for fetching web content
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FetchMode {
    /// Fetch content using plain HTTP request (no JavaScript rendering)
    PlainRequest,
    /// Fetch content using browser with visible window
    BrowserHead,
    /// Fetch content using browser in headless mode
    BrowserHeadless,
}

impl std::str::FromStr for FetchMode {
    type Err = TarziError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            FETCHER_MODE_PLAIN_REQUEST | "plain" => Ok(FetchMode::PlainRequest),
            FETCHER_MODE_HEAD | "browser_head" => Ok(FetchMode::BrowserHead),
            FETCHER_MODE_BROWSER_HEADLESS => Ok(FetchMode::BrowserHeadless),
            _ => Err(TarziError::InvalidMode(s.to_string())),
        }
    }
}

/// Main WebFetcher type alias for backward compatibility
pub type WebFetcher = crate::fetcher::webfetcher::WebFetcher;
