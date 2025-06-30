use super::SearchResultParser;
use crate::Result;
use crate::search::types::{SearchEngineType, SearchResult};
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use tracing::{info, warn};

pub struct GoogleParser;

impl GoogleParser {
    pub fn new() -> Self {
        Self
    }
}

impl SearchResultParser for GoogleParser {
    fn parse(&self, html: &str, limit: usize) -> Result<Vec<SearchResult>> {
        info!("Parsing Google search results from HTML");

        if html.is_empty() {
            warn!("Empty HTML provided to GoogleParser");
            return Ok(Vec::new());
        }

        let document = Document::from(html);
        let mut results = Vec::new();

        // Look for Google search result containers
        // Google uses .tF2Cxc for individual result containers
        for (rank, node) in document.find(Class("tF2Cxc")).take(limit).enumerate() {
            // Extract title and URL from .yuRUbf a element
            let title_link = node.find(Class("yuRUbf").descendant(Name("a"))).next();

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
            let snippet = node
                .find(Class("IsZvec"))
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
                info!("Extracted Google result #{}: {}", rank + 1, result.title);
                results.push(result);
            }
        }

        info!(
            "Successfully parsed {} Google search results from HTML",
            results.len()
        );
        Ok(results)
    }

    fn name(&self) -> &str {
        "GoogleParser"
    }

    fn supports(&self, engine_type: &SearchEngineType) -> bool {
        matches!(engine_type, SearchEngineType::Google)
    }
}

impl Default for GoogleParser {
    fn default() -> Self {
        Self::new()
    }
}
