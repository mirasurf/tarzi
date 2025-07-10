Rust API Reference
===================

Complete reference for the tarzi Rust API.

Quick Reference
---------------

**Core Modules**
   - ``tarzi::converter`` - HTML conversion functionality
   - ``tarzi::fetcher`` - Web page fetching
   - ``tarzi::search`` - Search engine integration
   - ``tarzi::search::parser`` - Search result parsing

**Main Structs**
   - ``Converter`` - HTML conversion
   - ``WebFetcher`` - Web page fetching
   - ``SearchEngine`` - Web search operations
   - ``ParserFactory`` - Parser creation and management

**Base Parser Traits**
   - ``BaseSearchParser`` - Core parser trait
   - ``WebSearchParser`` - HTML-based parsing
   - ``ApiSearchParser`` - JSON-based parsing
   - ``UnifiedParser`` - Combined web and API parsing

**Enums**
   - ``Format`` - Output formats (Markdown, JSON, YAML, HTML)
   - ``FetchMode`` - Fetching strategies
   - ``SearchMode`` - Search strategies
   - ``SearchEngineType`` - Supported search engines

Basic Usage
-----------

.. code-block:: rust

   use tarzi::{Converter, WebFetcher, SearchEngine, Format, FetchMode, SearchMode};
   use tarzi::search::parser::{ParserFactory, SearchEngineType};

   #[tokio::main]
   async fn main() -> Result<(), Box<dyn std::error::Error>> {
       // Convert HTML
       let converter = Converter::new();
       let markdown = converter.convert("<h1>Hello</h1>", Format::Markdown).await?;

       // Fetch web page
       let mut fetcher = WebFetcher::new();
       let content = fetcher.fetch("https://example.com", FetchMode::PlainRequest, Format::Markdown).await?;

       // Search web
       let mut search_engine = SearchEngine::new();
       let results = search_engine.search("agentic AI", SearchMode::WebQuery, 10).await?;

       // Use parser factory
       let factory = ParserFactory::new();
       let parser = factory.get_parser(&SearchEngineType::Google, SearchMode::WebQuery);
       let parsed_results = parser.parse(html_content, 10)?;

       Ok(())
   } 