//! Search module
//!
//! This module provides functionality for searching the web using different search engines:
//! - Web-based search through browser automation
//! - API-based search through search engine APIs
//! - Support for multiple search engines (Bing, Google, DuckDuckGo, etc.)
//! - Extensible parser system for extracting search results from HTML

pub mod api;
pub mod engine;
pub mod types;

#[cfg(test)]
mod tests;

// Re-export main types and functions
pub use crate::parser::{CustomParserConfig, ParserFactory, SearchResultParser};
pub use api::{ApiSearchManager, AutoSwitchStrategy, SearchApiProvider};
pub use engine::SearchEngine;
pub use types::{SearchEngineType, SearchMode, SearchResult};
