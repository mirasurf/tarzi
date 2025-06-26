# Development Guide

## Building

```bash
cargo build
```

## Running Examples

```bash
# Basic usage example
cargo run --example basic_usage

# CLI usage
cargo run -- fetch --url "https://httpbin.org/html" --mode plain_request --format markdown
```

## Testing

```bash
cargo test
```

## Project Structure

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

## Benefits

1. **Maintainability**: Clear module boundaries make code easier to maintain
2. **Reusability**: Fetcher interfaces are reused by search module
3. **Flexibility**: Multiple fetch modes and formats supported
4. **Simplicity**: No config module dependencies in core functionality
5. **Extensibility**: Easy to add new formats or fetch modes

## Search Engine Support

Tarzi supports multiple search engines with configurable query patterns:

### Supported Search Engines
- **Bing** (default): `https://www.bing.com/search?q={query}`
- **Google**: `https://www.google.com/search?q={query}`
- **DuckDuckGo**: `https://duckduckgo.com/?q={query}`
- **Brave Search**: `https://search.brave.com/search?q={query}`
- **Tavily**: `https://tavily.com/search?q={query}`
- **SearchApi**: `https://www.searchapi.io/search?q={query}`
- **Custom**: User-defined query patterns

### Configuration
Configure search engines in your `tarzi.toml` file:

```toml
[search]
mode = "webquery"
engine = "bing"  # Default search engine
query_pattern = "https://www.bing.com/search?q={query}"  # Custom pattern (optional)
limit = 5
api_key = "your-api-key-for-apiquery-mode"
```

### Custom Query Patterns
You can define custom query patterns for any search engine:

```toml
[search]
engine = "google"
query_pattern = "https://custom-search.com/search?query={query}&lang=en&region=us"
```

The `{query}` placeholder is automatically replaced with the user's search query. 