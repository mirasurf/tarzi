use super::base::{BaseParser, BaseParserImpl};
use crate::Result;
use crate::search::types::{SearchEngineType, SearchResult};
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use std::collections::HashSet;

/// Google web parser (HTML-based)
pub struct GoogleParser {
    base: BaseParserImpl,
}

impl GoogleParser {
    pub fn new() -> Self {
        Self {
            base: BaseParserImpl::new("GoogleParser".to_string(), SearchEngineType::Google),
        }
    }
}

impl BaseParser for GoogleParser {
    fn name(&self) -> &str {
        self.base.name()
    }

    fn engine_type(&self) -> SearchEngineType {
        self.base.engine_type()
    }

    fn parse(&self, html: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let document = Document::from(html);
        let mut results = Vec::new();
        let mut seen_urls = HashSet::new();

        // Multiple selectors to handle different Google HTML structures
        let result_selectors = [
            Class("tF2Cxc"),    // Modern Google results
            Class("g"),         // Traditional Google results
            Class("rc"),        // Legacy Google results
            Class("result"),    // Alternative structure
            Class("serp-item"), // Another possible structure
        ];

        for selector in &result_selectors {
            if results.len() >= limit {
                break;
            }

            for result_element in document.find(*selector) {
                if results.len() >= limit {
                    break;
                }

                // Try multiple title/URL extraction strategies
                let (title, url) = self.extract_title_and_url(&result_element);

                if title.is_empty() || url.is_empty() || seen_urls.contains(&url) {
                    continue;
                }

                // Try multiple snippet extraction strategies
                let snippet = self.extract_snippet(&result_element);

                seen_urls.insert(url.clone());
                results.push(SearchResult {
                    title,
                    url,
                    snippet,
                    rank: results.len() + 1,
                });
            }
        }

        Ok(results)
    }
}

impl GoogleParser {
    fn extract_title_and_url(&self, element: &select::node::Node) -> (String, String) {
        // Try multiple strategies for title and URL extraction

        // Strategy 1: Modern Google structure with yuRUbf
        if let Some(link) = element.find(Class("yuRUbf").descendant(Name("a"))).next() {
            let title = link.text().trim().to_string();
            let url = link
                .attr("href")
                .map(|href| self.normalize_url(href))
                .unwrap_or_default();

            if !title.is_empty() && !url.is_empty() {
                return (title, url);
            }
        }

        // Strategy 2: Direct links with specific classes
        for class_name in &["LC20lb", "DKV0Md", "result__a", "serp-item__title"] {
            if let Some(link) = element.find(Name("a").and(Class(*class_name))).next() {
                let title = link.text().trim().to_string();
                let url = link
                    .attr("href")
                    .map(|href| self.normalize_url(href))
                    .unwrap_or_default();

                if !title.is_empty() && !url.is_empty() {
                    return (title, url);
                }
            }
        }

        // Strategy 3: Headers with links
        for tag_name in &["h3", "h2"] {
            if let Some(link) = element.find(Name(*tag_name).descendant(Name("a"))).next() {
                let title = link.text().trim().to_string();
                let url = link
                    .attr("href")
                    .map(|href| self.normalize_url(href))
                    .unwrap_or_default();

                if !title.is_empty() && !url.is_empty() {
                    return (title, url);
                }
            }
        }

        // Strategy 4: Fallback - any link with href
        if let Some(link) = element.find(Name("a")).next() {
            let title = link.text().trim().to_string();
            let url = link
                .attr("href")
                .map(|href| self.normalize_url(href))
                .unwrap_or_default();

            if !title.is_empty() && !url.is_empty() {
                return (title, url);
            }
        }

        (String::new(), String::new())
    }

    fn extract_snippet(&self, element: &select::node::Node) -> String {
        // Try multiple strategies for snippet extraction

        // Strategy 1: Modern Google structure
        for class_name in &["IsZvec", "VwiC3b", "yXK7lf"] {
            if let Some(snippet_element) = element.find(Class(*class_name)).next() {
                let snippet = snippet_element.text().trim().to_string();
                if !snippet.is_empty() {
                    return snippet;
                }
            }
        }

        // Strategy 2: Traditional structure
        for class_name in &["s", "st", "aCOpRe"] {
            if let Some(snippet_element) = element.find(Class(*class_name)).next() {
                let snippet = snippet_element.text().trim().to_string();
                if !snippet.is_empty() {
                    return snippet;
                }
            }
        }

        // Strategy 3: Alternative structures
        for class_name in &[
            "result__snippet",
            "serp-item__snippet",
            "web-result__snippet",
            "organic-result__snippet",
        ] {
            if let Some(snippet_element) = element.find(Class(*class_name)).next() {
                let snippet = snippet_element.text().trim().to_string();
                if !snippet.is_empty() {
                    return snippet;
                }
            }
        }

        // Strategy 4: Fallback - any div with snippet class
        if let Some(snippet_element) = element.find(Name("div").and(Class("snippet"))).next() {
            let snippet = snippet_element.text().trim().to_string();
            if !snippet.is_empty() {
                return snippet;
            }
        }

        String::new()
    }

    fn normalize_url(&self, href: &str) -> String {
        if href.starts_with("http://") || href.starts_with("https://") {
            href.to_string()
        } else if href.starts_with("/") {
            format!("https://www.google.com{href}")
        } else if href.starts_with("//") {
            format!("https:{href}")
        } else {
            href.to_string()
        }
    }
}

impl Default for GoogleParser {
    fn default() -> Self {
        Self::new()
    }
}
