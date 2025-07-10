use super::super::types::{SearchEngineType, SearchResult};
use super::SearchResultParser;
use super::base::{
    ApiSearchParser, BaseApiParser, BaseSearchParser, BaseWebParser, WebSearchParser, helpers,
};
use crate::Result;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use serde_json::Value;

/// Google web parser (HTML-based)
pub struct GoogleParser {
    base: BaseWebParser,
}

impl GoogleParser {
    pub fn new() -> Self {
        Self {
            base: BaseWebParser::new("GoogleParser".to_string(), SearchEngineType::Google),
        }
    }
}

impl BaseSearchParser for GoogleParser {
    fn name(&self) -> &str {
        self.base.name()
    }

    fn engine_type(&self) -> SearchEngineType {
        self.base.engine_type()
    }
}

impl WebSearchParser for GoogleParser {
    fn parse_html(&self, html: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let document = Document::from(html);
        let mut results = Vec::new();

        // Google search results are typically in elements with class "tF2Cxc"
        for (i, result_element) in document.find(Class("tF2Cxc")).take(limit).enumerate() {
            // Extract title and URL from .yuRUbf a element
            let title_link = result_element
                .find(Class("yuRUbf").descendant(Name("a")))
                .next();

            let title = title_link
                .map(|n| n.text().trim().to_string())
                .unwrap_or_default();

            let url = title_link
                .and_then(|n| n.attr("href"))
                .map(|href| {
                    // Google sometimes uses redirect URLs or relative paths
                    if href.starts_with("http") {
                        href.to_string()
                    } else if href.starts_with("/") {
                        format!("https://www.google.com{href}")
                    } else {
                        href.to_string()
                    }
                })
                .unwrap_or_default();

            // Extract snippet from .IsZvec element
            let snippet = result_element
                .find(Class("IsZvec"))
                .next()
                .map(|n| n.text().trim().to_string())
                .unwrap_or_default();

            // Only add if we have at least a title
            if !title.is_empty() {
                results.push(SearchResult {
                    title,
                    url,
                    snippet,
                    rank: i + 1, // Test expects 1-based ranking
                });
            }
        }

        Ok(results)
    }
}

impl SearchResultParser for GoogleParser {
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

impl Default for GoogleParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Google API parser (JSON-based)
pub struct GoogleApiParser {
    base: BaseApiParser,
}

impl GoogleApiParser {
    pub fn new() -> Self {
        Self {
            base: BaseApiParser::new("GoogleApiParser".to_string(), SearchEngineType::Google),
        }
    }
}

impl BaseSearchParser for GoogleApiParser {
    fn name(&self) -> &str {
        self.base.name()
    }

    fn engine_type(&self) -> SearchEngineType {
        self.base.engine_type()
    }
}

impl ApiSearchParser for GoogleApiParser {
    fn parse_json(&self, json_content: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let json: Value = serde_json::from_str(json_content)?;
        let mut results = Vec::new();

        if let Some(organic_results) = helpers::extract_json_array(&json, "organic") {
            for (i, result) in organic_results.iter().take(limit).enumerate() {
                results.push(helpers::create_search_result_from_json(
                    result, "title", "link", "snippet", i,
                ));
            }
        }

        Ok(results)
    }
}

impl SearchResultParser for GoogleApiParser {
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

impl Default for GoogleApiParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Google Serper API parser (JSON-based)
pub struct GoogleSerperApiParser {
    base: BaseApiParser,
}

impl GoogleSerperApiParser {
    pub fn new() -> Self {
        Self {
            base: BaseApiParser::new(
                "GoogleSerperApiParser".to_string(),
                SearchEngineType::GoogleSerper,
            ),
        }
    }
}

impl BaseSearchParser for GoogleSerperApiParser {
    fn name(&self) -> &str {
        self.base.name()
    }

    fn engine_type(&self) -> SearchEngineType {
        self.base.engine_type()
    }
}

impl ApiSearchParser for GoogleSerperApiParser {
    fn parse_json(&self, json_content: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let json: Value = serde_json::from_str(json_content)?;
        let mut results = Vec::new();

        if let Some(organic_results) = helpers::extract_json_array(&json, "organic") {
            for (i, result) in organic_results.iter().take(limit).enumerate() {
                results.push(helpers::create_search_result_from_json(
                    result, "title", "link", "snippet", i,
                ));
            }
        }

        Ok(results)
    }
}

impl SearchResultParser for GoogleSerperApiParser {
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

impl Default for GoogleSerperApiParser {
    fn default() -> Self {
        Self::new()
    }
}
