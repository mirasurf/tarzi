use clap::{Parser, Subcommand};
use std::str::FromStr;
use tarzi::constants::{FORMAT_HTML, FORMAT_JSON, FORMAT_MARKDOWN};
use tarzi::{
    Result,
    config::{CliConfigParams, Config},
    converter::{Converter, Format, convert_search_results},
    fetcher::{FetchMode, WebFetcher},
    search::{SearchEngine, SearchMode},
};
use tracing::{debug, info};

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
        #[arg(short, long, default_value = FORMAT_MARKDOWN)]
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
        /// Output format: html, markdown, json, or yaml
        #[arg(short, long, default_value = FORMAT_HTML)]
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
        /// Number of results to return
        #[arg(short, long)]
        limit: Option<usize>,
        /// Output format: json or yaml
        #[arg(short, long, default_value = FORMAT_JSON)]
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
        /// Number of results to return
        #[arg(short, long, default_value = "5")]
        limit: usize,
        /// Output format: html, markdown, json, or yaml
        #[arg(short, long, default_value = FORMAT_MARKDOWN)]
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
    // Initialize logging as early as possible
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let cli = Cli::parse();

    // Load configuration with proper precedence
    let mut config = Config::load_with_precedence()?;

    // Apply CLI parameters to config
    let mut cli_params = CliConfigParams::new();

    match cli.command {
        Commands::Convert {
            input,
            format,
            output,
            verbose: _,
        } => {
            // Convert HTML input to specified format
            debug!("Input length: {} characters", input.len());

            let converter = Converter::new();
            let format = Format::from_str(&format)?;
            let result = converter.convert(&input, format).await?;

            if let Some(output_path) = output {
                std::fs::write(&output_path, result)?;
                info!("Output written to file: {}", output_path);
            } else {
                println!("{result}");
            }
        }
        Commands::Fetch {
            url,
            format,
            output,
            verbose: _,
        } => {
            // Fetch and convert web content
            debug!("Using format: {}", format);

            // Apply CLI parameters to config
            cli_params.fetcher_format = Some(format.clone());
            config.apply_cli_params(&cli_params);

            let mut fetcher = WebFetcher::from_config(&config);
            let format = Format::from_str(&format)?;

            let result = fetcher.fetch(&url, FetchMode::PlainRequest, format).await?;

            if let Some(output_path) = output {
                std::fs::write(&output_path, result)?;
                info!("Output written to file: {}", output_path);
            } else {
                println!("{result}");
            }
        }
        Commands::Search {
            query,
            limit,
            format,
            output,
            verbose: _,
        } => {
            // Perform web search and return results
            // Use CLI limit if provided, otherwise use config limit
            let effective_limit = limit.unwrap_or(config.search.limit);

            // Apply CLI parameters to config
            cli_params.search_limit = Some(effective_limit);
            config.apply_cli_params(&cli_params);

            let mut search_engine = SearchEngine::from_config(&config);
            let mode = SearchMode::from_str(&config.search.mode)?;

            let results = search_engine.search(&query, mode, effective_limit).await?;

            debug!("Processing results for output format: {}", format);

            let format = Format::from_str(&format)?;
            let result = convert_search_results(&results, format)?;

            if let Some(output_path) = output {
                std::fs::write(&output_path, result)?;
                info!("Output written to file: {}", output_path);
            } else {
                println!("{result}");
            }

            // Explicitly clean up browser and driver resources before exit
            search_engine.shutdown().await;
        }
        Commands::SearchAndFetch {
            query,
            limit,
            format,
            output,
            verbose: _,
        } => {
            // Search and fetch content for each result
            // Apply CLI parameters to config
            cli_params.search_limit = Some(limit);
            cli_params.fetcher_format = Some(format.clone());
            config.apply_cli_params(&cli_params);

            let mut search_engine = SearchEngine::from_config(&config);
            let mode = SearchMode::from_str(&config.search.mode)?;
            let format = Format::from_str(&format)?;

            let results_with_content = search_engine
                .search_and_fetch(&query, mode, limit, FetchMode::PlainRequest, format)
                .await?;

            // Convert results to JSON for output
            let result = serde_json::to_string_pretty(&results_with_content)?;

            if let Some(output_path) = output {
                std::fs::write(&output_path, result)?;
                info!("Output written to file: {}", output_path);
            } else {
                println!("{result}");
            }

            // Explicitly clean up browser and driver resources before exit
            search_engine.shutdown().await;
        }
    }

    Ok(())
}
