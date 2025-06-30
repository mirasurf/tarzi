Overview
========

What is tarzi?
--------------

tarzi is a powerful, Rust-native search library designed specifically for AI applications. 
It provides a comprehensive toolkit for content conversion, web fetching, and search engine 
integration with both browser-based and API-based approaches.

Core Philosophy
---------------

tarzi is built around three core principles:

**Performance First**
   Written in Rust for maximum performance and memory safety, with zero-cost abstractions

**AI-Native Design**
   Purpose-built for AI applications with structured data output and batch processing capabilities

**Developer Experience**
   Simple APIs with comprehensive error handling and extensive documentation

Key Components
--------------

Converter Module
~~~~~~~~~~~~~~~~

The Converter module is responsible for transforming raw HTML content into various structured formats:

- **HTML to Markdown**: Clean, readable text format perfect for AI training data
- **HTML to JSON**: Structured data with metadata (title, links, images, content)
- **HTML to YAML**: Human-readable structured format for configuration and data storage

Key features:

- Intelligent content extraction
- Metadata preservation
- Customizable output formatting
- Memory-efficient processing

Fetcher Module
~~~~~~~~~~~~~~

The Fetcher module handles web page retrieval with multiple strategies:

**HTTP Mode**
   Fast, lightweight HTTP requests for static content

**Browser Automation**
   Full browser automation for JavaScript-heavy sites:
   
   - Headless mode for server environments
   - Headed mode for debugging
   - External browser connection for custom setups

**Proxy Support**
   Built-in proxy support for all fetch modes

Key features:

- Multiple fetch strategies
- Automatic retry logic
- Custom user agent support
- Timeout configuration
- Cookie and session management

Search Module
~~~~~~~~~~~~~

The Search module provides comprehensive search engine integration:

**Browser-Based Search**
   Scrape search results directly from search engine pages:
   
   - Google, Bing, DuckDuckGo, Brave Search support
   - Custom search engine configuration
   - Anti-detection measures

**API-Based Search**
   Direct API integration for supported search engines:
   
   - Official API support
   - Rate limiting and quota management
   - Structured result parsing

Key features:

- Multiple search engine support
- Configurable result limits
- Search result ranking
- Snippet extraction
- URL validation and cleaning

Architecture Benefits
---------------------

**Modularity**
   Each component can be used independently or combined for complex workflows

**Performance**
   Rust's zero-cost abstractions ensure maximum performance

**Reliability**
   Comprehensive error handling and graceful degradation

**Extensibility**
   Plugin architecture for custom search engines and formats

**Cross-Platform**
   Works on Linux, macOS, and Windows

Use Cases
---------

AI Training Data Collection
~~~~~~~~~~~~~~~~~~~~~~~~~~~

Collect and process web content for AI model training:

.. code-block:: python

   import tarzi

   # Search for relevant content
   results = tarzi.search_web("machine learning tutorials", "browser", 50)
   
   # Fetch and convert each result
   training_data = []
   for result in results:
       content = tarzi.fetch_url(result.url, mode="browser_headless", format="markdown")
       training_data.append({
           "title": result.title,
           "url": result.url,
           "content": content
       })

Web Research Automation
~~~~~~~~~~~~~~~~~~~~~~~

Automate research workflows for business intelligence:

.. code-block:: rust

   use tarzi::{SearchEngine, WebFetcher, SearchMode, FetchMode, Format};

   let mut search_engine = SearchEngine::new();
   let mut fetcher = WebFetcher::new();

   // Search for industry reports
   let results = search_engine.search(
       "industry report 2024 market analysis",
       SearchMode::WebQuery,
       20
   ).await?;

   // Fetch and analyze each report
   for result in results {
       let content = fetcher.fetch(
           &result.url,
           FetchMode::BrowserHeadless,
           Format::Json
       ).await?;
       
       // Process structured content
       analyze_report(content).await?;
   }

Content Aggregation Systems
~~~~~~~~~~~~~~~~~~~~~~~~~~~

Build content aggregation systems with automatic format conversion:

.. code-block:: python

   import tarzi

   # Configure for news aggregation
   config = tarzi.Config.from_str("""
   [fetcher]
   timeout = 30
   format = "json"
   
   [search]
   engine = "bing"
   limit = 100
   """)

   search_engine = tarzi.SearchEngine.from_config(config)
   
   # Aggregate news articles
   articles = search_engine.search_and_fetch(
       "technology news today",
       "browser",
       50,
       "plain_request",
       "json"
   )

Performance Characteristics
---------------------------

**Memory Efficiency**
   Streaming processing for large documents with minimal memory footprint

**Speed**
   Rust's performance with optimized HTML parsing and network handling

**Concurrency**
   Built-in async/await support for handling multiple requests concurrently

**Scalability**
   Designed to handle thousands of requests per minute

Comparison with Alternatives
----------------------------

vs. Traditional Web Scraping Libraries
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

========================= =============== ======================
Feature                   tarzi           Traditional Libraries
========================= =============== ======================
Performance               Rust-native     Python/Node.js overhead
Memory Usage              Minimal         High memory allocation
Error Handling            Comprehensive   Basic error reporting
AI Integration            Built-in        Manual integration
Search Engine Support     Multiple        Limited or none
Format Conversion         Built-in        Requires external tools
Async Support             Native          Varies by library
========================= =============== ======================

vs. Browser Automation Tools
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

========================= =============== ======================
Feature                   tarzi           Browser Automation
========================= =============== ======================
Setup Complexity          Simple          Complex configuration
Resource Usage            Optimized       High CPU/memory usage
Anti-Detection            Built-in        Requires custom setup
Content Processing        Integrated      Manual processing
Multi-Format Output       Yes             No
API Alternative           Available       Browser-only
========================= =============== ======================

Getting Started
---------------

Ready to get started? Check out our :doc:`installation` guide and :doc:`quickstart` tutorial 
to begin using tarzi in your projects.

For detailed examples and advanced usage patterns, see our :doc:`examples/index` section. 