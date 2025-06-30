tarzi - Rust-native lite search for AI applications
====================================================

.. image:: https://img.shields.io/crates/v/tarzi.svg
   :target: https://crates.io/crates/tarzi
   :alt: Crate Version

.. image:: https://img.shields.io/pypi/v/tarzi.svg
   :target: https://pypi.org/project/tarzi/
   :alt: PyPI Version

.. image:: https://img.shields.io/badge/License-Apache%202.0-blue.svg
   :target: https://www.apache.org/licenses/LICENSE-2.0
   :alt: License

.. image:: https://img.shields.io/github/actions/workflow/status/mirasurf/tarzi.rs/rust-ci.yml?branch=main
   :target: https://github.com/mirasurf/tarzi.rs/actions
   :alt: Build Status

**tarzi** is a powerful, Rust-native search library designed specifically for AI applications. 
It provides a comprehensive toolkit for content conversion, web fetching, and search engine integration 
with both browser-based and API-based approaches.

.. toctree::
   :maxdepth: 2
   :caption: Contents:

   overview
   installation
   quickstart
   user_guide/index
   python_api/index
   rust_api/index
   examples/index
   configuration

Key Features
============

🔧 **Dual Implementation**
   Native Rust library with Python bindings and CLI tools

🔄 **Content Conversion**
   Convert raw HTML to Markdown, JSON, or YAML formats

🌐 **Web Fetching**
   Fetch web pages with optional JavaScript rendering support

🔍 **Search Integration**
   Query search engines using browser mode (headless/headed/existing) or API mode

🎯 **Multiple Search Engines**
   Support for Bing, Google, DuckDuckGo, Brave Search, Tavily, and custom engines

🔒 **Proxy Support**
   Use proxies in both browser-based and API-based operations

⚡ **End-to-End Pipeline**
   Complete workflow from search queries to content extraction for AI applications

Quick Start
===========

Python
------

.. code-block:: bash

   pip install tarzi

.. code-block:: python

   import tarzi

   # Convert HTML to Markdown
   markdown = tarzi.convert_html("<h1>Hello</h1>", "markdown")

   # Fetch web page
   content = tarzi.fetch_url("https://example.com", js=True)

   # Search web
   results = tarzi.search_web("python programming", "browser", 10)

Rust
----

.. code-block:: bash

   cargo add tarzi

.. code-block:: rust

   use tarzi::{Converter, WebFetcher, SearchEngine, Format, FetchMode, SearchMode};

   #[tokio::main]
   async fn main() -> Result<(), Box<dyn std::error::Error>> {
       // Convert HTML to Markdown
       let converter = Converter::new();
       let markdown = converter.convert("<h1>Hello</h1>", Format::Markdown).await?;

       // Fetch web page
       let mut fetcher = WebFetcher::new();
       let content = fetcher.fetch(
           "https://example.com",
           FetchMode::BrowserHeadless,
           Format::Markdown
       ).await?;

       // Search web
       let mut search_engine = SearchEngine::new();
       let results = search_engine.search(
           "rust programming",
           SearchMode::WebQuery,
           5
       ).await?;

       Ok(())
   }

CLI
---

.. code-block:: bash

   # Install the CLI tool
   cargo install tarzi

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

Architecture
============

tarzi is built with a modular architecture consisting of three core components:

**Converter Module**
   Converts raw HTML content into structured formats (Markdown, JSON, YAML)

**Fetcher Module**
   Handles web page retrieval with multiple strategies (HTTP, Browser automation)

**Search Module**
   Provides search engine integration and result processing

This modular design ensures:

- **Maintainability**: Clear module boundaries make code easier to maintain
- **Reusability**: Fetcher interfaces are reused by search module
- **Flexibility**: Multiple fetch modes and formats supported
- **Extensibility**: Easy to add new formats or fetch modes

Use Cases
=========

🤖 **AI Data Collection**
   Gather and process web content for training data or knowledge bases

📊 **Research Automation**
   Automate web research workflows for academic or business intelligence

🔍 **Content Aggregation**
   Build content aggregation systems that convert web pages to structured data

🕷️ **Web Scraping Pipelines**
   Create robust web scraping pipelines with built-in retry logic and format conversion

🔄 **API Development**
   Use as a backend service for search and content extraction APIs

Support
=======

- **Documentation**: https://tarzi.readthedocs.io/
- **Source Code**: https://github.com/mirasurf/tarzi.rs
- **Issues**: https://github.com/mirasurf/tarzi.rs/issues
- **PyPI**: https://pypi.org/project/tarzi/
- **Crates.io**: https://crates.io/crates/tarzi

License
=======

This project is licensed under the Apache License 2.0 - see the `LICENSE <https://github.com/mirasurf/tarzi.rs/blob/main/LICENSE>`_ file for details.

Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search` 