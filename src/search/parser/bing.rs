use super::SearchResultParser;
use crate::Result;
use crate::search::types::{SearchEngineType, SearchResult};
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use tracing::{info, warn};

pub struct BingParser;

impl BingParser {
    pub fn new() -> Self {
        Self
    }
}

impl SearchResultParser for BingParser {
    fn parse(&self, html: &str, limit: usize) -> Result<Vec<SearchResult>> {
        info!("Parsing Bing search results from HTML");

        if html.is_empty() {
            warn!("Empty HTML provided to BingParser");
            return Ok(Vec::new());
        }

        let document = Document::from(html);
        let mut results = Vec::new();

        // Look for Bing search result containers
        for (rank, node) in document.find(Class("b_algo")).take(limit).enumerate() {
            // Extract title and URL from h2 > a element
            let title_link = node.find(Name("h2").descendant(Name("a"))).next();

            let title = title_link
                .map(|n| n.text().trim().to_string())
                .unwrap_or_default();

            let url = title_link
                .and_then(|n| n.attr("href"))
                .map(|href| {
                    // Bing sometimes uses relative URLs or has tracking parameters
                    if href.starts_with("http") {
                        href.to_string()
                    } else if href.starts_with("/") {
                        format!("https://www.bing.com{}", href)
                    } else {
                        href.to_string()
                    }
                })
                .unwrap_or_default();

            // Extract snippet from .b_caption p element
            let snippet = node
                .find(Class("b_caption").descendant(Name("p")))
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
                info!("Extracted Bing result #{}: {}", rank + 1, result.title);
                results.push(result);
            }
        }

        info!(
            "Successfully parsed {} Bing search results from HTML",
            results.len()
        );
        Ok(results)
    }

    fn name(&self) -> &str {
        "BingParser"
    }

    fn supports(&self, engine_type: &SearchEngineType) -> bool {
        matches!(engine_type, SearchEngineType::Bing)
    }
}

impl Default for BingParser {
    fn default() -> Self {
        Self::new()
    }
}
