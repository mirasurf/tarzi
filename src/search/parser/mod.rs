use super::types::{SearchEngineType, SearchResult};
use crate::Result;

pub mod baidu;
pub mod bing;
pub mod brave;
pub mod custom;
pub mod duckduckgo;
pub mod google;

#[cfg(test)]
mod tests;

pub use baidu::BaiduParser;
pub use bing::BingParser;
pub use brave::BraveParser;
pub use custom::{CustomParser, CustomParserConfig};
pub use duckduckgo::DuckDuckGoParser;
pub use google::GoogleParser;

/// Trait for parsing search results from HTML content
pub trait SearchResultParser: Send + Sync {
    /// Parse search results from HTML content
    fn parse(&self, html: &str, limit: usize) -> Result<Vec<SearchResult>>;

    /// Get the name of the parser
    fn name(&self) -> &str;

    /// Check if this parser can handle the given search engine type
    fn supports(&self, engine_type: &SearchEngineType) -> bool;
}

/// Factory for creating parsers based on search engine type
pub struct ParserFactory {
    custom_parsers: std::collections::HashMap<String, Box<dyn SearchResultParser>>,
}

impl ParserFactory {
    pub fn new() -> Self {
        Self {
            custom_parsers: std::collections::HashMap::new(),
        }
    }

    /// Register a custom parser
    pub fn register_custom_parser(&mut self, name: String, parser: Box<dyn SearchResultParser>) {
        self.custom_parsers.insert(name, parser);
    }

    /// Get a parser for the given search engine type
    pub fn get_parser(&self, engine_type: &SearchEngineType) -> Box<dyn SearchResultParser> {
        match engine_type {
            SearchEngineType::Bing => Box::new(BingParser::new()),
            SearchEngineType::DuckDuckGo => Box::new(DuckDuckGoParser::new()),
            SearchEngineType::Google => Box::new(GoogleParser::new()),
            SearchEngineType::BraveSearch => Box::new(BraveParser::new()),
            SearchEngineType::Baidu => Box::new(BaiduParser::new()),
            SearchEngineType::Exa => Box::new(CustomParser::new("Exa".to_string())),
            SearchEngineType::Travily => Box::new(CustomParser::new("Travily".to_string())),
            SearchEngineType::GoogleSerper => Box::new(CustomParser::new("GoogleSerper".to_string())),
            SearchEngineType::Custom(name) => {
                if let Some(_parser) = self.custom_parsers.get(name) {
                    // Note: This is a simplified approach. In practice, you might want
                    // to use Arc<dyn SearchResultParser> for shared ownership
                    Box::new(CustomParser::new(name.clone()))
                } else {
                    Box::new(CustomParser::new(name.clone()))
                }
            }
        }
    }
}

impl Default for ParserFactory {
    fn default() -> Self {
        Self::new()
    }
}
