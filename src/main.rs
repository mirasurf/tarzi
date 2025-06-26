use clap::{Parser, Subcommand};
use std::str::FromStr;
use tarsier::{
    Result,
    converter::{Converter, Format, convert_search_results},
    fetcher::WebFetcher,
    search::{SearchEngine, SearchMode},
};
use tracing::{Level, debug, info};

#[derive(Parser)]
#[command(name = "tarsier")]
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
        /// Enable JavaScript rendering
        #[arg(short, long)]
        js: bool,
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

            info!("Tarsier Convert starting with verbose mode: {}", verbose);
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
            js,
            format,
            output,
            verbose,
        } => {
            // Initialize logging for this subcommand
            let log_level = if verbose { Level::DEBUG } else { Level::INFO };
            tracing_subscriber::fmt().with_max_level(log_level).init();

            info!("Tarsier Fetch starting with verbose mode: {}", verbose);
            info!("Fetching URL: {} (JS: {})", url, js);
            debug!("Using format: {}", format);

            let mut fetcher = WebFetcher::new();
            let content = if js {
                info!("Fetching with JavaScript rendering enabled");
                fetcher.fetch_with_js(&url).await?
            } else {
                info!("Fetching with basic HTTP client");
                fetcher.fetch(&url).await?
            };

            info!(
                "Successfully fetched content ({} characters)",
                content.len()
            );

            let converter = Converter::new();
            let format = Format::from_str(&format)?;
            let result = converter.convert(&content, format).await?;

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

            info!("Tarsier Search starting with verbose mode: {}", verbose);
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
    }

    Ok(())
}
