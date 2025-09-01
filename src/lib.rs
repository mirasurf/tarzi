pub mod config;
pub mod constants;
pub mod converter;
pub mod error;
pub mod fetcher;
pub mod search;
pub mod utils;

#[cfg(feature = "pyo3")]
pub mod python;

pub use error::{Result, TarziError};

// Re-export main types for convenience
pub use converter::{Converter, Format};
pub use fetcher::{FetchMode, WebFetcher};
pub use search::{SearchEngine, SearchResult};

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_format_parsing() {
        assert_eq!(Format::from_str("markdown").unwrap(), Format::Markdown);
        assert_eq!(Format::from_str("json").unwrap(), Format::Json);
        assert_eq!(Format::from_str("yaml").unwrap(), Format::Yaml);
        assert_eq!(Format::from_str("html").unwrap(), Format::Html);
    }

    #[test]
    fn test_fetch_mode_parsing() {
        assert_eq!(
            FetchMode::from_str("plain_request").unwrap(),
            FetchMode::PlainRequest
        );
        assert_eq!(
            FetchMode::from_str("browser_head").unwrap(),
            FetchMode::BrowserHead
        );
        assert_eq!(
            FetchMode::from_str("browser_headless").unwrap(),
            FetchMode::BrowserHeadless
        );
    }

    #[test]
    fn test_modular_structure() {
        // Test that modules can be instantiated
        let _converter = Converter::new();
        let _fetcher = WebFetcher::new();
        let _search_engine = SearchEngine::new();

        // Test that types can be created
        let _format = Format::Markdown;
        let _fetch_mode = FetchMode::PlainRequest;
    }

    #[test]
    fn test_basic_functionality() {
        // Basic functionality test
    }
}
