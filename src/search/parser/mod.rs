use crate::Result;
use crate::search::types::{SearchEngineType, SearchResult};

pub mod baidu;
pub mod base;
pub mod bing;
pub mod brave;
pub mod duckduckgo;
pub mod exa;
pub mod google;
pub mod travily;

#[cfg(test)]
mod tests;

pub use baidu::{BaiduApiParser, BaiduParser};
pub use bing::BingParser;
pub use brave::{BraveApiParser, BraveParser};
pub use duckduckgo::{DuckDuckGoApiParser, DuckDuckGoParser};
pub use exa::ExaApiParser;
pub use google::GoogleParser;
pub use travily::TravilyApiParser;

use base::{ApiSearchParser, BaseSearchParser, WebSearchParser};

/// Unified parser that can handle both web and API queries
pub struct UnifiedParser {
    web_parser: Box<dyn WebSearchParser>,
    api_parser: Option<Box<dyn ApiSearchParser>>,
}

impl UnifiedParser {
    pub fn new(
        web_parser: Box<dyn WebSearchParser>,
        api_parser: Option<Box<dyn ApiSearchParser>>,
    ) -> Self {
        Self {
            web_parser,
            api_parser,
        }
    }

    pub fn web_only(web_parser: Box<dyn WebSearchParser>) -> Self {
        Self {
            web_parser,
            api_parser: None,
        }
    }

    pub fn api_only(api_parser: Box<dyn ApiSearchParser>) -> Self {
        Self {
            web_parser: Box::new(DummyParser::new("DummyWebParser".to_string())),
            api_parser: Some(api_parser),
        }
    }
}

impl SearchResultParser for UnifiedParser {
    fn parse(&self, content: &str, limit: usize) -> Result<Vec<SearchResult>> {
        // Try to detect if content is JSON or HTML
        if content.trim().starts_with('{') || content.trim().starts_with('[') {
            // Likely JSON content
            if let Some(ref api_parser) = self.api_parser {
                api_parser.parse_json(content, limit)
            } else {
                // Fallback to web parser if no API parser available
                self.web_parser.parse_html(content, limit)
            }
        } else {
            // Likely HTML content
            self.web_parser.parse_html(content, limit)
        }
    }

    fn name(&self) -> &str {
        self.web_parser.name()
    }

    fn supports(&self, engine_type: &SearchEngineType) -> bool {
        self.web_parser.supports(engine_type)
            || self
                .api_parser
                .as_ref()
                .is_some_and(|p| p.supports(engine_type))
    }
}

/// Dummy parser for unsupported combinations
struct DummyParser {
    name: String,
}

impl DummyParser {
    fn new(name: String) -> Self {
        Self { name }
    }
}

impl SearchResultParser for DummyParser {
    fn parse(&self, _content: &str, _limit: usize) -> Result<Vec<SearchResult>> {
        Ok(Vec::new()) // Return empty results for unsupported combinations
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn supports(&self, _engine_type: &SearchEngineType) -> bool {
        false
    }
}

impl WebSearchParser for DummyParser {
    fn parse_html(&self, _html: &str, _limit: usize) -> Result<Vec<SearchResult>> {
        Ok(Vec::new()) // Return empty results for unsupported combinations
    }
}

impl BaseSearchParser for DummyParser {
    fn name(&self) -> &str {
        &self.name
    }

    fn engine_type(&self) -> SearchEngineType {
        SearchEngineType::Bing // Use a default engine type
    }
}

/// Trait for parsing search results from HTML content (legacy compatibility)
pub trait SearchResultParser: Send + Sync {
    /// Parse search results from HTML content
    fn parse(&self, html: &str, limit: usize) -> Result<Vec<SearchResult>>;

    /// Get the name of the parser
    fn name(&self) -> &str;

    /// Check if this parser can handle the given search engine type
    fn supports(&self, engine_type: &SearchEngineType) -> bool;
}

/// Factory for creating parsers based on search engine type
pub struct ParserFactory;

impl ParserFactory {
    pub fn new() -> Self {
        Self
    }

    /// Get a parser for the given search engine type
    pub fn get_parser(&self, engine_type: &SearchEngineType) -> Box<dyn SearchResultParser> {
        match engine_type {
            // Web query parsers (HTML-based)
            SearchEngineType::Bing => Box::new(BingParser::new()),
            SearchEngineType::DuckDuckGo => Box::new(DuckDuckGoParser::new()),
            SearchEngineType::Google => Box::new(GoogleParser::new()),
            SearchEngineType::BraveSearch => Box::new(BraveParser::new()),
            SearchEngineType::Baidu => Box::new(BaiduParser::new()),
            SearchEngineType::Exa => Box::new(ExaParser::new()),
        }
    }
}

impl Default for ParserFactory {
    fn default() -> Self {
        Self::new()
    }
}
