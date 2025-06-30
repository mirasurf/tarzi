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
  <a href="https://crates.io/crates/tarzi">
    <img src="https://img.shields.io/badge/AI%20Assisted-Yes-green?style=for-the-badge" style="height: 18px; vertical-align: middle;" alt="AI Assisted Yes" />
  </a>
</p>

## Features

### Core Capabilities
- **Dual Implementation**: Native Rust library and Python wrapper with CLI tools
- **Content Conversion**: Convert raw HTML to Markdown, JSON, or YAML formats
- **Web Fetching**: Fetch web pages with optional JavaScript rendering support
- **Search Integration**: Query search engines using browser mode (headless/headed/existing. NO API KEY) or API mode
- **Multiple Search Engines**: Support for Bing, Google, DuckDuckGo, Brave Search, Tavily, and custom engines
- **Proxy Support**: Use proxies in both browser-based and API-based operations
- **End-to-End Pipeline**: Complete workflow from search queries to content extraction for AI applications

### Advanced Features (coming soon)
- **Customizable Browser Controls**: Control screen resolution, viewport, user-agent, and locale to mimic real users.
- **Automatic CAPTCHA Handling**: Detect and bypass CAPTCHAs using solvers or manual fallback.
- **Intelligent Query Formulation**: Enhance search queries with prompt rewriting or intent-aware generation.
- **Stealth & Anti-Bot Evasion**: Use fingerprint spoofing, proxy rotation, and human-like interaction patterns.
- **Workflow-Oriented Task Automation**: Chain multiple actions like search, click, form fill, and scrape into workflows.
- **Multi-Channel Processing (MCP) Integration**: Integrate with agent frameworks for context-aware, distributed task execution.
- **Search Metrics & Observability**: Track success rate, latency, CAPTCHA rate, and export logs to observability tools.

## Architecture

Tarzi is built with a modular architecture consisting of three core components:

* Converter Module: Converts raw HTML content into structured formats
* Fetcher Module: Handles web page retrieval with multiple strategies
* Search Module: Provides search engine integration and result processing

## Usage Examples

### Basic Content Conversion
```rust
use tarzi::{Converter, Format};

let converter = Converter::new();
let html = "<h1>Hello World</h1><p>This is content.</p>";
let markdown = converter.convert(html, Format::Markdown).await?;
```

### Web Page Fetching
```rust
use tarzi::{WebFetcher, FetchMode, Format};

let mut fetcher = WebFetcher::new();

// Simple HTTP request
let content = fetcher.fetch(
    "https://example.com", 
    FetchMode::PlainRequest, 
    Format::Markdown
).await?;

// With JavaScript rendering
let content = fetcher.fetch(
    "https://example.com", 
    FetchMode::BrowserHeadless, 
    Format::Json
).await?;
```

### Search and Content Extraction
```rust
use tarzi::{SearchEngine, SearchMode, FetchMode, Format};

let mut search_engine = SearchEngine::new();

// Search and fetch content for each result
let results_with_content = search_engine.search_and_fetch(
    "rust programming", 
    SearchMode::WebQuery, 
    5, 
    FetchMode::PlainRequest, 
    Format::Markdown
).await?;
```

### Python Integration
```python
import tarzi

# Convert HTML to Markdown
markdown = tarzi.convert_html("<h1>Hello</h1>", "markdown")

# Fetch web page
content = tarzi.fetch_url("https://example.com", js=True)

# Search web
results = tarzi.search_web("python programming", "browser", 10)
```

### CLI Usage
```bash
# Convert HTML to Markdown
tarzi convert --input "<h1>Hello</h1>" --format markdown

# Fetch web page with JavaScript rendering
tarzi fetch --url "https://example.com" --mode browser_headless --format json

# Search and fetch content
tarzi search-and-fetch \
  --query "rust programming" \
  --search-mode browser \
  --fetch-mode plain_request \
  --format markdown \
  --limit 5
```

## License

Apache License 2.0 - see LICENSE file for details.

## Contributors

Thank you ❤ all human and non-human contributors.

[![tarzi contributors](https://contrib.rocks/image?repo=mirasurf/tarzi "tarzi contributors")](https://github.com/mirasurf/tarzi/graphs/contributors)
