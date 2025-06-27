use super::SearchResultParser;
use crate::Result;
use crate::search::types::{SearchEngineType, SearchResult};
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use tracing::{info, warn};

pub struct DuckDuckGoParser;

impl DuckDuckGoParser {
    pub fn new() -> Self {
        Self
    }
}

impl SearchResultParser for DuckDuckGoParser {
    fn parse(&self, html: &str, limit: usize) -> Result<Vec<SearchResult>> {
        info!("Parsing DuckDuckGo search results from HTML");

        if html.is_empty() {
            warn!("Empty HTML provided to DuckDuckGoParser");
            return Ok(Vec::new());
        }

        let document = Document::from(html);
        let mut results = Vec::new();

        // Look for DuckDuckGo search result containers
        // DuckDuckGo uses .result__body for individual result containers
        for (rank, node) in document.find(Class("result__body")).take(limit).enumerate() {
            // Extract title and URL from a.result__a element
            let title_link = node.find(Name("a").and(Class("result__a"))).next();

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
                        format!("https://duckduckgo.com{}", href)
                    } else {
                        href.to_string()
                    }
                })
                .unwrap_or_default();

            // Extract snippet from .result__snippet element
            let snippet = node
                .find(Class("result__snippet"))
                .next()
                .map(|n| n.text().trim().to_string())
                .unwrap_or_default();

            // Only add if we have at least a title
            if !title.is_empty() {
                let result = SearchResult {
                    title,
                    url,
                    snippet,
                    rank: rank + 1,
                };
                info!(
                    "Extracted DuckDuckGo result #{}: {}",
                    rank + 1,
                    result.title
                );
                results.push(result);
            }
        }

        info!(
            "Successfully parsed {} DuckDuckGo search results from HTML",
            results.len()
        );
        Ok(results)
    }

    fn name(&self) -> &str {
        "DuckDuckGoParser"
    }

    fn supports(&self, engine_type: &SearchEngineType) -> bool {
        matches!(engine_type, SearchEngineType::DuckDuckGo)
    }
}

impl Default for DuckDuckGoParser {
    fn default() -> Self {
        Self::new()
    }
}
