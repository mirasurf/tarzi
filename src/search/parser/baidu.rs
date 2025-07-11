use super::SearchResultParser;
use super::base::{
    ApiSearchParser, BaseApiParser, BaseSearchParser, BaseWebParser, WebSearchParser,
};
use crate::Result;
use crate::search::types::{SearchEngineType, SearchResult};
use select::document::Document;
use select::predicate::{And, Class, Descendant, Name};
use serde_json::Value;

pub struct BaiduParser {
    base: BaseWebParser,
}

impl BaiduParser {
    pub fn new() -> Self {
        Self {
            base: BaseWebParser::new("BaiduParser".to_string(), SearchEngineType::Baidu),
        }
    }
}

impl BaseSearchParser for BaiduParser {
    fn name(&self) -> &str {
        self.base.name()
    }
    fn engine_type(&self) -> SearchEngineType {
        self.base.engine_type()
    }
}

impl WebSearchParser for BaiduParser {
    fn parse_html(&self, html: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let document = Document::from(html);
        let mut results = Vec::new();
        let result_selector = And(Class("result"), Class("c-container"));
        for node in document.find(result_selector) {
            // Skip ads
            if node.attr("data-tuiguang").is_some() {
                continue;
            }
            let title = node
                .find(Descendant(Name("h3"), Name("a")))
                .next()
                .map(|n| n.text().trim().to_string())
                .unwrap_or_default();
            let url = node
                .find(Descendant(Name("h3"), Name("a")))
                .next()
                .and_then(|n| n.attr("href"))
                .unwrap_or_default()
                .to_string();
            let snippet = node
                .find(Class("c-abstract"))
                .next()
                .map(|n| n.text().trim().to_string())
                .unwrap_or_default();
            if !title.is_empty() {
                results.push(SearchResult {
                    title,
                    url,
                    snippet,
                    rank: results.len() + 1,
                });
            }
            if results.len() >= limit {
                break;
            }
        }
        Ok(results)
    }
}

impl SearchResultParser for BaiduParser {
    fn parse(&self, html: &str, limit: usize) -> Result<Vec<SearchResult>> {
        self.parse_html(html, limit)
    }
    fn name(&self) -> &str {
        BaseSearchParser::name(self)
    }
    fn supports(&self, engine_type: &SearchEngineType) -> bool {
        BaseSearchParser::supports(self, engine_type)
    }
}

impl Default for BaiduParser {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BaiduApiParser {
    base: BaseApiParser,
}

impl BaiduApiParser {
    pub fn new() -> Self {
        Self {
            base: BaseApiParser::new("BaiduApiParser".to_string(), SearchEngineType::Baidu),
        }
    }
}

impl BaseSearchParser for BaiduApiParser {
    fn name(&self) -> &str {
        self.base.name()
    }
    fn engine_type(&self) -> SearchEngineType {
        self.base.engine_type()
    }
}

impl ApiSearchParser for BaiduApiParser {
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
                    snippet: result["snippet"].as_str().unwrap_or("").to_string(),
                    rank: results.len() + 1, // Use results.len() + 1 for proper ranking
                });
            }
        }
        Ok(results)
    }
}

impl SearchResultParser for BaiduApiParser {
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

impl Default for BaiduApiParser {
    fn default() -> Self {
        Self::new()
    }
}
