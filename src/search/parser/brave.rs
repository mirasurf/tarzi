use super::SearchResultParser;
use crate::Result;
use crate::search::types::{SearchEngineType, SearchResult};
use select::document::Document;
use select::predicate::{Class, Name};
use tracing::{info, warn};

pub struct BraveParser;

impl BraveParser {
    pub fn new() -> Self {
        Self
    }
}

impl SearchResultParser for BraveParser {
    fn parse(&self, html: &str, limit: usize) -> Result<Vec<SearchResult>> {
        info!("Parsing Brave search results from HTML");

        if html.is_empty() {
            warn!("Empty HTML provided to BraveParser");
            return Ok(Vec::new());
        }

        let document = Document::from(html);
        let mut results = Vec::new();

        // Look for Brave search result containers
        // Brave uses .result-row for individual result containers
        for (rank, node) in document.find(Class("result-row")).take(limit).enumerate() {
            // Extract title and URL from a element
            let title_link = node.find(Name("a")).next();

            let title = title_link
                .map(|n| n.text().trim().to_string())
                .unwrap_or_default();

            let url = title_link
                .and_then(|n| n.attr("href"))
                .map(|href| {
                    // Brave sometimes uses relative URLs or redirect URLs
                    if href.starts_with("http") {
                        href.to_string()
                    } else if href.starts_with("/") {
                        format!("https://search.brave.com{href}")
                    } else {
                        href.to_string()
                    }
                })
                .unwrap_or_default();

            // Extract snippet from .result-snippet element
            let snippet = node
                .find(Class("result-snippet"))
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
                info!("Extracted Brave result #{}: {}", rank + 1, result.title);
                results.push(result);
            }
        }

        info!(
            "Successfully parsed {} Brave search results from HTML",
            results.len()
        );
        Ok(results)
    }

    fn name(&self) -> &str {
        "BraveParser"
    }

    fn supports(&self, engine_type: &SearchEngineType) -> bool {
        matches!(engine_type, SearchEngineType::BraveSearch)
    }
}

impl Default for BraveParser {
    fn default() -> Self {
        Self::new()
    }
}
