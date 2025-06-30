use tarzi::search::parser::CustomParser;
use tarzi::search::{
    CustomParserConfig, ParserFactory, SearchEngine, SearchEngineType, SearchResultParser,
};

/// Example custom parser for a hypothetical search engine
struct MyCustomParser {
    name: String,
}

impl MyCustomParser {
    fn new(name: String) -> Self {
        Self { name }
    }
}

impl SearchResultParser for MyCustomParser {
    fn parse(&self, _html: &str, limit: usize) -> tarzi::Result<Vec<tarzi::search::SearchResult>> {
        use tarzi::search::SearchResult;

        let mut results = Vec::new();
        let mock_results_count = std::cmp::min(limit, 5);

        for i in 0..mock_results_count {
            let rank = i + 1;
            results.push(SearchResult {
                title: format!("{} - Custom Search Result #{}", self.name, rank),
                url: format!("https://mycustomengine.com/result/{rank}"),
                snippet: format!(
                    "This is a custom search result snippet from {} for result #{}",
                    self.name, rank
                ),
                rank,
            });
        }

        Ok(results)
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn supports(&self, engine_type: &SearchEngineType) -> bool {
        match engine_type {
            SearchEngineType::Custom(name) => name == &self.name,
            _ => false,
        }
    }
}

#[tokio::main]
async fn main() -> tarzi::Result<()> {
    // Initialize logging (optional - remove if tracing_subscriber is not in dependencies)
    // tracing_subscriber::fmt::init();

    println!("=== Tarzi Search Parser Examples ===\n");

    // 1. Basic usage with built-in parsers
    println!("1. Testing built-in search engine parsers:");
    let factory = ParserFactory::new();

    let engines = vec![
        (SearchEngineType::Bing, "Bing"),
        (SearchEngineType::Google, "Google"),
        (SearchEngineType::DuckDuckGo, "DuckDuckGo"),
        (SearchEngineType::BraveSearch, "Brave Search"),
        (SearchEngineType::Baidu, "Baidu"),
    ];

    for (engine_type, engine_name) in engines {
        println!("  Testing {engine_name} parser:");
        let parser = factory.get_parser(&engine_type);
        let mock_html = format!("<html><body>Mock HTML for {engine_name}</body></html>");
        let results = parser.parse(&mock_html, 3)?;

        for result in results {
            println!("    - {}", result.title);
            println!("      URL: {}", result.url);
            println!("      Rank: {}\n", result.rank);
        }
    }

    // 2. Custom parser configuration
    println!("2. Testing custom parser with configuration:");
    let custom_config = CustomParserConfig {
        result_container_selector: ".my-search-result".to_string(),
        title_selector: ".my-title".to_string(),
        url_selector: ".my-url".to_string(),
        snippet_selector: ".my-snippet".to_string(),
        ..Default::default()
    };

    let custom_parser = CustomParser::with_config("MySearchEngine".to_string(), custom_config);
    let results = custom_parser.parse("<html><body>Custom HTML</body></html>", 2)?;

    for result in results {
        println!("  - {}", result.title);
        println!("    URL: {}", result.url);
        println!("    Snippet: {}", result.snippet);
        println!("    Rank: {}\n", result.rank);
    }

    // 3. Using SearchEngine with custom parser
    println!("3. Using SearchEngine with custom parser registration:");
    let mut search_engine = SearchEngine::new();

    // Register a completely custom parser
    let my_parser = Box::new(MyCustomParser::new("SuperSearch".to_string()));
    search_engine.register_custom_parser("SuperSearch".to_string(), my_parser);

    // The SearchEngine will now use our custom parser when engine type is Custom("SuperSearch")
    println!("  Custom parser registered successfully!");

    // 4. Parser factory with custom parsers
    println!("4. Parser factory with registered custom parsers:");
    let mut factory = ParserFactory::new();

    // Register multiple custom parsers
    let engines = vec!["CustomEngine1", "CustomEngine2", "SpecialSearchEngine"];

    for engine_name in engines {
        let custom_parser = Box::new(MyCustomParser::new(engine_name.to_string()));
        factory.register_custom_parser(engine_name.to_string(), custom_parser);
        println!("  Registered custom parser: {engine_name}");
    }

    // Test the registered custom parsers
    for engine_name in ["CustomEngine1", "CustomEngine2", "SpecialSearchEngine"] {
        let engine_type = SearchEngineType::Custom(engine_name.to_string());
        let parser = factory.get_parser(&engine_type);
        let results = parser.parse("<html><body>Test</body></html>", 2)?;

        println!("  Results from {engine_name}:");
        for result in results {
            println!("    - {}", result.title);
        }
    }

    // 5. Demonstrating parser capabilities
    println!("\n5. Parser capabilities demonstration:");
    let factory = ParserFactory::new();

    let test_engines = vec![
        SearchEngineType::Bing,
        SearchEngineType::Google,
        SearchEngineType::Custom("TestEngine".to_string()),
    ];

    for engine_type in test_engines {
        let parser = factory.get_parser(&engine_type);
        println!(
            "  Parser: {} supports {:?}: {}",
            parser.name(),
            engine_type,
            parser.supports(&engine_type)
        );
    }

    println!("\n=== Parser Examples Complete ===");
    Ok(())
}
