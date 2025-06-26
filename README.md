# tarsier

<div align="center">
  <img src="tarsier.png" alt="Tarsier Logo" width="200" height="200">
</div>

<div align="center">
  Rust-native lite search for AI applications.
</div>

## Features

### Goals 0 ✅

- [x] Provide both a native Rust implementation and a Python wrapper, available as a library and a CLI tool.  
- [x] Convert raw HTML strings into semi-structured formats such as Markdown, JSON, or YAML.  
- [x] Fetch individual web pages from URLs, with optional JavaScript rendering support.  
- [x] Perform search engine queries using either browser mode (headless or headed, no token required) or API mode (token-based).  
- [x] Support proxy usage in both browser-based and API-based modes.  
- [x] Implement an end-to-end pipeline for querying search engines, parsing SERPs, and extracting target pages for AI applications.

### Goals 1

- [ ] Capture screenshots of web pages.  
- [ ] Define and execute custom workflows for interacting with web pages and crawling structured content.

## Installation

### Rust

```bash
# Clone the repository
git clone https://github.com/mirasurf/tarsier.git
cd tarsier

# Build and install
cargo install --path .

# Or build for development
cargo build --release
```

### Python

```bash
# Install from source (requires Rust toolchain)
pip install maturin
maturin develop

# Or install the Python package
pip install tarsier
```

## Configuration

Tarsier supports configuration via TOML files:

### Default Configuration Locations
- **User config**: `$HOME/.tarsier.toml` (for production use)
- **Development config**: `./tarsier.toml` (in project root)

### Configuration Sections

```toml
[general]
log_level = "info"        # Logging level: debug, info, warn, error
timeout = 30              # General timeout in seconds

[converter]
default_format = "markdown"  # Default output format: markdown, json, yaml

[fetcher]
user_agent = "Mozilla/5.0 (compatible; Tarsier/1.0)"  # Custom user agent
timeout = 30              # HTTP request timeout

[browser]
browser_mode = "headless" # Browser mode: headless, head
timeout = 60              # Browser operation timeout

[search]
search_mode = "browser"   # Search mode: browser, api
search_engine = "bing.com" # Default search engine
result_limit = 3          # Default number of results
# API keys for different search engines (optional)
# google_search_api_key = "your_google_api_key"
# bing_search_api_key = "your_bing_api_key"
# duckduckgo_api_key = "your_duckduckgo_api_key"
```

### Configuration Management

```rust
use tarsier::config::Config;

// Load user configuration
let config = Config::load()?;

// Load development configuration
let dev_config = Config::load_dev()?;

// Save configuration
config.save()?;
config.save_dev()?;
```

## Usage

### CLI Usage

```bash
# Convert HTML to Markdown
tarsier convert -i "<h1>Hello World</h1>" -f markdown

# Fetch a web page and convert to JSON
tarsier fetch -u "https://example.com" -f json

# Search using browser mode
tarsier search -q "Rust programming" -m browser -l 5

# Search using API mode (requires API key)
tarsier search -q "Python programming" -m api -l 3
```

**Verbose Mode**: Use the `-v` or `--verbose` flag with any subcommand to enable detailed logging for that specific operation. This is particularly useful for debugging issues with browser-based search operations that might hang or fail silently.

### Rust Library Usage

```rust
use tarsier::{
    converter::{Converter, Format},
    fetcher::WebFetcher,
    search::{SearchEngine, SearchMode},
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Convert HTML to Markdown
    let converter = Converter::new();
    let html = "<h1>Hello World</h1>";
    let markdown = converter.convert(html, Format::Markdown).await?;
    println!("{}", markdown);

    // Fetch web page
    let fetcher = WebFetcher::new();
    let content = fetcher.fetch("https://example.com").await?;
    
    // Search with browser mode
    let mut search_engine = SearchEngine::new();
    let results = search_engine.search("Rust programming", SearchMode::Browser, 5).await?;
    
    for result in results {
        println!("{}: {}", result.title, result.url);
    }

    Ok(())
}
```

### Python Library Usage

```python
import tarsier

# Convert HTML to Markdown
html = "<h1>Hello World</h1>"
markdown = tarsier.convert_html(html, "markdown")
print(markdown)

# Fetch web page
content = tarsier.fetch_url("https://example.com", js=False)

# Search with browser mode
results = tarsier.search_web("Rust programming", "browser", 5)
for result in results:
    print(f"{result.title}: {result.url}")

# Using class-based API
converter = tarsier.PyConverter()
fetcher = tarsier.PyWebFetcher()
search_engine = tarsier.PySearchEngine()

# Convert to different formats
json_output = converter.convert(html, "json")
yaml_output = converter.convert(html, "yaml")
```

## API Reference

### Converter

The `Converter` class handles HTML to various format conversions:

- **HTML → Markdown**: Uses `html2md` crate
- **HTML → JSON**: Uses `pulldown-cmark` to parse and `serde_json` to serialize
- **HTML → YAML**: Uses `pulldown-cmark` to parse and `serde_yaml` to serialize

### WebFetcher

The `WebFetcher` class handles web page fetching:

- **Basic fetching**: Uses `reqwest` for HTTP requests
- **JavaScript rendering**: Uses `chromiumoxide` for headless browser automation
- **Proxy support**: Configurable proxy settings for both modes

### SearchEngine

The `SearchEngine` class handles search functionality:

- **Browser mode**: Uses `chromiumoxide` to scrape search results from Google
- **API mode**: Supports token-based search APIs (configurable)
- **Proxy support**: Works with both browser and API modes

## Dependencies

* Rust edition 2024
* chromiumoxide: support chrome browser instance
* HTML → Markdown: html2md
* Markdown → JSON: use pulldown-cmark to produce JSON
* JSON → YAML: serde_yaml
* mistral.rs: run local LLM
* pyo3: Python bindings
* reqwest: HTTP client
* tokio: async runtime

## Examples

See the `examples/` directory for complete usage examples:

- `examples/basic_usage.rs` - Rust library usage
- `examples/basic_usage.py` - Python library usage

## Development

```bash
# Run tests
cargo test

# Run examples
cargo run --example basic_usage

# Build Python bindings
maturin develop

# Run Python example
python examples/basic_usage.py
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Contributors

Thank you all contributors ❤

[![tarsier contributors](https://contrib.rocks/image?repo=mirasurf/tarsier "tarsier contributors")](https://github.com/mirasurf/tarsier/graphs/contributors)
