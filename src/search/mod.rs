//! Search module
//!
//! This module provides functionality for searching the web using different search engines:
//! - Web-based search through browser automation
//! - API-based search through search engine APIs
//! - Support for multiple search engines (Bing, Google, DuckDuckGo, etc.)
//! - Extensible parser system for extracting search results from HTML

pub mod engine;
pub mod parser;
pub mod types;

#[cfg(test)]
mod tests;

// Re-export main types and functions
pub use engine::SearchEngine;
pub use parser::{CustomParserConfig, ParserFactory, SearchResultParser};
pub use types::{SearchEngineType, SearchMode, SearchResult};
