Overview
========

.. note::
   tarzi supports only Linux and macOS. Windows is not supported.

What is tarzi?
--------------

**Tarzi** is a unified search interface designed for **Retrieval-Augmented Generation (RAG)** and **agentic systems** built on large language models. Search is a core functionality in these systems, yet most search engine providers impose API paywalls or strict rate limits—even for light or research-driven usage.

**Tarzi** removes these barriers by supporting both token-based APIs and free web queries across multiple search engines. With a single dependency, you can integrate and switch between different Search Engine Providers (SEPs) as needed—seamlessly and efficiently.

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
       content = tarzi.fetch_url(result.url, "browser_headless", "markdown")
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

Getting Started
---------------

Ready to get started? Check out our :doc:`installation` guide and :doc:`quickstart` tutorial 
to begin using tarzi in your projects.

For detailed examples and advanced usage patterns, see our :doc:`examples/index` section. 