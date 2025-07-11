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

The Search module provides comprehensive search engine integration with a unified parser architecture:

**Base Parser Architecture**
   All search engines inherit from base parser traits:
   
   - **BaseSearchParser**: Core trait with name, engine type, and support checking
   - **WebSearchParser**: HTML-based parsing with `parse_html()` method
   - **ApiSearchParser**: JSON-based parsing with `parse_json()` method
   - **UnifiedParser**: Combines web and API parsing capabilities

**Browser-Based Search**
   Scrape search results directly from search engine pages:
   
   - Google, Bing, DuckDuckGo, Brave Search, Baidu support
   - Custom search engine configuration
   - Anti-detection measures

**API-Based Search**
   Direct API integration for supported search engines:
   
   - **Multiple API Providers**: Brave, Google, Exa, Travily, DuckDuckGo (more to come)
   - **Automatic Provider Switching**: Smart fallback when primary provider fails
   - **Proxy Support**: Full proxy support for all API providers
   - **Structured Results**: Consistent result format across all providers

**Parser Factory**
   Factory pattern for creating and managing parsers:
   
   - Mode-aware parser selection (WebQuery vs ApiQuery)
   - Custom parser registration
   - Automatic fallback for unsupported combinations

Key features:

- Multiple search engine support
- Configurable result limits
- Search result ranking
- Snippet extraction
- URL validation and cleaning
- Extensible parser architecture

Getting Started
---------------

Ready to get started? Check out our :doc:`installation` guide and :doc:`quickstart` tutorial 
to begin using tarzi in your projects.

For detailed examples and advanced usage patterns, see our :doc:`examples/index` section. 