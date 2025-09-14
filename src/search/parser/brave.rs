use super::base::{BaseParser, BaseParserImpl};
use crate::Result;
use crate::search::types::{SearchEngineType, SearchResult};
use select::document::Document;
use select::predicate::{Class, Name};

pub struct BraveParser {
    base: BaseParserImpl,
}

impl BraveParser {
    pub fn new() -> Self {
        Self {
            base: BaseParserImpl::new("BraveParser".to_string(), SearchEngineType::BraveSearch),
        }
    }
}

impl BaseParser for BraveParser {
    fn name(&self) -> &str {
        self.base.name()
    }
    fn engine_type(&self) -> SearchEngineType {
        self.base.engine_type()
    }

    fn parse(&self, html: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let document = Document::from(html);
        let mut results = Vec::new();
        for node in document.find(Class("result-row")) {
            // Check if we've reached the limit
            if results.len() >= limit {
                break;
            }

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
                    rank: results.len() + 1, // Use results.len() + 1 for proper ranking
                });
            }
        }
        Ok(results)
    }
}

impl Default for BraveParser {
    fn default() -> Self {
        Self::new()
    }
}
