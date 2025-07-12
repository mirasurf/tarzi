//! Web content fetching module
//!
//! This module provides functionality for fetching web content using different methods:
//! - Plain HTTP requests
//! - Browser automation (headless and headed)

pub mod browser;
pub mod driver;
pub mod types;
pub mod webfetcher;

// Re-export main types and functions
pub use driver::{DriverConfig, DriverInfo, DriverManager, DriverStatus, DriverType};
pub use types::{FetchMode, WebFetcher};
pub use webfetcher::WebFetcher as WebFetcherImpl;
