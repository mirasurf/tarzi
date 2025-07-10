THIS SHOULD BE A LINTER ERRORuse super::super::types::{SearchEngineType, SearchResult};
use crate::Result;
use serde_json::Value;

/// Base trait for all search result parsers
pub trait BaseSearchParser: Send + Sync {
    /// Get the name of the parser
    fn name(&self) -> &str;

    /// Get the engine type this parser supports
    fn engine_type(&self) -> SearchEngineType;

    /// Check if this parser can handle the given search engine type
    fn supports(&self, engine_type: &SearchEngineType) -> bool {
        &self.engine_type() == engine_type
    }
}

/// Base trait for web query parsers (HTML-based)
pub trait WebSearchParser: BaseSearchParser {
    /// Parse search results from HTML content
    fn parse_html(&self, html: &str, limit: usize) -> Result<Vec<SearchResult>>;
}

/// Base trait for API query parsers (JSON-based)
pub trait ApiSearchParser: BaseSearchParser {
    /// Parse search results from JSON content
    fn parse_json(&self, json_content: &str, limit: usize) -> Result<Vec<SearchResult>>;
}

/// Unified parser trait that combines web and API parsing capabilities
pub trait SearchResultParser: Send + Sync {
    /// Parse search results from content (HTML or JSON)
    fn parse(&self, content: &str, limit: usize) -> Result<Vec<SearchResult>>;

    /// Get the name of the parser
    fn name(&self) -> &str;

    /// Check if this parser can handle the given search engine type
    fn supports(&self, engine_type: &SearchEngineType) -> bool;
}

/// Base implementation for web parsers
pub struct BaseWebParser {
    name: String,
    engine_type: SearchEngineType,
}

impl BaseWebParser {
    pub fn new(name: String, engine_type: SearchEngineType) -> Self {
        Self { name, engine_type }
    }
}

impl BaseSearchParser for BaseWebParser {
    fn name(&self) -> &str {
        &self.name
    }

    fn engine_type(&self) -> SearchEngineType {
        self.engine_type.clone()
    }
}

/// Base implementation for API parsers
pub struct BaseApiParser {
    name: String,
    engine_type: SearchEngineType,
}

impl BaseApiParser {
    pub fn new(name: String, engine_type: SearchEngineType) -> Self {
        Self { name, engine_type }
    }
}

impl BaseSearchParser for BaseApiParser {
    fn name(&self) -> &str {
        &self.name
    }

    fn engine_type(&self) -> SearchEngineType {
        self.engine_type.clone()
    }
}

/// Helper functions for common parsing operations
pub mod helpers {
    use super::*;

    /// Extract text from a JSON field safely
    pub fn extract_json_text(json: &Value, field: &str) -> String {
        json[field].as_str().unwrap_or("").to_string()
    }

    /// Extract text from a nested JSON field safely
    pub fn extract_nested_json_text(json: &Value, path: &[&str]) -> String {
        let mut current = json;
        for field in path {
            current = &current[field];
        }
        current.as_str().unwrap_or("").to_string()
    }

    /// Extract array from JSON safely
    pub fn extract_json_array(json: &Value, field: &str) -> Option<Vec<Value>> {
        json[field].as_array().cloned()
    }

    /// Extract nested array from JSON safely
    pub fn extract_nested_json_array(json: &Value, path: &[&str]) -> Option<Vec<Value>> {
        let mut current = json;
        for field in path {
            current = &current[field];
        }
        current.as_array().cloned()
    }

    /// Create a SearchResult from JSON fields
    pub fn create_search_result_from_json(
        json: &Value,
        title_field: &str,
        url_field: &str,
        snippet_field: &str,
        rank: usize,
    ) -> SearchResult {
        SearchResult {
            title: extract_json_text(json, title_field),
            url: extract_json_text(json, url_field),
            snippet: extract_json_text(json, snippet_field),
            rank,
        }
    }

    /// Create a SearchResult from nested JSON fields
    pub fn create_search_result_from_nested_json(
        json: &Value,
        title_path: &[&str],
        url_path: &[&str],
        snippet_path: &[&str],
        rank: usize,
    ) -> SearchResult {
        SearchResult {
            title: extract_nested_json_text(json, title_path),
            url: extract_nested_json_text(json, url_path),
            snippet: extract_nested_json_text(json, snippet_path),
            rank,
        }
    }
}
