//! Search module
//!
//! This module provides functionality for searching the web using different search engines:
//! - Web-based search through browser automation
//! - Support for multiple search engines (Bing, Google, DuckDuckGo, etc.)
//! - Extensible parser system for extracting search results from HTML

pub mod api;
pub mod engine;
pub mod parser;
pub mod providers;
pub mod types;

#[cfg(test)]
mod tests;

// Re-export main types and functions
pub use api::AutoSwitchStrategy;
pub use engine::SearchEngine;
pub use parser::{ParserFactory, SearchResultParser};
pub use types::{SearchEngineType, SearchResult};
