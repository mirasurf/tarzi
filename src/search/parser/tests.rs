use super::{
    BaiduParser, BingParser, BraveParser, DuckDuckGoParser, GoogleParser, ParserFactory,
    SearchResultParser,
};
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

    // Check first result
    assert_eq!(results[0].title, "Test Result 1");
    assert_eq!(results[0].url, "https://example1.com");
    assert_eq!(results[0].snippet, "This is a test snippet 1");
    assert_eq!(results[0].rank, 1);
}

#[test]
fn test_google_parser() {
    let parser = GoogleParser::new();
    let html = r#"
        <html>
            <body>
                <div class="tF2Cxc">
                    <div class="yuRUbf">
                        <a href="https://example1.com">Google Test Result 1</a>
                    </div>
                    <div class="IsZvec">This is a test snippet for Google 1</div>
                </div>
                <div class="tF2Cxc">
                    <div class="yuRUbf">
                        <a href="https://example2.com">Google Test Result 2</a>
                    </div>
                    <div class="IsZvec">This is a test snippet for Google 2</div>
                </div>
                <div class="tF2Cxc">
                    <div class="yuRUbf">
                        <a href="https://example3.com">Google Test Result 3</a>
                    </div>
                    <div class="IsZvec">This is a test snippet for Google 3</div>
                </div>
            </body>
        </html>
        "#;
    let results = parser.parse(html, 2).unwrap();

    assert_eq!(results.len(), 2);
    assert_eq!(parser.name(), "GoogleParser");
    assert!(parser.supports(&SearchEngineType::Google));
    assert!(!parser.supports(&SearchEngineType::Bing));

    // Check first result
    assert_eq!(results[0].title, "Google Test Result 1");
    assert_eq!(results[0].url, "https://example1.com");
    assert_eq!(results[0].snippet, "This is a test snippet for Google 1");
    assert_eq!(results[0].rank, 1);

    // Check second result
    assert_eq!(results[1].title, "Google Test Result 2");
    assert_eq!(results[1].url, "https://example2.com");
    assert_eq!(results[1].snippet, "This is a test snippet for Google 2");
    assert_eq!(results[1].rank, 2);
}

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

    // Check first result
    assert_eq!(results[0].title, "DuckDuckGo Test Result 1");
    assert_eq!(results[0].url, "https://example1.com");
    assert_eq!(
        results[0].snippet,
        "This is a test snippet for DuckDuckGo 1"
    );
    assert_eq!(results[0].rank, 1);

    // Check second result
    assert_eq!(results[1].title, "DuckDuckGo Test Result 2");
    assert_eq!(results[1].url, "https://example2.com");
    assert_eq!(
        results[1].snippet,
        "This is a test snippet for DuckDuckGo 2"
    );
    assert_eq!(results[1].rank, 2);
}

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

    // Check first result
    assert_eq!(results[0].title, "Brave Test Result 1");
    assert_eq!(results[0].url, "https://example1.com");
    assert_eq!(results[0].snippet, "This is a test snippet for Brave 1");
    assert_eq!(results[0].rank, 1);

    // Check second result
    assert_eq!(results[1].title, "Brave Test Result 2");
    assert_eq!(results[1].url, "https://example2.com");
    assert_eq!(results[1].snippet, "This is a test snippet for Brave 2");
    assert_eq!(results[1].rank, 2);
}

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

    // Check first result
    assert_eq!(results[0].title, "Baidu Test Result 1");
    assert_eq!(results[0].url, "https://example1.com");
    assert_eq!(results[0].snippet, "This is a test snippet for Baidu 1");
    assert_eq!(results[0].rank, 1);

    // Check second result (should be the third div, skipping the ad)
    assert_eq!(results[1].title, "Baidu Test Result 2");
    assert_eq!(results[1].url, "https://example2.com");
    assert_eq!(results[1].snippet, "This is a test snippet for Baidu 2");
    assert_eq!(results[1].rank, 2);
}

// Custom parser tests removed - custom engines are no longer supported

#[test]
fn test_parser_factory() {
    let factory = ParserFactory::new();

    // Test that we get the correct parser for each engine type
    let bing_parser = factory.get_parser(&SearchEngineType::Bing);
    assert_eq!(bing_parser.name(), "BingParser");

    let duckduckgo_parser = factory.get_parser(&SearchEngineType::DuckDuckGo);
    assert_eq!(duckduckgo_parser.name(), "DuckDuckGoParser");

    let google_parser = factory.get_parser(&SearchEngineType::Google);
    assert_eq!(google_parser.name(), "GoogleParser");

    let brave_parser = factory.get_parser(&SearchEngineType::BraveSearch);
    assert_eq!(brave_parser.name(), "BraveParser");

    let baidu_parser = factory.get_parser(&SearchEngineType::Baidu);
    assert_eq!(baidu_parser.name(), "BaiduParser");
}

#[test]
fn test_parser_support() {
    let factory = ParserFactory::new();

    // Test that each parser supports its own engine type
    let parsers = vec![
        ("BingParser", factory.get_parser(&SearchEngineType::Bing)),
        (
            "DuckDuckGoParser",
            factory.get_parser(&SearchEngineType::DuckDuckGo),
        ),
        (
            "GoogleParser",
            factory.get_parser(&SearchEngineType::Google),
        ),
        (
            "BraveParser",
            factory.get_parser(&SearchEngineType::BraveSearch),
        ),
        ("BaiduParser", factory.get_parser(&SearchEngineType::Baidu)),
    ];

    for (name, parser) in parsers {
        assert!(
            parser.supports(&SearchEngineType::Bing)
                || parser.supports(&SearchEngineType::DuckDuckGo)
                || parser.supports(&SearchEngineType::Google)
                || parser.supports(&SearchEngineType::BraveSearch)
                || parser.supports(&SearchEngineType::Baidu),
            "Parser {name} should support at least one engine type"
        );
    }
}

#[test]
fn test_all_parsers_with_different_limits() {
    let factory = ParserFactory::new();
    let html = "<html><body>Test content</body></html>";

    let parsers = vec![
        ("BingParser", factory.get_parser(&SearchEngineType::Bing)),
        (
            "GoogleParser",
            factory.get_parser(&SearchEngineType::Google),
        ),
        (
            "DuckDuckGoParser",
            factory.get_parser(&SearchEngineType::DuckDuckGo),
        ),
        (
            "BraveParser",
            factory.get_parser(&SearchEngineType::BraveSearch),
        ),
        ("BaiduParser", factory.get_parser(&SearchEngineType::Baidu)),
    ];

    for (name, parser) in parsers {
        assert_eq!(parser.name(), name);

        // Test with different limits
        for limit in [1, 5, 10] {
            let results = parser.parse(html, limit).unwrap();
            assert!(results.len() <= limit);
            assert!(results.len() <= 10); // All our mock parsers limit to 10

            // Verify ranking is correct
            for (i, result) in results.iter().enumerate() {
                assert_eq!(result.rank, i + 1);
            }
        }
    }
}
