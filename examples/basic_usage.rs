use tarsier::{
    Result,
    converter::{Converter, Format},
    fetcher::WebFetcher,
    search::{SearchEngine, SearchMode},
};

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== Tarsier Basic Usage Example ===\n");

    // 1. HTML to Markdown conversion
    println!("1. Converting HTML to Markdown:");
    let html_input = r#"
        <html>
            <head><title>Example Page</title></head>
            <body>
                <h1>Welcome to Tarsier</h1>
                <p>This is a <strong>test</strong> page with <a href="https://example.com">a link</a>.</p>
                <img src="image.jpg" alt="Test image">
            </body>
        </html>
    "#;

    let converter = Converter::new();
    let markdown = converter.convert(html_input, Format::Markdown).await?;
    println!("Markdown output:\n{}\n", markdown);

    // 2. HTML to JSON conversion
    println!("2. Converting HTML to JSON:");
    let json_output = converter.convert(html_input, Format::Json).await?;
    println!("JSON output:\n{}\n", json_output);

    // 3. Web page fetching (without JavaScript)
    println!("3. Fetching web page (without JavaScript):");
    let fetcher = WebFetcher::new();
    match fetcher.fetch("https://httpbin.org/html").await {
        Ok(content) => {
            println!("Successfully fetched page ({} characters)", content.len());
            let markdown = converter.convert(&content, Format::Markdown).await?;
            println!(
                "Converted to markdown (first 200 chars):\n{}...\n",
                &markdown[..markdown.len().min(200)]
            );
        }
        Err(e) => {
            println!("Failed to fetch page: {}\n", e);
        }
    }

    // 4. Search functionality (browser mode)
    println!("4. Search functionality (browser mode):");
    let mut search_engine = SearchEngine::new();
    match search_engine
        .search("Rust programming", SearchMode::Browser, 3)
        .await
    {
        Ok(results) => {
            println!("Found {} search results:", results.len());
            for (i, result) in results.iter().enumerate() {
                println!("  {}. {} ({})", i + 1, result.title, result.url);
                println!("     {}", result.snippet);
            }
        }
        Err(e) => {
            println!("Search failed: {}\n", e);
        }
    }

    // 5. Search functionality (API mode)
    println!("5. Search functionality (API mode):");
    let mut api_search_engine = SearchEngine::new().with_api_key("demo_key".to_string());
    match api_search_engine
        .search("Python programming", SearchMode::Api, 2)
        .await
    {
        Ok(results) => {
            println!("Found {} API search results:", results.len());
            for (i, result) in results.iter().enumerate() {
                println!("  {}. {} ({})", i + 1, result.title, result.url);
                println!("     {}", result.snippet);
            }
        }
        Err(e) => {
            println!("API search failed: {}\n", e);
        }
    }

    println!("=== Example completed successfully! ===");
    Ok(())
}
