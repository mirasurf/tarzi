use super::SearchResultParser;
use super::base::{ApiSearchParser, BaseApiParser, BaseSearchParser};
use crate::Result;
use crate::search::types::{SearchEngineType, SearchResult};
use serde_json::Value;

pub struct TravilyApiParser {
    base: BaseApiParser,
}

impl TravilyApiParser {
    pub fn new() -> Self {
        Self {
            base: BaseApiParser::new("TravilyApiParser".to_string(), SearchEngineType::Travily),
        }
    }
}

impl BaseSearchParser for TravilyApiParser {
    fn name(&self) -> &str {
        self.base.name()
    }
    fn engine_type(&self) -> SearchEngineType {
        self.base.engine_type()
    }
}

impl ApiSearchParser for TravilyApiParser {
    fn parse_json(&self, json_content: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let json: Value = serde_json::from_str(json_content)?;
        let mut results = Vec::new();
        if let Some(results_array) = json["results"].as_array() {
            for result in results_array.iter() {
                // Check if we've reached the limit
                if results.len() >= limit {
                    break;
                }

                results.push(SearchResult {
                    title: result["title"].as_str().unwrap_or("").to_string(),
                    url: result["url"].as_str().unwrap_or("").to_string(),
                    snippet: result["content"].as_str().unwrap_or("").to_string(),
                    rank: results.len() + 1, // Use results.len() + 1 for proper ranking
                });
            }
        }
        Ok(results)
    }
}

impl SearchResultParser for TravilyApiParser {
    fn parse(&self, json_content: &str, limit: usize) -> Result<Vec<SearchResult>> {
        self.parse_json(json_content, limit)
    }
    fn name(&self) -> &str {
        BaseSearchParser::name(self)
    }
    fn supports(&self, engine_type: &SearchEngineType) -> bool {
        BaseSearchParser::supports(self, engine_type)
    }
}

impl Default for TravilyApiParser {
    fn default() -> Self {
        Self::new()
    }
}
