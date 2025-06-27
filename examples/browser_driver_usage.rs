use std::env;
use tarzi::{
    Result,
    config::Config,
    converter::Format,
    fetcher::{types::FetchMode, webfetcher::WebFetcher},
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("=== Browser Driver Integration Demo ===\n");

    // Load configuration
    let config = Config::load().unwrap_or_else(|_| {
        println!("Using default configuration (tarzi.toml not found)");
        Config::default()
    });

    // Create WebFetcher with configuration
    let mut fetcher = WebFetcher::from_config(&config);

    // Demo URL
    let test_url = tarzi::constants::HTTPBIN_HTML_URL;

    println!("Testing browser integration with URL: {}", test_url);
    println!();

    // Show current environment
    match env::var("TARZI_WEBDRIVER_URL") {
        Ok(url) if !url.is_empty() => {
            println!("✓ TARZI_WEBDRIVER_URL is set: {}", url);
            println!("  → Will use this URL with highest priority");
        }
        _ => {
            println!("ℹ TARZI_WEBDRIVER_URL is not set");
            println!("  → Will check for default webdriver at localhost:9515");
            println!("  → If not found, will try to start one with DriverManager");
        }
    }
    println!();

    // Test browser fetching
    println!("Attempting to fetch content using browser (headless mode)...");
    match fetcher
        .fetch(test_url, FetchMode::BrowserHeadless, Format::Html)
        .await
    {
        Ok(content) => {
            println!("✓ Successfully fetched content!");
            println!("Content length: {} characters", content.len());

            // Show if we're using a managed driver
            if let Some(driver_info) = fetcher.get_managed_driver_info() {
                println!("📱 Using managed driver:");
                println!("   Type: {:?}", driver_info.config.driver_type);
                println!("   Endpoint: {}", driver_info.endpoint);
                println!("   PID: {:?}", driver_info.pid);
                println!("   Started: {:?}", driver_info.started_at);
            } else {
                println!("🌐 Using external WebDriver server");
            }

            println!();
            println!("Content preview (first 200 chars):");
            println!("{}", &content.chars().take(200).collect::<String>());
            if content.len() > 200 {
                println!("...");
            }
        }
        Err(e) => {
            println!("✗ Failed to fetch content: {}", e);
            println!();
            println!("This might happen if:");
            println!("- No WebDriver is available at TARZI_WEBDRIVER_URL");
            println!("- No WebDriver is running at the default port (9515)");
            println!("- ChromeDriver or GeckoDriver is not installed");
            println!("- Network connectivity issues");
            println!();
            println!("To fix this:");
            println!("1. Install ChromeDriver: https://chromedriver.chromium.org/");
            println!("2. Or install GeckoDriver: https://github.com/mozilla/geckodriver/releases");
            println!("3. Or set TARZI_WEBDRIVER_URL to your WebDriver endpoint");
        }
    }

    // Clean up managed driver if any
    if fetcher.has_managed_driver() {
        println!();
        println!("Cleaning up managed driver...");
        match fetcher.cleanup_managed_driver().await {
            Ok(()) => println!("✓ Managed driver cleaned up successfully"),
            Err(e) => println!("⚠ Warning: Failed to clean up managed driver: {}", e),
        }
    }

    println!();
    println!("=== Demo Complete ===");
    Ok(())
}
