use super::base::{BaseParser, BaseParserImpl};
use crate::search::types::{SearchEngineType, SearchResult};
use crate::Result;
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
                            } else if href.starts_with("//") {
                                format!("https:{href}")
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
                                } else if href.starts_with("//") {
                                    format!("https:{href}")
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::search::types::SearchEngineType;

    #[test]
    fn test_duckduckgo_parser() {
        let parser = DuckDuckGoParser::new();
        let html = r#"
        <html>
            <body>
                <div class="result__body">
                    <a class="result__a" href="https://example1.com">DuckDuckGo Test Result 1</a>
                    <div class="result__snippet">This is a test snippet for DuckDuckGo 1</div>
                </div>
                <div class="result__body">
                    <a class="result__a" href="https://example2.com">DuckDuckGo Test Result 2</a>
                    <div class="result__snippet">This is a test snippet for DuckDuckGo 2</div>
                </div>
                <div class="result__body">
                    <a class="result__a" href="https://example3.com">DuckDuckGo Test Result 3</a>
                    <div class="result__snippet">This is a test snippet for DuckDuckGo 3</div>
                </div>
            </body>
        </html>
        "#;
        let results = parser.parse(html, 2).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(parser.name(), "DuckDuckGoParser");
        assert!(parser.supports(&SearchEngineType::DuckDuckGo));

        assert_eq!(results[0].title, "DuckDuckGo Test Result 1");
        assert_eq!(results[0].url, "https://example1.com");
        assert_eq!(
            results[0].snippet,
            "This is a test snippet for DuckDuckGo 1"
        );
        assert_eq!(results[0].rank, 1);

        assert_eq!(results[1].title, "DuckDuckGo Test Result 2");
        assert_eq!(results[1].url, "https://example2.com");
        assert_eq!(
            results[1].snippet,
            "This is a test snippet for DuckDuckGo 2"
        );
        assert_eq!(results[1].rank, 2);
    }

    #[test]
    fn test_duckduckgo_parser_empty_and_edge_cases() {
        let parser = DuckDuckGoParser::new();

        // Test empty HTML
        let results = parser.parse("", 5).unwrap();
        assert!(results.is_empty());

        // Test zero limit
        let html = r#"<html><body><article><a class="result__a" href="https://example.com">Test</a></article></body></html>"#;
        let results = parser.parse(html, 0).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_duckduckgo_parser_article_structure() {
        let parser = DuckDuckGoParser::new();
        let html = r#"
        <html>
            <body>
                <article>
                    <a class="eVNpHGjtxRBq_gLOfGDr" href="https://example1.com">Modern Article 1</a>
                    <div class="OgdwYG6KE2qthn9XQWFC">Modern snippet 1</div>
                </article>
                <article>
                    <a class="LQNqh2U1kzYxREs65IJu" href="https://example2.com">Modern Article 2</a>
                    <div class="kY2IgmnCmOGjharHErah">Modern snippet 2</div>
                </article>
            </body>
        </html>
        "#;
        let results = parser.parse(html, 10).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].title, "Modern Article 1");
        assert_eq!(results[0].snippet, "Modern snippet 1");
        assert_eq!(results[1].title, "Modern Article 2");
        assert_eq!(results[1].snippet, "Modern snippet 2");
    }

    #[test]
    fn test_duckduckgo_parser_url_deduplication() {
        let parser = DuckDuckGoParser::new();
        let html = r#"
        <html>
            <body>
                <article>
                    <a class="result__a" href="https://duplicate.com">First</a>
                </article>
                <article>
                    <a class="result__a" href="https://duplicate.com">Duplicate</a>
                </article>
                <article>
                    <a class="result__a" href="https://unique.com">Unique</a>
                </article>
            </body>
        </html>
        "#;
        let results = parser.parse(html, 10).unwrap();
        assert_eq!(results.len(), 2); // Should deduplicate
        assert_eq!(results[0].url, "https://duplicate.com");
        assert_eq!(results[1].url, "https://unique.com");
    }

    #[test]
    fn test_duckduckgo_parser_url_normalization() {
        let parser = DuckDuckGoParser::new();
        let html = r#"
        <html>
            <body>
                <article>
                    <a class="result__a" href="/relative">Relative URL</a>
                </article>
                <article>
                    <a class="result__a" href="//protocol-relative.com">Protocol-relative</a>
                </article>
            </body>
        </html>
        "#;
        let results = parser.parse(html, 10).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].url, "https://duckduckgo.com/relative");
        assert_eq!(results[1].url, "https://protocol-relative.com");
    }

    #[test]
    fn test_duckduckgo_parser_fallback_selector() {
        let parser = DuckDuckGoParser::new();
        let html = r#"
        <html>
            <body>
                <div class="result__body">
                    <a href="https://fallback.com">Fallback Title</a>
                    <div class="result__snippet">Fallback snippet</div>
                </div>
            </body>
        </html>
        "#;
        let results = parser.parse(html, 10).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Fallback Title");
        assert_eq!(results[0].snippet, "Fallback snippet");
    }

    #[test]
    fn test_duckduckgo_parser_missing_data() {
        let parser = DuckDuckGoParser::new();
        let html = r#"
        <html>
            <body>
                <article>
                    <a>No href</a>
                </article>
                <article>
                    <a href="">Empty href</a>
                </article>
                <article>
                    <a href="https://good.com">Good result</a>
                </article>
            </body>
        </html>
        "#;
        let results = parser.parse(html, 10).unwrap();
        assert_eq!(results.len(), 1); // Only the good result
        assert_eq!(results[0].url, "https://good.com");
        assert_eq!(results[0].title, "Good result");
    }
}
