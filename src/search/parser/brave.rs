use super::base::{BaseParser, BaseParserImpl};
use crate::search::types::{SearchEngineType, SearchResult};
use crate::Result;
use select::document::Document;
use select::predicate::{Class, Name};

pub struct BraveParser {
    base: BaseParserImpl,
}

impl BraveParser {
    pub fn new() -> Self {
        Self {
            base: BaseParserImpl::new("BraveParser".to_string(), SearchEngineType::BraveSearch),
        }
    }
}

impl BaseParser for BraveParser {
    fn name(&self) -> &str {
        self.base.name()
    }
    fn engine_type(&self) -> SearchEngineType {
        self.base.engine_type()
    }

    fn parse(&self, html: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let document = Document::from(html);
        let mut results = Vec::new();
        for node in document.find(Class("result-row")) {
            // Check if we've reached the limit
            if results.len() >= limit {
                break;
            }

            let title_link = node.find(Name("a")).next();
            let title = title_link
                .map(|n| n.text().trim().to_string())
                .unwrap_or_default();
            let url = title_link
                .and_then(|n| n.attr("href"))
                .map(|href| {
                    if href.starts_with("http") {
                        href.to_string()
                    } else if href.starts_with("//") {
                        format!("https:{href}")
                    } else if href.starts_with("/") {
                        format!("https://search.brave.com{href}")
                    } else {
                        href.to_string()
                    }
                })
                .unwrap_or_default();
            let snippet = node
                .find(Class("result-snippet"))
                .next()
                .map(|n| n.text().trim().to_string())
                .unwrap_or_default();
            if !title.is_empty() && !url.is_empty() {
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

impl Default for BraveParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::search::types::SearchEngineType;

    #[test]
    fn test_brave_parser() {
        let parser = BraveParser::new();
        let html = r#"
        <html>
            <body>
                <div class="result-row">
                    <a href="https://example1.com">Brave Test Result 1</a>
                    <div class="result-snippet">This is a test snippet for Brave 1</div>
                </div>
                <div class="result-row">
                    <a href="https://example2.com">Brave Test Result 2</a>
                    <div class="result-snippet">This is a test snippet for Brave 2</div>
                </div>
                <div class="result-row">
                    <a href="https://example3.com">Brave Test Result 3</a>
                    <div class="result-snippet">This is a test snippet for Brave 3</div>
                </div>
            </body>
        </html>
        "#;
        let results = parser.parse(html, 2).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(parser.name(), "BraveParser");
        assert!(parser.supports(&SearchEngineType::BraveSearch));
        assert!(!parser.supports(&SearchEngineType::Google));

        assert_eq!(results[0].title, "Brave Test Result 1");
        assert_eq!(results[0].url, "https://example1.com");
        assert_eq!(results[0].snippet, "This is a test snippet for Brave 1");
        assert_eq!(results[0].rank, 1);

        assert_eq!(results[1].title, "Brave Test Result 2");
        assert_eq!(results[1].url, "https://example2.com");
        assert_eq!(results[1].snippet, "This is a test snippet for Brave 2");
        assert_eq!(results[1].rank, 2);
    }

    #[test]
    fn test_brave_parser_empty_and_edge_cases() {
        let parser = BraveParser::new();

        // Test empty HTML
        let results = parser.parse("", 5).unwrap();
        assert!(results.is_empty());

        // Test zero limit
        let html = r#"<html><body><div class="result-row"><a href="https://example.com">Test</a></div></body></html>"#;
        let results = parser.parse(html, 0).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_brave_parser_url_normalization() {
        let parser = BraveParser::new();
        let html = r#"
        <html>
            <body>
                <div class="result-row">
                    <a href="/relative/path">Relative URL</a>
                    <div class="result-snippet">Relative snippet</div>
                </div>
                <div class="result-row">
                    <a href="//protocol-relative.com">Protocol-relative</a>
                    <div class="result-snippet">Protocol snippet</div>
                </div>
            </body>
        </html>
        "#;
        let results = parser.parse(html, 10).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].url, "https://search.brave.com/relative/path");
        assert_eq!(results[1].url, "https://protocol-relative.com");
    }

    #[test]
    fn test_brave_parser_missing_elements() {
        let parser = BraveParser::new();
        let html = r#"
        <html>
            <body>
                <div class="result-row">
                    <a>No href attribute</a>
                </div>
                <div class="result-row">
                    <a href="">Empty href</a>
                </div>
                <div class="result-row">
                    <a href="https://good.com">Good result</a>
                    <div class="result-snippet">Good snippet</div>
                </div>
            </body>
        </html>
        "#;
        let results = parser.parse(html, 10).unwrap();
        assert_eq!(results.len(), 1); // Only the good result
        assert_eq!(results[0].url, "https://good.com");
        assert_eq!(results[0].title, "Good result");
        assert_eq!(results[0].snippet, "Good snippet");
    }

    #[test]
    fn test_brave_parser_limit_enforcement() {
        let parser = BraveParser::new();
        let html = r#"
        <html>
            <body>
                <div class="result-row">
                    <a href="https://example1.com">Result 1</a>
                </div>
                <div class="result-row">
                    <a href="https://example2.com">Result 2</a>
                </div>
                <div class="result-row">
                    <a href="https://example3.com">Result 3</a>
                </div>
            </body>
        </html>
        "#;
        let results = parser.parse(html, 2).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].rank, 1);
        assert_eq!(results[1].rank, 2);
    }
}
