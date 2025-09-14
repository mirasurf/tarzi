use super::base::{BaseParser, BaseParserImpl};
use crate::Result;
use crate::search::types::{SearchEngineType, SearchResult};
use select::document::Document;
use select::predicate::{Class, Name, Predicate};

/// DuckDuckGo web parser (HTML-based)
pub struct DuckDuckGoParser {
    base: BaseParserImpl,
}

impl DuckDuckGoParser {
    pub fn new() -> Self {
        Self {
            base: BaseParserImpl::new("DuckDuckGoParser".to_string(), SearchEngineType::DuckDuckGo),
        }
    }
}

impl BaseParser for DuckDuckGoParser {
    fn name(&self) -> &str {
        self.base.name()
    }

    fn engine_type(&self) -> SearchEngineType {
        self.base.engine_type()
    }

    fn parse(&self, html: &str, limit: usize) -> Result<Vec<SearchResult>> {
        use std::collections::HashSet;
        let document = Document::from(html);
        let mut results = Vec::new();
        let mut seen_urls = HashSet::new();

        // Try article elements first (modern structure)
        for result_element in document.find(Name("article")) {
            if results.len() >= limit {
                break;
            }

            // Process this result element
            if let Some(result) = self.process_result_element(&result_element, &mut seen_urls) {
                results.push(result);
            }
        }

        // If we still need more results, try legacy structures
        if results.len() < limit {
            for class_name in &["result__body", "serp-item", "result"] {
                if results.len() >= limit {
                    break;
                }

                for result_element in document.find(Class(*class_name)) {
                    if results.len() >= limit {
                        break;
                    }

                    // Process this result element
                    if let Some(result) =
                        self.process_result_element(&result_element, &mut seen_urls)
                    {
                        results.push(result);
                    }
                }
            }
        }

        // Set proper ranks for results
        for (i, result) in results.iter_mut().enumerate() {
            result.rank = i + 1;
        }

        Ok(results)
    }
}

impl DuckDuckGoParser {
    fn process_result_element(
        &self,
        result_element: &select::node::Node,
        seen_urls: &mut std::collections::HashSet<String>,
    ) -> Option<SearchResult> {
        // Title extraction
        let title_selectors = [
            Name("a").and(Class("eVNpHGjtxRBq_gLOfGDr")),
            Name("a").and(Class("LQNqh2U1kzYxREs65IJu")),
            // Legacy selectors as fallback
            Name("a").and(Class("result__a")),
            Name("a").and(Class("serp-item__title")),
            Name("a").and(Class("result__title")),
        ];

        let (title, url) = title_selectors
            .iter()
            .find_map(|sel| {
                result_element.find(*sel).next().map(|link| {
                    let title = link.text().trim().to_string();
                    let url = link
                        .attr("href")
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
                    (title, url)
                })
            })
            .unwrap_or_else(|| {
                // Fallback: any link
                result_element
                    .find(Name("a"))
                    .next()
                    .map(|link| {
                        let title = link.text().trim().to_string();
                        let url = link
                            .attr("href")
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
                        (title, url)
                    })
                    .unwrap_or_default()
            });

        if title.is_empty() || url.is_empty() || seen_urls.contains(&url) {
            return None;
        }

        // Snippet extraction
        let snippet_selectors = [
            Class("OgdwYG6KE2qthn9XQWFC"),
            Class("kY2IgmnCmOGjharHErah"),
            // Legacy selectors as fallback
            Class("result__snippet"),
            Class("serp-item__snippet"),
            Class("result__content"),
            Class("web-result__snippet"),
            Class("organic-result__snippet"),
            Class("result__extras"),
        ];
        let snippet = snippet_selectors
            .iter()
            .find_map(|sel| {
                result_element
                    .find(*sel)
                    .next()
                    .map(|el| el.text().trim().to_string())
            })
            .unwrap_or_default();

        seen_urls.insert(url.clone());
        Some(SearchResult {
            title,
            url,
            snippet,
            rank: 0, // Will be set by caller
        })
    }
}

impl Default for DuckDuckGoParser {
    fn default() -> Self {
        Self::new()
    }
}
