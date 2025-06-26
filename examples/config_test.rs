use tarzi::config::Config;

fn main() -> tarzi::Result<()> {
    println!("=== Tarzi Configuration Test ===\n");

    // Test loading default config
    println!("1. Loading default configuration:");
    let config = Config::new();
    println!("   General log level: {}", config.general.log_level);
    println!("   General timeout: {}", config.general.timeout);
    println!("   Fetcher mode: {}", config.fetcher.mode);
    println!("   Fetcher format: {}", config.fetcher.format);
    println!("   Fetcher user agent: {}", config.fetcher.user_agent);
    println!("   Search mode: {}", config.search.mode);
    println!("   Search engine: {}", config.search.engine);
    println!("   Result limit: {}", config.search.limit);
    println!();

    // Test loading dev config
    println!("2. Loading development configuration:");
    match Config::load_dev() {
        Ok(config) => {
            println!("   Successfully loaded dev config");
            println!("   Search engine: {}", config.search.engine);
            println!("   Result limit: {}", config.search.limit);
        }
        Err(e) => {
            println!("   Failed to load dev config: {}", e);
        }
    }
    println!();

    // Test saving dev config
    println!("3. Saving development configuration:");
    let mut config = Config::new();
    config.search.limit = 5;
    config.search.engine = "google.com".to_string();
    config.fetcher.mode = "browser_head".to_string();

    match config.save_dev() {
        Ok(_) => println!("   Successfully saved dev config"),
        Err(e) => println!("   Failed to save dev config: {}", e),
    }
    println!();

    // Test loading the saved config
    println!("4. Loading saved development configuration:");
    match Config::load_dev() {
        Ok(config) => {
            println!("   Successfully loaded saved dev config");
            println!("   Search engine: {}", config.search.engine);
            println!("   Result limit: {}", config.search.limit);
            println!("   Fetcher mode: {}", config.fetcher.mode);
        }
        Err(e) => {
            println!("   Failed to load saved dev config: {}", e);
        }
    }

    println!("\n=== Configuration test completed ===");
    Ok(())
}
