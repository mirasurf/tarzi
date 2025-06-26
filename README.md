<div align="center">
  <img src="tarzi-320.png" alt="Tarzi Logo" width="200" height="200">
</div>

<h1 align="center">tarzi.rs</h1>

<div align="center">
  Rust-native lite search for your AI applications.
</div>

<p align="center">
  <!-- Rust crate: version and download count -->
  <a href="https://crates.io/crates/tarzi">
    <img src="https://img.shields.io/crates/v/tarzi.svg?style=flat-square" alt="Crate Version" />
  </a>
  <a href="https://crates.io/crates/tarzi">
    <img src="https://img.shields.io/crates/d/tarzi.svg?style=flat-square" alt="Crate Downloads" />
  </a>
  <!-- PyPI package: version and monthly downloads -->
  <a href="https://pypi.org/project/tarzi/">
    <img src="https://img.shields.io/pypi/v/tarzi.svg?style=flat-square" alt="PyPI Version" />
  </a>
  <a href="https://pypistats.org/packages/tarzi">
    <img src="https://img.shields.io/pypi/dm/tarzi.svg?style=flat-square" alt="PyPI Downloads" />
  </a>
  <!-- License -->
  <a href="https://www.apache.org/licenses/LICENSE-2.0">
    <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg?style=flat-square" alt="License" />
  </a>
  <!-- X (formerly Twitter) -->
  <a href="https://x.com/mirasurf_ai">
    <img src="https://img.shields.io/twitter/follow/mirasurf_ai?label=@mirasurf_ai&style=flat-square" alt="X Follow" />
  </a>
  <!-- DeepWiki badge -->
  <a href="https://deepwiki.com/mirasurf/tarzi.rs">
    <img src="https://devin.ai/assets/deepwiki-badge.png" style="height: 18px; vertical-align: middle;" alt="Ask DeepWiki" />
  </a>
</p>


## Architecture Overview

The codebase has been restructured into three main modules with clear separation of concerns:

### 1. Converter Module (`src/converter.rs`)
- **Purpose**: Converts raw web page content to various formats
- **Input**: Raw HTML string + target format
- **Output**: Formatted content in the specified format
- **Supported Formats**: HTML, Markdown, JSON, YAML

### 2. Fetcher Module (`src/fetcher.rs`)
- **Purpose**: Fetches web page content from URLs
- **Input**: URL + fetch mode + target format
- **Output**: Formatted content in the specified format
- **Three Fetch Modes**:
  - `plain_request`: Simple HTTP request (no JS rendering)
  - `browser_head`: Browser with UI (JS rendering)
  - `browser_headless`: Headless browser (JS rendering)

### 3. Search Module (`src/search.rs`)
- **Purpose**: Searches for content and fetches results
- **Input**: Search query + search mode + fetch mode + format
- **Output**: Search results with fetched content
- **Reuses**: Fetcher module interfaces without duplication

## Key Improvements

### Modular Design
- **Clear separation**: Each module has a single responsibility
- **No duplication**: Search module reuses fetcher interfaces
- **Format integration**: Fetcher automatically converts to target format
- **Mode flexibility**: Three distinct fetch modes for different use cases

### Enhanced Fetcher
```rust
// New fetcher interface
let mut fetcher = WebFetcher::new();
let content = fetcher.fetch(url, FetchMode::BrowserHeadless, Format::Markdown).await?;
```

### Improved Search
```rust
// Search and fetch content for each result
let mut search_engine = SearchEngine::new();
let results_with_content = search_engine.search_and_fetch(
    query, 
    SearchMode::Browser, 
    limit, 
    FetchMode::PlainRequest, 
    Format::Json
).await?;
```

## Usage Examples

### Basic Fetching
```bash
# Plain HTTP request
cargo run -- fetch --url "https://example.com" --mode plain_request --format markdown

# Browser with JS rendering
cargo run -- fetch --url "https://example.com" --mode browser_headless --format json
```

### Search and Fetch
```bash
# Search for results and fetch content for each
cargo run -- search-and-fetch \
  --query "rust programming" \
  --search-mode browser \
  --fetch-mode plain_request \
  --format markdown \
  --limit 5
```

### Direct Module Usage
```rust
use tarzi::{WebFetcher, FetchMode, SearchEngine, SearchMode, Format};

// Fetch content with conversion
let mut fetcher = WebFetcher::new();
let content = fetcher.fetch("https://example.com", FetchMode::BrowserHeadless, Format::Markdown).await?;

// Search and fetch
let mut search_engine = SearchEngine::new();
let results = search_engine.search_and_fetch(
    "query", 
    SearchMode::Browser, 
    5, 
    FetchMode::PlainRequest, 
    Format::Json
).await?;
```

## Module Dependencies

```
Search Module
    ↓ uses
Fetcher Module
    ↓ uses
Converter Module
```

- **Search** → **Fetcher**: Reuses fetcher interfaces for content retrieval
- **Fetcher** → **Converter**: Automatically converts content to target format
- **No circular dependencies**: Clean, hierarchical structure

## Benefits

1. **Maintainability**: Clear module boundaries make code easier to maintain
2. **Reusability**: Fetcher interfaces are reused by search module
3. **Flexibility**: Multiple fetch modes and formats supported
4. **Simplicity**: No config module dependencies in core functionality
5. **Extensibility**: Easy to add new formats or fetch modes

## Development

### Building
```bash
cargo build
```

### Running Examples
```bash
# Basic usage example
cargo run --example basic_usage

# CLI usage
cargo run -- fetch --url "https://httpbin.org/html" --mode plain_request --format markdown
```

### Testing
```bash
cargo test
```

## License

MIT License - see LICENSE file for details.

## Contributors

Thank you ❤ all human and non-human contributors.

[![tarzi contributors](https://contrib.rocks/image?repo=mirasurf/tarzi "tarzi contributors")](https://github.com/mirasurf/tarzi/graphs/contributors)
