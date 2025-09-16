use tarzi::{config::Config, search::SearchEngine};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load configuration with proper precedence
    let mut config = Config::load()?;
    config.fetcher.mode = "browser_head".to_string();

    // Create search engine from config
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
