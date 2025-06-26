use clap::{Parser, Subcommand};
use std::str::FromStr;
use tarzi::{
    Result,
    converter::{Converter, Format, convert_search_results},
    fetcher::{WebFetcher, FetchMode},
    search::{SearchEngine, SearchMode},
};
use tracing::{Level, debug, info};

#[derive(Parser)]
#[command(name = "tarzi")]
#[command(about = "Rust-native lite search for AI applications")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert HTML to various formats
    Convert {
        /// Input HTML string or file path
        #[arg(short, long)]
        input: String,
        /// Output format: markdown, json, or yaml
        #[arg(short, long, default_value = "markdown")]
        format: String,
        /// Output file path (optional)
        #[arg(short, long)]
        output: Option<String>,
        /// Enable verbose logging
        #[arg(short, long)]
        verbose: bool,
    },
    /// Fetch web page content
    Fetch {
        /// URL to fetch
        #[arg(short, long)]
        url: String,
        /// Fetch mode: plain_request, browser_head, or browser_headless
        #[arg(short, long, default_value = "plain_request")]
        mode: String,
        /// Output format: html, markdown, json, or yaml
        #[arg(short, long, default_value = "html")]
        format: String,
        /// Output file path (optional)
        #[arg(short, long)]
        output: Option<String>,
        /// Enable verbose logging
        #[arg(short, long)]
        verbose: bool,
    },
    /// Search using search engines
    Search {
        /// Search query
        #[arg(short, long)]
        query: String,
        /// Search mode: browser or api
        #[arg(short, long, default_value = "browser")]
        mode: String,
        /// Number of results to return
        #[arg(short, long, default_value = "10")]
        limit: usize,
        /// Output format: json or yaml
        #[arg(short, long, default_value = "json")]
        format: String,
        /// Output file path (optional)
        #[arg(short, long)]
        output: Option<String>,
        /// Enable verbose logging
        #[arg(short, long)]
        verbose: bool,
    },
    /// Search and fetch content for each result
    SearchAndFetch {
        /// Search query
        #[arg(short, long)]
        query: String,
        /// Search mode: browser or api
        #[arg(short, long, default_value = "browser")]
        search_mode: String,
        /// Fetch mode: plain_request, browser_head, or browser_headless
        #[arg(short, long, default_value = "plain_request")]
        fetch_mode: String,
        /// Number of results to return
        #[arg(short, long, default_value = "5")]
        limit: usize,
        /// Output format: html, markdown, json, or yaml
        #[arg(short, long, default_value = "markdown")]
        format: String,
        /// Output file path (optional)
        #[arg(short, long)]
        output: Option<String>,
        /// Enable verbose logging
        #[arg(short, long)]
        verbose: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Convert {
            input,
            format,
            output,
            verbose,
        } => {
            // Initialize logging for this subcommand
            let log_level = if verbose { Level::DEBUG } else { Level::INFO };
            tracing_subscriber::fmt().with_max_level(log_level).init();

            info!("Tarzi Convert starting with verbose mode: {}", verbose);
            info!("Converting input to {}", format);
            debug!("Input length: {} characters", input.len());

            let converter = Converter::new();
            let format = Format::from_str(&format)?;
            let result = converter.convert(&input, format).await?;

            if let Some(output_path) = output {
                std::fs::write(&output_path, result)?;
                info!("Output written to file: {}", output_path);
            } else {
                println!("{}", result);
            }
        }
        Commands::Fetch {
            url,
            mode,
            format,
            output,
            verbose,
        } => {
            // Initialize logging for this subcommand
            let log_level = if verbose { Level::DEBUG } else { Level::INFO };
            tracing_subscriber::fmt().with_max_level(log_level).init();

            info!("Tarzi Fetch starting with verbose mode: {}", verbose);
            info!("Fetching URL: {} with mode: {}", url, mode);
            debug!("Using format: {}", format);

            let mut fetcher = WebFetcher::new();
            let fetch_mode = FetchMode::from_str(&mode)?;
            let format = Format::from_str(&format)?;
            
            let result = fetcher.fetch(&url, fetch_mode, format).await?;

            info!(
                "Successfully fetched and converted content ({} characters)",
                result.len()
            );

            if let Some(output_path) = output {
                std::fs::write(&output_path, result)?;
                info!("Output written to file: {}", output_path);
            } else {
                println!("{}", result);
            }
        }
        Commands::Search {
            query,
            mode,
            limit,
            format,
            output,
            verbose,
        } => {
            // Initialize logging for this subcommand
            let log_level = if verbose { Level::DEBUG } else { Level::INFO };
            tracing_subscriber::fmt().with_max_level(log_level).init();

            info!("Tarzi Search starting with verbose mode: {}", verbose);
            info!("Starting search operation");
            info!("Query: '{}'", query);
            info!("Mode: {}", mode);
            info!("Limit: {}", limit);
            info!("Format: {}", format);

            let mut search_engine = SearchEngine::new();
            let mode = SearchMode::from_str(&mode)?;

            info!("Search engine initialized, starting search...");
            let results = search_engine.search(&query, mode, limit).await?;

            info!("Search completed, found {} results", results.len());
            debug!("Processing results for output format: {}", format);

            let format = Format::from_str(&format)?;
            let result = convert_search_results(&results, format)?;

            if let Some(output_path) = output {
                std::fs::write(&output_path, result)?;
                info!("Output written to file: {}", output_path);
            } else {
                println!("{}", result);
            }
        }
        Commands::SearchAndFetch {
            query,
            search_mode,
            fetch_mode,
            limit,
            format,
            output,
            verbose,
        } => {
            // Initialize logging for this subcommand
            let log_level = if verbose { Level::DEBUG } else { Level::INFO };
            tracing_subscriber::fmt().with_max_level(log_level).init();

            info!("Tarzi SearchAndFetch starting with verbose mode: {}", verbose);
            info!("Starting search and fetch operation");
            info!("Query: '{}'", query);
            info!("Search mode: {}", search_mode);
            info!("Fetch mode: {}", fetch_mode);
            info!("Limit: {}", limit);
            info!("Format: {}", format);

            let mut search_engine = SearchEngine::new();
            let search_mode = SearchMode::from_str(&search_mode)?;
            let fetch_mode = FetchMode::from_str(&fetch_mode)?;
            let format = Format::from_str(&format)?;

            info!("Search engine initialized, starting search and fetch...");
            let results_with_content = search_engine.search_and_fetch(
                &query, 
                search_mode, 
                limit, 
                fetch_mode, 
                format
            ).await?;

            info!("Search and fetch completed, processed {} results", results_with_content.len());

            // Convert results to JSON for output
            let result = serde_json::to_string_pretty(&results_with_content)?;

            if let Some(output_path) = output {
                std::fs::write(&output_path, result)?;
                info!("Output written to file: {}", output_path);
            } else {
                println!("{}", result);
            }
        }
    }

    Ok(())
}
