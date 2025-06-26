use clap::{Parser, Subcommand};
use std::str::FromStr;
use tarsier::{
    converter::{Converter, Format, convert_search_results},
    fetcher::WebFetcher,
    search::{SearchEngine, SearchMode},
    Result,
};
use tracing::{info, Level};

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
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Convert { input, format, output } => {
            info!("Converting input to {}", format);
            let converter = Converter::new();
            let format = Format::from_str(&format)?;
            let result = converter.convert(&input, format).await?;
            
            if let Some(output_path) = output {
                std::fs::write(output_path, result)?;
                info!("Output written to file");
            } else {
                println!("{}", result);
            }
        }
        Commands::Fetch { url, js, format, output } => {
            info!("Fetching URL: {}", url);
            let mut fetcher = WebFetcher::new();
            let content = if js {
                fetcher.fetch_with_js(&url).await?
            } else {
                fetcher.fetch(&url).await?
            };
            
            let converter = Converter::new();
            let format = Format::from_str(&format)?;
            let result = converter.convert(&content, format).await?;
            
            if let Some(output_path) = output {
                std::fs::write(output_path, result)?;
                info!("Output written to file");
            } else {
                println!("{}", result);
            }
        }
        Commands::Search { query, mode, limit, format, output } => {
            info!("Searching for: {}", query);
            let mut search_engine = SearchEngine::new();
            let mode = SearchMode::from_str(&mode)?;
            let results = search_engine.search(&query, mode, limit).await?;
            
            let format = Format::from_str(&format)?;
            let result = convert_search_results(&results, format)?;
            
            if let Some(output_path) = output {
                std::fs::write(output_path, result)?;
                info!("Output written to file");
            } else {
                println!("{}", result);
            }
        }
    }

    Ok(())
} 