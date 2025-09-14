use std::str::FromStr;
use tarzi::{
    config::Config,
    search::{types::SearchEngineType, SearchEngine},
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load configuration with proper precedence
    let mut config = Config::load()?;

    // Configure for SougouWeixin search
    config.search.engine = "sogou_weixin".to_string();
    config.search.limit = 10; // Search for up to 10 WeChat articles

    // Reset query pattern to use engine-specific pattern
    let engine_type = SearchEngineType::from_str(&config.search.engine).unwrap();
    config.search.query_pattern = engine_type.get_query_pattern();

    // Configure to use Chrome driver by default
    config.fetcher.web_driver = "chromedriver".to_string();

    // Use head browser mode (visible window) to potentially bypass anti-bot detection
    config.fetcher.mode = "browser_head".to_string();

    println!("Configured search engine: {}", config.search.engine);
    println!("Query pattern: {}", config.search.query_pattern);
    println!(
        "Fetcher mode: {} (visible browser window)",
        config.fetcher.mode
    );

    // Create search engine from config
    let mut search_engine = SearchEngine::from_config(&config);

    // Query about Oracle Corporation stock price in Chinese
    let query = "甲骨文股价";
    println!("\nSearching WeChat articles for: '{query}'");
    println!(
        "This will search for articles about Oracle Corporation stock prices on WeChat platform"
    );

    // Perform the search
    match search_engine.search(query, config.search.limit).await {
        Ok(results) => {
            println!("\nFound {} WeChat articles:", results.len());

            if results.is_empty() {
                println!("No results found. This might be due to:");
                println!("- Network connectivity issues");
                println!("- SougouWeixin blocking automated requests");
                println!("- No recent articles matching the query");
                println!("- WebDriver not available (run 'make setup-drivers' to install)");
            } else {
                for (i, result) in results.iter().enumerate() {
                    println!("\n{}. {}", i + 1, result.title);
                    println!("   URL: {}", result.url);
                    if !result.snippet.is_empty() {
                        println!("   Snippet: {}", result.snippet);
                    }
                    println!("   Rank: {}", result.rank);
                }

                println!("\n=== Search Summary ===");
                println!("Total results: {}", results.len());
                println!("All results are from mp.weixin.qq.com (WeChat articles)");
            }
        }
        Err(e) => {
            eprintln!("Search failed: {}", e);
            eprintln!("\nPossible causes:");
            eprintln!("- Network connectivity issues");
            eprintln!("- WebDriver (Chrome/Firefox) not available");
            eprintln!("- SougouWeixin anti-bot measures (CAPTCHA protection)");
            eprintln!("- Configuration issues");
            eprintln!("\nTo install WebDriver dependencies:");
            eprintln!("- Run: make setup-drivers (requires Homebrew)");
            eprintln!("- Or manually install:");
            eprintln!("  • ChromeDriver: brew install --cask chromedriver");
            eprintln!("  • GeckoDriver: brew install geckodriver");
            eprintln!("\nNote: SougouWeixin has strong anti-bot protection and may require");
            eprintln!("manual browser access or alternative search engines for automated use.");

            return Err(e);
        }
    }

    // Ensure clean shutdown of browser and driver resources
    search_engine.shutdown().await;

    Ok(())
}
