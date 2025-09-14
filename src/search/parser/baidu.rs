use super::base::{BaseParser, BaseParserImpl};
use crate::search::types::{SearchEngineType, SearchResult};
use crate::Result;
use select::document::Document;
use select::predicate::{And, Class, Descendant, Name};

pub struct BaiduParser {
    base: BaseParserImpl,
}

impl BaiduParser {
    pub fn new() -> Self {
        Self {
            base: BaseParserImpl::new("BaiduParser".to_string(), SearchEngineType::Baidu),
        }
    }
}

impl BaseParser for BaiduParser {
    fn name(&self) -> &str {
        self.base.name()
    }
    fn engine_type(&self) -> SearchEngineType {
        self.base.engine_type()
    }

    fn parse(&self, html: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let document = Document::from(html);
        let mut results = Vec::new();

        if limit == 0 {
            return Ok(results);
        }

        let result_selector = And(Class("result"), Class("c-container"));
        for node in document.find(result_selector) {
            if results.len() >= limit {
                break;
            }

            // Skip ads
            if node.attr("data-tuiguang").is_some() {
                continue;
            }
            let title = node
                .find(Descendant(Name("h3"), Name("a")))
                .next()
                .map(|n| n.text().trim().to_string())
                .unwrap_or_default();
            let url = node
                .find(Descendant(Name("h3"), Name("a")))
                .next()
                .and_then(|n| n.attr("href"))
                .unwrap_or_default()
                .to_string();
            let snippet = node
                .find(Class("c-abstract"))
                .next()
                .map(|n| n.text().trim().to_string())
                .unwrap_or_default();
            if !title.is_empty() && !url.is_empty() {
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

impl Default for BaiduParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::search::types::SearchEngineType;

    #[test]
    fn test_baidu_parser() {
        let parser = BaiduParser::new();
        let html = r#"
        <html>
            <body>
                <div class="result c-container">
                    <h3><a href="https://example1.com">Baidu Test Result 1</a></h3>
                    <div class="c-abstract">This is a test snippet for Baidu 1</div>
                </div>
                <div class="result c-container" data-tuiguang="1">
                    <h3><a href="https://ad-example.com">Ad Result</a></h3>
                    <div class="c-abstract">This is an ad snippet</div>
                </div>
                <div class="result c-container">
                    <h3><a href="https://example2.com">Baidu Test Result 2</a></h3>
                    <div class="c-abstract">This is a test snippet for Baidu 2</div>
                </div>
                <div class="result c-container">
                    <h3><a href="https://example3.com">Baidu Test Result 3</a></h3>
                    <div class="c-abstract">This is a test snippet for Baidu 3</div>
                </div>
            </body>
        </html>
        "#;
        let results = parser.parse(html, 2).unwrap();

        // Should get 2 results, skipping the ad (data-tuiguang)
        assert_eq!(results.len(), 2);
        assert_eq!(parser.name(), "BaiduParser");
        assert!(parser.supports(&SearchEngineType::Baidu));
        assert!(!parser.supports(&SearchEngineType::Google));

        assert_eq!(results[0].title, "Baidu Test Result 1");
        assert_eq!(results[0].url, "https://example1.com");
        assert_eq!(results[0].snippet, "This is a test snippet for Baidu 1");
        assert_eq!(results[0].rank, 1);

        assert_eq!(results[1].title, "Baidu Test Result 2");
        assert_eq!(results[1].url, "https://example2.com");
        assert_eq!(results[1].snippet, "This is a test snippet for Baidu 2");
        assert_eq!(results[1].rank, 2);
    }

    #[test]
    fn test_baidu_parser_empty_and_edge_cases() {
        let parser = BaiduParser::new();

        // Test empty HTML
        let results = parser.parse("", 5).unwrap();
        assert!(results.is_empty());

        // Test zero limit
        let html = r#"<html><body><div class="result c-container"><h3><a href="https://example.com">Test</a></h3></div></body></html>"#;
        let results = parser.parse(html, 0).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_baidu_parser_ad_filtering() {
        let parser = BaiduParser::new();
        let html = r#"
        <html>
            <body>
                <div class="result c-container" data-tuiguang="1">
                    <h3><a href="https://ad1.com">Ad 1</a></h3>
                    <div class="c-abstract">Ad snippet 1</div>
                </div>
                <div class="result c-container" data-tuiguang="true">
                    <h3><a href="https://ad2.com">Ad 2</a></h3>
                    <div class="c-abstract">Ad snippet 2</div>
                </div>
                <div class="result c-container">
                    <h3><a href="https://organic.com">Organic Result</a></h3>
                    <div class="c-abstract">Organic snippet</div>
                </div>
            </body>
        </html>
        "#;
        let results = parser.parse(html, 10).unwrap();
        assert_eq!(results.len(), 1); // Only organic result should be included
        assert_eq!(results[0].url, "https://organic.com");
        assert_eq!(results[0].title, "Organic Result");
    }

    #[test]
    fn test_baidu_parser_missing_elements() {
        let parser = BaiduParser::new();
        let html = r#"
        <html>
            <body>
                <div class="result c-container">
                    <h3><a>No href</a></h3>
                </div>
                <div class="result c-container">
                    <h3><a href="">Empty href</a></h3>
                </div>
                <div class="result c-container">
                    <div class="c-abstract">No title</div>
                </div>
                <div class="result c-container">
                    <h3><a href="https://good.com">Good Result</a></h3>
                    <div class="c-abstract">Good snippet</div>
                </div>
            </body>
        </html>
        "#;
        let results = parser.parse(html, 10).unwrap();
        assert_eq!(results.len(), 1); // Only the good result
        assert_eq!(results[0].url, "https://good.com");
        assert_eq!(results[0].title, "Good Result");
        assert_eq!(results[0].snippet, "Good snippet");
    }

    #[test]
    fn test_baidu_parser_limit_enforcement() {
        let parser = BaiduParser::new();
        let html = r#"
        <html>
            <body>
                <div class="result c-container">
                    <h3><a href="https://example1.com">Result 1</a></h3>
                    <div class="c-abstract">Snippet 1</div>
                </div>
                <div class="result c-container">
                    <h3><a href="https://example2.com">Result 2</a></h3>
                    <div class="c-abstract">Snippet 2</div>
                </div>
                <div class="result c-container">
                    <h3><a href="https://example3.com">Result 3</a></h3>
                    <div class="c-abstract">Snippet 3</div>
                </div>
            </body>
        </html>
        "#;
        let results = parser.parse(html, 2).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].rank, 1);
        assert_eq!(results[1].rank, 2);
    }

    #[test]
    fn test_baidu_parser_empty_title_filtered() {
        let parser = BaiduParser::new();
        let html = r#"
        <html>
            <body>
                <div class="result c-container">
                    <h3><a href="https://example1.com">   </a></h3>
                    <div class="c-abstract">Has snippet but empty title</div>
                </div>
                <div class="result c-container">
                    <h3><a href="https://example2.com">Valid Title</a></h3>
                    <div class="c-abstract">Valid snippet</div>
                </div>
            </body>
        </html>
        "#;
        let results = parser.parse(html, 10).unwrap();
        assert_eq!(results.len(), 1); // Empty title should be filtered out
        assert_eq!(results[0].title, "Valid Title");
    }
}
