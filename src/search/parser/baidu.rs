use super::SearchResultParser;
use crate::Result;
use crate::search::types::{SearchEngineType, SearchResult};
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use tracing::{info, warn};

pub struct BaiduParser;

impl BaiduParser {
    pub fn new() -> Self {
        Self
    }
}

impl SearchResultParser for BaiduParser {
    fn parse(&self, html: &str, limit: usize) -> Result<Vec<SearchResult>> {
        info!("Parsing Baidu search results from HTML");

        if html.is_empty() {
            warn!("Empty HTML provided to BaiduParser");
            return Ok(Vec::new());
        }

        let document = Document::from(html);
        let mut results = Vec::new();

        // Look for Baidu search result containers
        // Baidu uses .result.c-container for individual result containers
        let result_selector = Class("result").and(Class("c-container"));

        for node in document.find(result_selector).take(limit * 2) {
            // Skip if we already have enough results
            if results.len() >= limit {
                break;
            }

            // Skip ad entries (contains "data-tuiguang" or certain ad class markers)
            let is_ad = node.attrs().any(|(k, _)| k.contains("data-tuiguang"))
                || node
                    .attr("class")
                    .map(|c| c.contains("ec_ad") || c.contains("ad-block"))
                    .unwrap_or(false);

            if is_ad {
                continue;
            }

            // Extract title and URL from h3 a element
            let title_link = node.find(Name("h3").descendant(Name("a"))).next();

            let title = title_link
                .map(|n| n.text().trim().to_string())
                .unwrap_or_default();

            let url = title_link
                .and_then(|n| n.attr("href"))
                .map(|href| {
                    // Baidu sometimes uses redirect URLs or relative paths
                    if href.starts_with("http") {
                        href.to_string()
                    } else if href.starts_with("/") {
                        format!("https://www.baidu.com{}", href)
                    } else {
                        href.to_string()
                    }
                })
                .unwrap_or_default();

            // Extract snippet from .c-abstract element
            let snippet = node
                .find(Class("c-abstract"))
                .next()
                .map(|n| n.text().trim().to_string())
                .unwrap_or_default();

            // Only add if we have at least a title
            if !title.is_empty() {
                let result = SearchResult {
                    title,
                    url,
                    snippet,
                    rank: results.len() + 1,
                };
                info!(
                    "Extracted Baidu result #{}: {}",
                    results.len() + 1,
                    result.title
                );
                results.push(result);
            }
        }

        info!(
            "Successfully parsed {} Baidu search results from HTML",
            results.len()
        );
        Ok(results)
    }

    fn name(&self) -> &str {
        "BaiduParser"
    }

    fn supports(&self, engine_type: &SearchEngineType) -> bool {
        matches!(engine_type, SearchEngineType::Baidu)
    }
}

impl Default for BaiduParser {
    fn default() -> Self {
        Self::new()
    }
}
