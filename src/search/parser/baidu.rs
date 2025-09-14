use super::base::{BaseParser, BaseParserImpl};
use crate::Result;
use crate::search::types::{SearchEngineType, SearchResult};
use select::document::Document;
use select::predicate::{And, Class, Descendant, Name};

pub struct BaiduParser {
    base: BaseParserImpl,
}

impl BaiduParser {
    pub fn new() -> Self {
        Self {
            base: BaseParserImpl::new("BaiduParser".to_string(), SearchEngineType::Baidu),
        }
    }
}

impl BaseParser for BaiduParser {
    fn name(&self) -> &str {
        self.base.name()
    }
    fn engine_type(&self) -> SearchEngineType {
        self.base.engine_type()
    }

    fn parse(&self, html: &str, limit: usize) -> Result<Vec<SearchResult>> {
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

impl Default for BaiduParser {
    fn default() -> Self {
        Self::new()
    }
}
