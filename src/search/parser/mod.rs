//! Parser module for extracting search results from different content types
//!
//! This module provides parsers for extracting search results from HTML content
//! and other formats returned by search engines.

pub mod baidu;
pub mod base;
pub mod bing;
pub mod brave;
pub mod duckduckgo;
pub mod google;
pub mod sogou_weixin;

use crate::search::types::SearchEngineType;

// Re-export parser types
pub use baidu::BaiduParser;
pub use base::BaseParser;
pub use bing::BingParser;
pub use brave::BraveParser;
pub use duckduckgo::DuckDuckGoParser;
pub use google::GoogleParser;
pub use sogou_weixin::SogouWeixinParser;

#[cfg(test)]
mod tests;

/// Factory for creating parsers based on search engine type
pub struct ParserFactory;

impl ParserFactory {
    pub fn new() -> Self {
        Self
    }

    /// Get a parser for the given search engine type
    pub fn get_parser(&self, engine_type: &SearchEngineType) -> Box<dyn BaseParser> {
        match engine_type {
            // Web query parsers (HTML-based)
            SearchEngineType::Bing => Box::new(BingParser::new()),
            SearchEngineType::DuckDuckGo => Box::new(DuckDuckGoParser::new()),
            SearchEngineType::Google => Box::new(GoogleParser::new()),
            SearchEngineType::BraveSearch => Box::new(BraveParser::new()),
            SearchEngineType::Baidu => Box::new(BaiduParser::new()),
            SearchEngineType::SougouWeixin => Box::new(SogouWeixinParser::new()),
        }
    }
}

impl Default for ParserFactory {
    fn default() -> Self {
        Self::new()
    }
}
