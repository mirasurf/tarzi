use std::str::FromStr;
use tarzi::{
    config::Config,
    search::{types::SearchEngineType, SearchEngine},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load configuration with proper precedence
    let mut config = Config::load()?;
    config.fetcher.mode = "browser_head".to_string();
    config.search.engine = "brave".to_string();
    // Try to make the browser appear more like a regular user
    config.fetcher.user_agent = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string();
    let engine_type = SearchEngineType::from_str(&config.search.engine).unwrap();
    config.search.query_pattern = engine_type.get_query_pattern();

    // Create search engine from config (fresh instance)
    let mut search_engine = SearchEngine::from_config(&config);

    // Perform a search
    let query = "agentic AI";
    let results = search_engine.search(query, config.search.limit).await?;

    println!("\nFound {} results:", results.len());
    for (i, result) in results.iter().enumerate() {
        println!("{}. {}", i + 1, result.title);
        println!("   URL: {}", result.url);
        println!("   Snippet: {}", result.snippet);
        println!();
    }

    search_engine.shutdown().await;

    Ok(())
}
