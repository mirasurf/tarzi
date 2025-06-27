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
    /// Fetch content using external browser instance
    BrowserHeadExternal,
}

impl std::str::FromStr for FetchMode {
    type Err = TarziError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "plain_request" | "plain" => Ok(FetchMode::PlainRequest),
            "browser_head" | "head" => Ok(FetchMode::BrowserHead),
            "browser_headless" | "headless" => Ok(FetchMode::BrowserHeadless),
            "browser_head_external" | "external" => Ok(FetchMode::BrowserHeadExternal),
            _ => Err(TarziError::InvalidMode(s.to_string())),
        }
    }
}

/// Main WebFetcher type alias for backward compatibility
pub type WebFetcher = crate::fetcher::webfetcher::WebFetcher;
