use super::*;
use crate::search::types::{SearchEngineType, SearchResult};

#[cfg(test)]
mod tests {
    use super::*;

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
        let html = "<html><body>Mock HTML content</body></html>";
        let results = parser.parse(html, 5).unwrap();

        assert_eq!(results.len(), 5);
        assert_eq!(parser.name(), "GoogleParser");
        assert!(parser.supports(&SearchEngineType::Google));
        assert!(!parser.supports(&SearchEngineType::Bing));
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
    fn test_custom_parser() {
        let parser = CustomParser::new("TestEngine".to_string());
        let html = "<html><body>Mock HTML content</body></html>";
        let results = parser.parse(html, 4).unwrap();

        assert_eq!(results.len(), 4);
        assert_eq!(parser.name(), "TestEngine");
        assert!(parser.supports(&SearchEngineType::Custom("TestEngine".to_string())));
        assert!(!parser.supports(&SearchEngineType::Custom("OtherEngine".to_string())));

        // Check that results contain engine name
        assert!(results[0].title.contains("TestEngine"));
    }

    #[test]
    fn test_custom_parser_with_config() {
        let config = CustomParserConfig {
            result_container_selector: ".custom-result".to_string(),
            title_selector: ".custom-title".to_string(),
            url_selector: ".custom-url".to_string(),
            snippet_selector: ".custom-snippet".to_string(),
            custom_rules: std::collections::HashMap::new(),
        };

        let parser = CustomParser::with_config("CustomEngine".to_string(), config);
        let html = "<html><body>Mock HTML content</body></html>";
        let results = parser.parse(html, 2).unwrap();

        assert_eq!(results.len(), 2);
        assert!(results[0].snippet.contains(".custom-result"));
        assert!(results[0].snippet.contains(".custom-title"));
    }

    #[test]
    fn test_parser_factory() {
        let factory = ParserFactory::new();

        // Test all built-in parsers
        let bing_parser = factory.get_parser(&SearchEngineType::Bing);
        assert_eq!(bing_parser.name(), "BingParser");

        let google_parser = factory.get_parser(&SearchEngineType::Google);
        assert_eq!(google_parser.name(), "GoogleParser");

        let duckduckgo_parser = factory.get_parser(&SearchEngineType::DuckDuckGo);
        assert_eq!(duckduckgo_parser.name(), "DuckDuckGoParser");

        let brave_parser = factory.get_parser(&SearchEngineType::BraveSearch);
        assert_eq!(brave_parser.name(), "BraveParser");

        let tavily_parser = factory.get_parser(&SearchEngineType::Tavily);
        assert_eq!(tavily_parser.name(), "TavilyParser");

        let searchapi_parser = factory.get_parser(&SearchEngineType::SearchApi);
        assert_eq!(searchapi_parser.name(), "SearchApiParser");

        let custom_parser = factory.get_parser(&SearchEngineType::Custom("TestCustom".to_string()));
        assert_eq!(custom_parser.name(), "TestCustom");
    }

    #[test]
    fn test_parser_factory_with_custom_parser() {
        let mut factory = ParserFactory::new();

        // Register a custom parser
        let custom_parser = Box::new(CustomParser::new("MyCustomEngine".to_string()));
        factory.register_custom_parser("MyCustomEngine".to_string(), custom_parser);

        // Test that we can get the custom parser
        let parser = factory.get_parser(&SearchEngineType::Custom("MyCustomEngine".to_string()));
        assert_eq!(parser.name(), "MyCustomEngine");
    }

    #[test]
    fn test_all_parsers_with_different_limits() {
        let factory = ParserFactory::new();
        let html = "<html><body>Test content</body></html>";

        let test_cases = vec![
            (SearchEngineType::Bing, "BingParser"),
            (SearchEngineType::Google, "GoogleParser"),
            (SearchEngineType::DuckDuckGo, "DuckDuckGoParser"),
            (SearchEngineType::BraveSearch, "BraveParser"),
            (SearchEngineType::Tavily, "TavilyParser"),
            (SearchEngineType::SearchApi, "SearchApiParser"),
        ];

        for (engine_type, expected_name) in test_cases {
            let parser = factory.get_parser(&engine_type);
            assert_eq!(parser.name(), expected_name);

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
}
