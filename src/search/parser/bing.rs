use super::base::{BaseParser, BaseParserImpl};
use crate::search::types::{SearchEngineType, SearchResult};
use crate::Result;
use select::document::Document;
use select::predicate::{Class, Descendant, Name};

pub struct BingParser {
    base: BaseParserImpl,
}

impl BingParser {
    pub fn new() -> Self {
        Self {
            base: BaseParserImpl::new("BingParser".to_string(), SearchEngineType::Bing),
        }
    }
}

impl BaseParser for BingParser {
    fn name(&self) -> &str {
        self.base.name()
    }
    fn engine_type(&self) -> SearchEngineType {
        self.base.engine_type()
    }

    fn parse(&self, html: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let document = Document::from(html);
        let mut results = Vec::new();
        for node in document.find(Class("b_algo")) {
            // Check if we've reached the limit
            if results.len() >= limit {
                break;
            }

            let title_link = node.find(Descendant(Name("h2"), Name("a"))).next();
            let title = title_link
                .map(|n| n.text().trim().to_string())
                .unwrap_or_default();
            let url = title_link
                .and_then(|n| n.attr("href"))
                .map(|href| {
                    if href.starts_with("http") {
                        href.to_string()
                    } else if href.starts_with("/") {
                        format!("https://www.bing.com{href}")
                    } else {
                        href.to_string()
                    }
                })
                .unwrap_or_default();
            let snippet = node
                .find(Descendant(Class("b_caption"), Name("p")))
                .next()
                .map(|n| n.text().trim().to_string())
                .unwrap_or_default();
            if !title.is_empty() {
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

impl Default for BingParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::search::types::SearchEngineType;

    #[test]
    fn test_bing_parser() {
        let parser = BingParser::new();
        let html = r#"
        <html>
            <body>
                <li class="b_algo">
                    <h2><a href="https://example1.com">Test Result 1</a></h2>
                    <div class="b_caption"><p>This is a test snippet 1</p></div>
                </li>
                <li class="b_algo">
                    <h2><a href="https://example2.com">Test Result 2</a></h2>
                    <div class="b_caption"><p>This is a test snippet 2</p></div>
                </li>
            </body>
        </html>
        "#;
        let results = parser.parse(html, 3).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(parser.name(), "BingParser");
        assert!(parser.supports(&SearchEngineType::Bing));
        assert!(!parser.supports(&SearchEngineType::Google));

        assert_eq!(results[0].title, "Test Result 1");
        assert_eq!(results[0].url, "https://example1.com");
        assert_eq!(results[0].snippet, "This is a test snippet 1");
        assert_eq!(results[0].rank, 1);
    }

    #[test]
    fn test_bing_parser_empty_html() {
        let parser = BingParser::new();
        let results = parser.parse("", 5).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_bing_parser_no_results() {
        let parser = BingParser::new();
        let html = r#"
        <html>
            <body>
                <div class="no-results">No search results found</div>
            </body>
        </html>
        "#;
        let results = parser.parse(html, 5).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_bing_parser_limit_enforcement() {
        let parser = BingParser::new();
        let html = r#"
        <html>
            <body>
                <li class="b_algo">
                    <h2><a href="https://example1.com">Result 1</a></h2>
                    <div class="b_caption"><p>Snippet 1</p></div>
                </li>
                <li class="b_algo">
                    <h2><a href="https://example2.com">Result 2</a></h2>
                    <div class="b_caption"><p>Snippet 2</p></div>
                </li>
                <li class="b_algo">
                    <h2><a href="https://example3.com">Result 3</a></h2>
                    <div class="b_caption"><p>Snippet 3</p></div>
                </li>
            </body>
        </html>
        "#;
        let results = parser.parse(html, 2).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].rank, 1);
        assert_eq!(results[1].rank, 2);
    }

    #[test]
    fn test_bing_parser_url_normalization() {
        let parser = BingParser::new();
        let html = r#"
        <html>
            <body>
                <li class="b_algo">
                    <h2><a href="/relative/path">Relative URL</a></h2>
                    <div class="b_caption"><p>Test snippet</p></div>
                </li>
            </body>
        </html>
        "#;
        let results = parser.parse(html, 5).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].url, "https://www.bing.com/relative/path");
    }

    #[test]
    fn test_bing_parser_missing_elements() {
        let parser = BingParser::new();
        let html = r#"
        <html>
            <body>
                <li class="b_algo">
                    <h2><a href="https://example1.com">Title Only</a></h2>
                </li>
                <li class="b_algo">
                    <div class="b_caption"><p>Snippet without title</p></div>
                </li>
            </body>
        </html>
        "#;
        let results = parser.parse(html, 5).unwrap();
        assert_eq!(results.len(), 1); // Only the first one with title should be included
        assert_eq!(results[0].title, "Title Only");
        assert_eq!(results[0].snippet, ""); // No snippet for this result
    }
}
