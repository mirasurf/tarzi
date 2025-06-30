use tarzi::{
    config::Config,
    search::{SearchEngine, SearchMode},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = Config::load_dev()?;
    println!("Loaded config with search engine: {}", config.search.engine);
    println!("Query pattern: {}", config.search.query_pattern);

    // Create search engine from config
    let mut search_engine = SearchEngine::from_config(&config);

    // Perform a search
    let query = "Rust programming language";
    println!("\nSearching for: '{query}'");

    let results = search_engine
        .search(query, SearchMode::WebQuery, config.search.limit)
        .await?;

    println!("\nFound {} results:", results.len());
    for (i, result) in results.iter().enumerate() {
        println!("{}. {}", i + 1, result.title);
        println!("   URL: {}", result.url);
        println!("   Snippet: {}", result.snippet);
        println!();
    }

    Ok(())
}
