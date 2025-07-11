use super::SearchResultParser;
use super::base::{BaseSearchParser, BaseWebParser, WebSearchParser};
use crate::Result;
use crate::search::types::{SearchEngineType, SearchResult};
use select::document::Document;
use select::predicate::{Class, Name};

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
        for (_, result_element) in document.find(Class("tF2Cxc")).enumerate() {
            // Check if we've reached the limit
            if results.len() >= limit {
                break;
            }

            // Extract title and URL from .yuRUbf a element
            let title_link = result_element.find(Name("a")).next();

            let title = title_link
                .map(|n| n.text().trim().to_string())
                .unwrap_or_default();

            let url = title_link
                .and_then(|n| n.attr("href"))
                .map(|href| {
                    if href.starts_with("http") {
                        href.to_string()
                    } else if href.starts_with("/") {
                        format!("https://www.google.com{href}")
                    } else {
                        href.to_string()
                    }
                })
                .unwrap_or_default();

            let snippet = result_element
                .find(Class("IsZvec"))
                .next()
                .map(|n| n.text().trim().to_string())
                .unwrap_or_default();

            if !title.is_empty() {
                results.push(SearchResult {
                    title,
                    url,
                    snippet,
                    rank: results.len() + 1, // Use results.len() + 1 for proper ranking
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
