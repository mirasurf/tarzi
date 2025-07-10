use super::super::types::{SearchEngineType, SearchResult};
use super::SearchResultParser;
use super::base::{
    ApiSearchParser, BaseApiParser, BaseSearchParser, BaseWebParser, WebSearchParser, helpers,
};
use crate::Result;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use serde_json::Value;

/// DuckDuckGo web parser (HTML-based)
pub struct DuckDuckGoParser {
    base: BaseWebParser,
}

impl DuckDuckGoParser {
    pub fn new() -> Self {
        Self {
            base: BaseWebParser::new("DuckDuckGoParser".to_string(), SearchEngineType::DuckDuckGo),
        }
    }
}

impl BaseSearchParser for DuckDuckGoParser {
    fn name(&self) -> &str {
        self.base.name()
    }

    fn engine_type(&self) -> SearchEngineType {
        self.base.engine_type()
    }
}

impl WebSearchParser for DuckDuckGoParser {
    fn parse_html(&self, html: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let document = Document::from(html);
        let mut results = Vec::new();

        // DuckDuckGo search results are typically in elements with class "result__body"
        for (i, result_element) in document.find(Class("result__body")).take(limit).enumerate() {
            // Extract title and URL from a.result__a element
            let title_link = result_element
                .find(Name("a").and(Class("result__a")))
                .next();

            let title = title_link
                .map(|n| n.text().trim().to_string())
                .unwrap_or_default();

            let url = title_link
                .and_then(|n| n.attr("href"))
                .map(|href| {
                    // DuckDuckGo sometimes uses redirect URLs or relative paths
                    if href.starts_with("http") {
                        href.to_string()
                    } else if href.starts_with("/") {
                        format!("https://duckduckgo.com{href}")
                    } else {
                        href.to_string()
                    }
                })
                .unwrap_or_default();

            // Extract snippet from .result__snippet element
            let snippet = result_element
                .find(Class("result__snippet"))
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

impl SearchResultParser for DuckDuckGoParser {
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

impl Default for DuckDuckGoParser {
    fn default() -> Self {
        Self::new()
    }
}

/// DuckDuckGo API parser (JSON-based)
pub struct DuckDuckGoApiParser {
    base: BaseApiParser,
}

impl DuckDuckGoApiParser {
    pub fn new() -> Self {
        Self {
            base: BaseApiParser::new(
                "DuckDuckGoApiParser".to_string(),
                SearchEngineType::DuckDuckGo,
            ),
        }
    }
}

impl BaseSearchParser for DuckDuckGoApiParser {
    fn name(&self) -> &str {
        self.base.name()
    }

    fn engine_type(&self) -> SearchEngineType {
        self.base.engine_type()
    }
}

impl ApiSearchParser for DuckDuckGoApiParser {
    fn parse_json(&self, json_content: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let json: Value = serde_json::from_str(json_content)?;
        let mut results = Vec::new();

        // Parse AbstractText if available
        let abstract_text = helpers::extract_json_text(&json, "AbstractText");
        if !abstract_text.is_empty() {
            results.push(SearchResult {
                title: helpers::extract_json_text(&json, "Heading"),
                url: helpers::extract_json_text(&json, "AbstractURL"),
                snippet: abstract_text,
                rank: results.len(),
            });
        }

        // Parse RelatedTopics if available
        if let Some(related_topics) = helpers::extract_json_array(&json, "RelatedTopics") {
            for topic in related_topics
                .iter()
                .take(limit.saturating_sub(results.len()))
            {
                let text = helpers::extract_json_text(topic, "Text");
                let first_url = helpers::extract_json_text(topic, "FirstURL");
                if !text.is_empty() && !first_url.is_empty() {
                    results.push(SearchResult {
                        title: text.split(" - ").next().unwrap_or("").to_string(),
                        url: first_url,
                        snippet: text,
                        rank: results.len(),
                    });
                }
            }
        }

        Ok(results.into_iter().take(limit).collect())
    }
}

impl SearchResultParser for DuckDuckGoApiParser {
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

impl Default for DuckDuckGoApiParser {
    fn default() -> Self {
        Self::new()
    }
}
