use super::SearchResultParser;
use super::base::{ApiSearchParser, BaseApiParser, BaseSearchParser};
use crate::Result;
use crate::search::types::{SearchEngineType, SearchResult};
use serde_json::Value;

pub struct ExaApiParser {
    base: BaseApiParser,
}

impl ExaApiParser {
    pub fn new() -> Self {
        Self {
            base: BaseApiParser::new("ExaApiParser".to_string(), SearchEngineType::Exa),
        }
    }
}

impl BaseSearchParser for ExaApiParser {
    fn name(&self) -> &str {
        self.base.name()
    }
    fn engine_type(&self) -> SearchEngineType {
        self.base.engine_type()
    }
}

impl ApiSearchParser for ExaApiParser {
    fn parse_json(&self, json_content: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let json: Value = serde_json::from_str(json_content)?;
        let mut results = Vec::new();
        if let Some(results_array) = json["results"].as_array() {
            for (i, result) in results_array.iter().take(limit).enumerate() {
                results.push(SearchResult {
                    title: result["title"].as_str().unwrap_or("").to_string(),
                    url: result["url"].as_str().unwrap_or("").to_string(),
                    snippet: result["text"].as_str().unwrap_or("").to_string(),
                    rank: i,
                });
            }
        }
        Ok(results)
    }
}

impl SearchResultParser for ExaApiParser {
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

impl Default for ExaApiParser {
    fn default() -> Self {
        Self::new()
    }
}
