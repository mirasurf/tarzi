use super::SearchResultParser;
use super::base::{
    ApiSearchParser, BaseApiParser, BaseSearchParser, BaseWebParser, WebSearchParser, helpers,
};
use crate::Result;
use crate::search::types::{SearchEngineType, SearchResult};
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

        for (i, result_element) in document.find(Class("result")).take(limit).enumerate() {
            // Title and URL
            let title_link = result_element
                .find(Name("a").and(Class("result__a")))
                .next();

            let title = title_link
                .as_ref()
                .map(|n| n.text().trim().to_string())
                .unwrap_or_default();

            let url = title_link
                .and_then(|n| n.attr("href"))
                .map(|href| {
                    if href.starts_with("http") {
                        href.to_string()
                    } else if href.starts_with("/") {
                        format!("https://duckduckgo.com{href}")
                    } else {
                        href.to_string()
                    }
                })
                .unwrap_or_default();

            // Snippet
            let snippet = result_element
                .find(Class("result__snippet"))
                .next()
                .map(|n| n.text().trim().to_string())
                .unwrap_or_else(|| {
                    // Fallback: some pages use 'div.result__content' or 'div.result__extras'
                    result_element
                        .find(Class("result__content"))
                        .next()
                        .map(|n| n.text().trim().to_string())
                        .unwrap_or_default()
                });

            if !title.is_empty() {
                results.push(SearchResult {
                    title,
                    url,
                    snippet,
                    rank: i + 1,
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
