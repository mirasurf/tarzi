use super::base::{BaseSearchParser, BaseWebParser, WebSearchParser};
use crate::Result;
use crate::search::types::{SearchEngineType, SearchResult};
use select::document::Document;
use select::predicate::{Class, Descendant, Name};

pub struct BingParser {
    base: BaseWebParser,
}

impl BingParser {
    pub fn new() -> Self {
        Self {
            base: BaseWebParser::new("BingParser".to_string(), SearchEngineType::Bing),
        }
    }
}

impl BaseSearchParser for BingParser {
    fn name(&self) -> &str {
        self.base.name()
    }
    fn engine_type(&self) -> SearchEngineType {
        self.base.engine_type()
    }
}

impl WebSearchParser for BingParser {
    fn parse_html(&self, html: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let document = Document::from(html);
        let mut results = Vec::new();
        for node in document.find(Class("b_algo")) {
            // Check if we've reached the limit
            if results.len() >= limit {
                break;
            }

            let title_link = node.find(Descendant(Name("h2"), Name("a"))).next();
            let title = title_link
                .map(|n| n.text().trim().to_string())
                .unwrap_or_default();
            let url = title_link
                .and_then(|n| n.attr("href"))
                .map(|href| {
                    if href.starts_with("http") {
                        href.to_string()
                    } else if href.starts_with("/") {
                        format!("https://www.bing.com{href}")
                    } else {
                        href.to_string()
                    }
                })
                .unwrap_or_default();
            let snippet = node
                .find(Descendant(Class("b_caption"), Name("p")))
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

use super::SearchResultParser;
impl SearchResultParser for BingParser {
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

impl Default for BingParser {
    fn default() -> Self {
        Self::new()
    }
}
