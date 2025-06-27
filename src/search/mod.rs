//! Search module
//!
//! This module provides functionality for searching the web using different search engines:
//! - Web-based search through browser automation
//! - API-based search through search engine APIs
//! - Support for multiple search engines (Bing, Google, DuckDuckGo, etc.)
//! - Extensible parser system for extracting search results from HTML

pub mod engine;
pub mod types;
pub mod parser;

#[cfg(test)]
mod tests;

// Re-export main types and functions
pub use engine::SearchEngine;
pub use types::{SearchEngineType, SearchMode, SearchResult};
pub use parser::{SearchResultParser, ParserFactory, CustomParserConfig}; 