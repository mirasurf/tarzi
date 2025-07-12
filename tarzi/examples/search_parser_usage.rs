use tarzi::SearchMode;
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
        (SearchEngineType::Google, "Google"),
        (SearchEngineType::DuckDuckGo, "DuckDuckGo"),
        (SearchEngineType::BraveSearch, "Brave Search"),
        (SearchEngineType::Baidu, "Baidu"),
    ];

    for (engine_type, engine_name) in engines {
        println!("  Testing {engine_name} parser:");
        let parser = factory.get_parser(&engine_type, SearchMode::WebQuery);
        let mock_html = format!("<html><body>Mock HTML for {engine_name}</body></html>");
        let results = parser.parse(&mock_html, 3)?;

        for result in results {
            println!("    - {}", result.title);
            println!("      URL: {}", result.url);
            println!("      Rank: {}\n", result.rank);
        }
    }

    // 2. Testing API parsers
    println!("2. Testing API parsers:");
    let api_engines = vec![
        (SearchEngineType::DuckDuckGo, "DuckDuckGo API"),
        (SearchEngineType::Google, "Google API"),
        (SearchEngineType::BraveSearch, "Brave API"),
        (SearchEngineType::Baidu, "Baidu API"),
        (SearchEngineType::Exa, "Exa API"),
        (SearchEngineType::Travily, "Travily API"),
    ];

    for (engine_type, engine_name) in api_engines {
        println!("  Testing {engine_name} parser:");
        let parser = factory.get_parser(&engine_type, SearchMode::ApiQuery);
        let mock_json = r#"{"results": [{"title": "Test Result", "url": "https://example.com", "text": "Test snippet"}]}"#;
        let results = parser.parse(mock_json, 3)?;

        for result in results {
            println!("    - {}", result.title);
            println!("      URL: {}", result.url);
            println!("      Rank: {}\n", result.rank);
        }
    }

    // 3. Using SearchEngine
    println!("3. Using SearchEngine:");
    println!("  SearchEngine created successfully!");

    // 4. Parser capabilities demonstration
    println!("\n4. Parser capabilities demonstration:");
    let test_engines = vec![
        SearchEngineType::Bing,
        SearchEngineType::Google,
        SearchEngineType::DuckDuckGo,
        SearchEngineType::BraveSearch,
        SearchEngineType::Baidu,
        SearchEngineType::Exa,
        SearchEngineType::Travily,
    ];

    for engine_type in test_engines {
        let parser = factory.get_parser(&engine_type, SearchMode::WebQuery);
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
