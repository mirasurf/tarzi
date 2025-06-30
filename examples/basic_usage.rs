use tarzi::{FetchMode, Format, Result, SearchEngine, SearchMode, WebFetcher};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("=== Tarzi Modular Example ===\n");

    // Example 1: Using the fetcher module directly
    println!("1. Fetching content with different modes:");

    let mut fetcher = WebFetcher::new();
    let test_url = tarzi::constants::HTTPBIN_HTML_URL;

    // Plain request mode
    match fetcher
        .fetch(test_url, FetchMode::PlainRequest, Format::Html)
        .await
    {
        Ok(content) => println!("   Plain request: {} characters", content.len()),
        Err(e) => println!("   Plain request failed: {e}"),
    }

    // Browser headless mode
    match fetcher
        .fetch(test_url, FetchMode::BrowserHeadless, Format::Markdown)
        .await
    {
        Ok(content) => println!("   Browser headless: {} characters", content.len()),
        Err(e) => println!("   Browser headless failed: {e}"),
    }

    println!();

    // Example 2: Using the search module
    println!("2. Searching and fetching content:");

    let mut search_engine = SearchEngine::new();
    let query = "rust programming";

    // Search for results
    match search_engine.search(query, SearchMode::WebQuery, 3).await {
        Ok(results) => {
            println!("   Found {} search results", results.len());
            for (i, result) in results.iter().enumerate() {
                println!("   {}. {} - {}", i + 1, result.title, result.url);
            }
        }
        Err(e) => println!("   Search failed: {e}"),
    }

    println!();

    // Example 3: Search and fetch content for each result
    println!("3. Search and fetch content for each result:");

    match search_engine
        .search_and_fetch(
            query,
            SearchMode::WebQuery,
            2,
            FetchMode::PlainRequest,
            Format::Json,
        )
        .await
    {
        Ok(results_with_content) => {
            println!(
                "   Successfully fetched content for {}/{} results",
                results_with_content.len(),
                2
            );
            for (i, (result, content)) in results_with_content.iter().enumerate() {
                println!(
                    "   {}. {} ({} characters)",
                    i + 1,
                    result.title,
                    content.len()
                );
            }
        }
        Err(e) => println!("   Search and fetch failed: {e}"),
    }

    println!();
    println!("=== Example completed ===");

    Ok(())
}
