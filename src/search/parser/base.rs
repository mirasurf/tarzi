use crate::search::types::{SearchEngineType, SearchResult};
use crate::Result;
use serde_json::Value;

/// Base trait for all search result parsers
pub trait BaseParser: Send + Sync {
    /// Get the name of the parser
    fn name(&self) -> &str;

    /// Get the engine type this parser supports
    fn engine_type(&self) -> SearchEngineType;

    /// Check if this parser can handle the given search engine type
    fn supports(&self, engine_type: &SearchEngineType) -> bool {
        &self.engine_type() == engine_type
    }

    /// Parse search results from HTML content
    fn parse(&self, html: &str, limit: usize) -> Result<Vec<SearchResult>>;
}

/// Common base implementation for all parsers
pub struct BaseParserImpl {
    name: String,
    engine_type: SearchEngineType,
}

impl BaseParserImpl {
    pub fn new(name: String, engine_type: SearchEngineType) -> Self {
        Self { name, engine_type }
    }
}

impl BaseParser for BaseParserImpl {
    fn name(&self) -> &str {
        &self.name
    }

    fn engine_type(&self) -> SearchEngineType {
        self.engine_type
    }

    fn parse(&self, _html: &str, _limit: usize) -> Result<Vec<SearchResult>> {
        Ok(Vec::new()) // dummy implementation
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
