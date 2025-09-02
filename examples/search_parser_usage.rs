use tarzi::search::{ParserFactory, SearchEngineType};

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
        (SearchEngineType::DuckDuckGo, "DuckDuckGo"),
        (SearchEngineType::Google, "Google"),
        (SearchEngineType::BraveSearch, "Brave Search"),
        (SearchEngineType::Baidu, "Baidu"),
    ];

    println!("Testing parsers for different search engines:");
    for (engine_type, name) in &engines {
        println!("- {}: {}", name, engine_type.get_query_pattern());
    }

    // Test with mock JSON content (simulating API response)
    println!("\n=== Testing API Parser with Mock JSON ===");
    let mock_json = r#"{"results": [{"title": "Test Result", "url": "https://example.com", "text": "Test snippet"}]}"#;

    // Test each engine type with the mock JSON
    for (engine_type, name) in &engines {
        println!("\nTesting {name} parser:");
        let parser = factory.get_parser(engine_type);
        println!("  Parser name: {}", parser.name());
        println!("  Supports {}: {}", name, parser.supports(engine_type));

        // Parse the mock JSON
        match parser.parse(mock_json, 5) {
            Ok(results) => {
                println!("  Parsed {} results", results.len());
                for (i, result) in results.iter().enumerate() {
                    println!("    {}. {} - {}", i + 1, result.title, result.url);
                }
            }
            Err(e) => {
                println!("  Parse error: {e}");
            }
        }
    }

    println!("\n=== Parser Examples Complete ===");
    Ok(())
}
