//! Web content fetching module
//!
//! This module provides functionality for fetching web content using different methods:
//! - Plain HTTP requests
//! - Browser automation (headless and headed)
//! - External browser connections

pub mod browser;
pub mod external;
pub mod types;
pub mod webfetcher;

// Re-export main types and functions
pub use types::{FetchMode, WebFetcher};
pub use webfetcher::WebFetcher as WebFetcherImpl;
