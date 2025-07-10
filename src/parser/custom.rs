use super::SearchResultParser;
use super::base::{
    ApiSearchParser, BaseApiParser, BaseSearchParser, BaseWebParser, WebSearchParser,
};
use crate::Result;
use crate::search::types::{SearchEngineType, SearchResult};
use serde_json::Value;
use std::collections::HashMap;

pub struct CustomParser {
    engine_name: String,
    custom_config: CustomParserConfig,
    base: BaseWebParser,
}

#[derive(Debug, Clone)]
pub struct CustomParserConfig {
    /// CSS selectors for extracting search results
    pub result_container_selector: String,
    pub title_selector: String,
    pub url_selector: String,
    pub snippet_selector: String,
    /// Optional custom parsing rules
    pub custom_rules: HashMap<String, String>,
}

impl Default for CustomParserConfig {
    fn default() -> Self {
        Self {
            result_container_selector: ".result".to_string(),
            title_selector: "h3 a".to_string(),
            url_selector: "cite".to_string(),
            snippet_selector: ".snippet".to_string(),
            custom_rules: HashMap::new(),
        }
    }
}

impl CustomParser {
    pub fn new(engine_name: String) -> Self {
        Self {
            base: BaseWebParser::new(
                engine_name.clone(),
                SearchEngineType::Custom(engine_name.clone()),
            ),
            engine_name,
            custom_config: CustomParserConfig::default(),
        }
    }

    pub fn with_config(engine_name: String, config: CustomParserConfig) -> Self {
        Self {
            base: BaseWebParser::new(
                engine_name.clone(),
                SearchEngineType::Custom(engine_name.clone()),
            ),
            engine_name,
            custom_config: config,
        }
    }

    /// Set custom CSS selectors for parsing
    pub fn set_selectors(&mut self, container: &str, title: &str, url: &str, snippet: &str) {
        self.custom_config.result_container_selector = container.to_string();
        self.custom_config.title_selector = title.to_string();
        self.custom_config.url_selector = url.to_string();
        self.custom_config.snippet_selector = snippet.to_string();
    }

    /// Add a custom parsing rule
    pub fn add_custom_rule(&mut self, key: String, value: String) {
        self.custom_config.custom_rules.insert(key, value);
    }

    pub fn config(&self) -> &CustomParserConfig {
        &self.custom_config
    }
}

impl BaseSearchParser for CustomParser {
    fn name(&self) -> &str {
        self.base.name()
    }
    fn engine_type(&self) -> SearchEngineType {
        self.base.engine_type()
    }
}

impl WebSearchParser for CustomParser {
    fn parse_html(&self, _html: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let mut results = Vec::new();
        let mock_results_count = std::cmp::min(limit, 8);
        for i in 0..mock_results_count {
            let rank = i + 1;
            let result = SearchResult {
                title: format!(
                    "{} Custom Search Result #{} - Mock Title",
                    self.engine_name, rank
                ),
                url: format!(
                    "https://example-{}-result-{}.com",
                    self.engine_name.to_lowercase(),
                    rank
                ),
                snippet: format!(
                    "This is a mock {} search result snippet for result {}. Custom parser using selectors: container='{}', title='{}', url='{}', snippet='{}'.",
                    self.engine_name,
                    rank,
                    self.custom_config.result_container_selector,
                    self.custom_config.title_selector,
                    self.custom_config.url_selector,
                    self.custom_config.snippet_selector
                ),
                rank,
            };
            results.push(result);
        }
        Ok(results)
    }
}

impl SearchResultParser for CustomParser {
    fn parse(&self, html: &str, limit: usize) -> Result<Vec<SearchResult>> {
        self.parse_html(html, limit)
    }
    fn name(&self) -> &str {
        BaseSearchParser::name(self)
    }
    fn supports(&self, engine_type: &SearchEngineType) -> bool {
        match engine_type {
            SearchEngineType::Custom(name) => name == &self.engine_name,
            _ => false,
        }
    }
}

impl Default for CustomParser {
    fn default() -> Self {
        Self::new("Custom".to_string())
    }
}

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
            for (i, result) in results_array.iter().take(limit).enumerate() {
                results.push(SearchResult {
                    title: result["title"].as_str().unwrap_or("").to_string(),
                    url: result["url"].as_str().unwrap_or("").to_string(),
                    snippet: result["content"].as_str().unwrap_or("").to_string(),
                    rank: i,
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
