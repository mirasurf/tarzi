use super::SearchResultParser;
use super::base::{
    ApiSearchParser, BaseApiParser, BaseSearchParser, BaseWebParser, WebSearchParser,
};
use crate::Result;
use crate::search::types::{SearchEngineType, SearchResult};
use select::document::Document;
use select::predicate::{Class, Name};
use serde_json::Value;

pub struct BraveParser {
    base: BaseWebParser,
}

impl BraveParser {
    pub fn new() -> Self {
        Self {
            base: BaseWebParser::new("BraveParser".to_string(), SearchEngineType::BraveSearch),
        }
    }
}

impl BaseSearchParser for BraveParser {
    fn name(&self) -> &str {
        self.base.name()
    }
    fn engine_type(&self) -> SearchEngineType {
        self.base.engine_type()
    }
}

impl WebSearchParser for BraveParser {
    fn parse_html(&self, html: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let document = Document::from(html);
        let mut results = Vec::new();
        for (rank, node) in document.find(Class("result-row")).take(limit).enumerate() {
            let title_link = node.find(Name("a")).next();
            let title = title_link
                .map(|n| n.text().trim().to_string())
                .unwrap_or_default();
            let url = title_link
                .and_then(|n| n.attr("href"))
                .map(|href| {
                    if href.starts_with("http") {
                        href.to_string()
                    } else if href.starts_with("/") {
                        format!("https://search.brave.com{href}")
                    } else {
                        href.to_string()
                    }
                })
                .unwrap_or_default();
            let snippet = node
                .find(Class("result-snippet"))
                .next()
                .map(|n| n.text().trim().to_string())
                .unwrap_or_default();
            if !title.is_empty() {
                results.push(SearchResult {
                    title,
                    url,
                    snippet,
                    rank: rank + 1,
                });
            }
        }
        Ok(results)
    }
}

impl SearchResultParser for BraveParser {
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

impl Default for BraveParser {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BraveApiParser {
    base: BaseApiParser,
}

impl BraveApiParser {
    pub fn new() -> Self {
        Self {
            base: BaseApiParser::new("BraveApiParser".to_string(), SearchEngineType::BraveSearch),
        }
    }
}

impl BaseSearchParser for BraveApiParser {
    fn name(&self) -> &str {
        self.base.name()
    }
    fn engine_type(&self) -> SearchEngineType {
        self.base.engine_type()
    }
}

impl ApiSearchParser for BraveApiParser {
    fn parse_json(&self, json_content: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let json: Value = serde_json::from_str(json_content)?;
        let mut results = Vec::new();
        if let Some(web_results) = json["web"]["results"].as_array() {
            for (i, result) in web_results.iter().take(limit).enumerate() {
                results.push(SearchResult {
                    title: result["title"].as_str().unwrap_or("").to_string(),
                    url: result["url"].as_str().unwrap_or("").to_string(),
                    snippet: result["description"].as_str().unwrap_or("").to_string(),
                    rank: i,
                });
            }
        }
        Ok(results)
    }
}

impl SearchResultParser for BraveApiParser {
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

impl Default for BraveApiParser {
    fn default() -> Self {
        Self::new()
    }
}
