Overview
========

.. note::
   tarzi supports only Linux and macOS. Windows is not supported.

What is tarzi?
--------------

**Tarzi** is a unified search interface designed for **Retrieval-Augmented Generation (RAG)** and **agentic systems** built on large language models. Search is a core functionality in these systems, yet most search engine providers (SEPs) impose API paywalls or strict rate limits. **Tarzi**, empowered by browser automation and web crawling technologies, removes these barriers by supporting token-free queries across multiple search engines. With a single dependency, you can integrate and switch between different SEPs as needed—seamlessly and efficiently.

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

**Proxy Support**
   Custom proxy support for all fetch modes

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
   - Anti-detection measures (under development)

**API-Based Search** (under development)
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

Getting Started
---------------

Ready to get started? Check out our :doc:`installation` guide and :doc:`quickstart` tutorial 
to begin using tarzi in your projects.

For detailed examples and advanced usage patterns, see our :doc:`examples/index` section. 