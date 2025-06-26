pub mod converter;
pub mod fetcher;
pub mod search;
pub mod error;

#[cfg(feature = "pyo3")]
pub mod python;

pub use error::{TarsierError, Result};

// Re-export main types for convenience
pub use converter::{Converter, Format};
pub use fetcher::WebFetcher;
pub use search::{SearchEngine, SearchMode, SearchResult};

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_html_to_markdown_conversion() {
        let converter = Converter::new();
        let html = "<h1>Hello World</h1><p>This is a <strong>test</strong>.</p>";
        let result = converter.convert(html, Format::Markdown).await.unwrap();
        assert!(result.contains("Hello World"));
        assert!(result.contains("test"));
    }

    #[tokio::test]
    async fn test_html_to_json_conversion() {
        let converter = Converter::new();
        let html = "<h1>Test Title</h1><p>Test content</p>";
        let result = converter.convert(html, Format::Json).await.unwrap();
        assert!(result.contains("Test Title"));
        assert!(result.contains("Test content"));
    }

    #[test]
    fn test_format_parsing() {
        assert_eq!(Format::from_str("markdown").unwrap(), Format::Markdown);
        assert_eq!(Format::from_str("json").unwrap(), Format::Json);
        assert_eq!(Format::from_str("yaml").unwrap(), Format::Yaml);
        assert_eq!(Format::from_str("html").unwrap(), Format::Html);
    }

    #[test]
    fn test_search_mode_parsing() {
        assert_eq!(SearchMode::from_str("browser").unwrap(), SearchMode::Browser);
        assert_eq!(SearchMode::from_str("api").unwrap(), SearchMode::Api);
    }
} 