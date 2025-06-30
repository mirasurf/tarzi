Quick Start Guide
==================

This guide will get you up and running with tarzi in just a few minutes. 
We'll cover the most common use cases and basic functionality.

Your First tarzi Program
-------------------------

Python
~~~~~~

Let's start with a simple example that demonstrates the core functionality:

.. code-block:: python

   import tarzi

   # 1. Convert HTML to Markdown
   html = "<h1>Hello World</h1><p>This is a <strong>test</strong>.</p>"
   markdown = tarzi.convert_html(html, "markdown")
   print("Converted to Markdown:")
   print(markdown)

   # 2. Fetch a web page
   try:
       content = tarzi.fetch_url(
           "https://httpbin.org/html", 
           mode="plain_request", 
           format="markdown"
       )
       print("\nFetched content:")
       print(content[:200] + "...")
   except Exception as e:
       print(f"Fetch failed: {e}")

   # 3. Search the web
   try:
       results = tarzi.search_web(
           "python web scraping", 
           mode="webquery", 
           limit=3
       )
       print(f"\nFound {len(results)} search results:")
       for i, result in enumerate(results):
           print(f"{i+1}. {result.title}")
           print(f"   URL: {result.url}")
           print(f"   Snippet: {result.snippet[:100]}...")
   except Exception as e:
       print(f"Search failed: {e}")

Save this as `quickstart.py` and run it:

.. code-block:: bash

   python quickstart.py

Rust
~~~~

Here's the equivalent Rust program:

.. code-block:: rust

   use tarzi::{Converter, WebFetcher, SearchEngine, Format, FetchMode, SearchMode};

   #[tokio::main]
   async fn main() -> Result<(), Box<dyn std::error::Error>> {
       // 1. Convert HTML to Markdown
       let converter = Converter::new();
       let html = "<h1>Hello World</h1><p>This is a <strong>test</strong>.</p>";
       let markdown = converter.convert(html, Format::Markdown).await?;
       println!("Converted to Markdown:\n{}", markdown);

       // 2. Fetch a web page
       let mut fetcher = WebFetcher::new();
       match fetcher.fetch(
           "https://httpbin.org/html",
           FetchMode::PlainRequest,
           Format::Markdown
       ).await {
           Ok(content) => {
               println!("\nFetched content:\n{}...", &content[..200.min(content.len())]);
           }
           Err(e) => println!("Fetch failed: {}", e),
       }

       // 3. Search the web
       let mut search_engine = SearchEngine::new();
       match search_engine.search(
           "rust programming",
           SearchMode::WebQuery,
           3
       ).await {
           Ok(results) => {
               println!("\nFound {} search results:", results.len());
               for (i, result) in results.iter().enumerate() {
                   println!("{}. {}", i + 1, result.title);
                   println!("   URL: {}", result.url);
                   println!("   Snippet: {}...", &result.snippet[..100.min(result.snippet.len())]);
               }
           }
           Err(e) => println!("Search failed: {}", e),
       }

       Ok(())
   }

Save this as `src/main.rs` in a new Cargo project and run:

.. code-block:: bash

   cargo run

CLI
~~~

You can also use the command-line interface:

.. code-block:: bash

   # Convert HTML to Markdown
   tarzi convert --input "<h1>Hello</h1>" --format markdown

   # Fetch a web page
   tarzi fetch --url "https://httpbin.org/html" --format markdown

   # Search the web
   tarzi search --query "rust programming" --limit 3

Core Concepts
-------------

Formats
~~~~~~~

tarzi supports multiple output formats:

- **Markdown**: Clean, readable text format
- **JSON**: Structured data with metadata
- **YAML**: Human-readable structured format
- **HTML**: Raw HTML (useful for debugging)

.. code-block:: python

   # Try different formats
   html = "<h1>Title</h1><p>Content with <a href='#'>link</a>.</p>"
   
   markdown = tarzi.convert_html(html, "markdown")
   json_data = tarzi.convert_html(html, "json")
   yaml_data = tarzi.convert_html(html, "yaml")
   
   print("Markdown:", markdown)
   print("JSON:", json_data)
   print("YAML:", yaml_data)

Fetch Modes
~~~~~~~~~~~

Different modes for fetching web content:

- **plain_request**: Fast HTTP GET request (no JavaScript)
- **browser_headless**: Full browser automation (supports JavaScript)
- **browser_head**: Browser automation with visible window (for debugging)

.. code-block:: python

   # Static content (fast)
   content = tarzi.fetch_url(
       "https://example.com", 
       mode="plain_request"
   )

   # JavaScript-heavy sites (slower but more complete)
   content = tarzi.fetch_url(
       "https://spa-example.com", 
       mode="browser_headless"
   )

Search Modes
~~~~~~~~~~~~

Two approaches to web search:

- **webquery**: Scrape search engine results pages (no API key needed)
- **apiquery**: Use official search APIs (requires API key)

.. code-block:: python

   # Browser-based search (no API key needed)
   results = tarzi.search_web(
       "machine learning", 
       mode="webquery", 
       limit=10
   )

   # API-based search (requires API key configuration)
   results = tarzi.search_web(
       "artificial intelligence", 
       mode="apiquery", 
       limit=10
   )

Class-Based API
---------------

For more advanced usage, use the class-based API:

Python
~~~~~~

.. code-block:: python

   import tarzi

   # Create instances for reuse
   converter = tarzi.Converter()
   fetcher = tarzi.WebFetcher()
   search_engine = tarzi.SearchEngine()

   # Configure with API key (if available)
   # search_engine = search_engine.with_api_key("your-api-key")

   # Batch conversion
   html_documents = [
       "<h1>Doc 1</h1><p>Content 1</p>",
       "<h1>Doc 2</h1><p>Content 2</p>",
   ]

   converted_docs = []
   for html in html_documents:
       markdown = converter.convert(html, "markdown")
       converted_docs.append(markdown)

   # Search and fetch pipeline
   query = "web scraping best practices"
   search_results = search_engine.search(query, "webquery", 5)

   enriched_results = []
   for result in search_results:
       try:
           content = fetcher.fetch(result.url, "plain_request", "markdown")
           enriched_results.append({
               "title": result.title,
               "url": result.url,
               "snippet": result.snippet,
               "full_content": content
           })
       except Exception as e:
           print(f"Failed to fetch {result.url}: {e}")

Rust
~~~~

.. code-block:: rust

   use tarzi::{Converter, WebFetcher, SearchEngine, Format, FetchMode, SearchMode};

   #[tokio::main]
   async fn main() -> Result<(), Box<dyn std::error::Error>> {
       // Create instances for reuse
       let converter = Converter::new();
       let mut fetcher = WebFetcher::new();
       let mut search_engine = SearchEngine::new();

       // Batch conversion
       let html_documents = vec![
           "<h1>Doc 1</h1><p>Content 1</p>",
           "<h1>Doc 2</h1><p>Content 2</p>",
       ];

       let mut converted_docs = Vec::new();
       for html in html_documents {
           let markdown = converter.convert(html, Format::Markdown).await?;
           converted_docs.push(markdown);
       }

       // Search and fetch pipeline
       let search_results = search_engine.search(
           "rust async programming",
           SearchMode::WebQuery,
           5
       ).await?;

       let mut enriched_results = Vec::new();
       for result in search_results {
           match fetcher.fetch(&result.url, FetchMode::PlainRequest, Format::Markdown).await {
               Ok(content) => {
                   enriched_results.push((result, content));
               }
               Err(e) => {
                   eprintln!("Failed to fetch {}: {}", result.url, e);
               }
           }
       }

       println!("Processed {} results", enriched_results.len());
       Ok(())
   }

Configuration
-------------

For advanced usage, create a configuration file:

.. code-block:: toml

   # tarzi.toml
   [general]
   log_level = "info"
   timeout = 30

   [fetcher]
   mode = "browser_headless"
   format = "markdown"
   user_agent = "tarzi/0.0.4"
   
   [search]
   engine = "bing"
   mode = "webquery"
   limit = 10

Load and use the configuration:

.. code-block:: python

   # Load from file
   config = tarzi.Config.from_file("tarzi.toml")
   
   # Or create from string
   config_str = """
   [fetcher]
   timeout = 60
   format = "json"
   """
   config = tarzi.Config.from_str(config_str)

   # Use with components
   fetcher = tarzi.WebFetcher.from_config(config)
   search_engine = tarzi.SearchEngine.from_config(config)

Error Handling
--------------

tarzi provides comprehensive error handling:

.. code-block:: python

   import tarzi

   try:
       # This might fail due to network issues
       content = tarzi.fetch_url("https://invalid-url.example", mode="plain_request")
   except tarzi.TarziError as e:
       print(f"tarzi error: {e}")
   except Exception as e:
       print(f"Unexpected error: {e}")

   try:
       # This might fail due to invalid HTML
       result = tarzi.convert_html("<<invalid html>>", "markdown")
   except Exception as e:
       print(f"Conversion error: {e}")

Common Patterns
---------------

Web Research Pipeline
~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

   import tarzi

   def research_topic(query, num_results=10):
       """Research a topic and return structured data."""
       
       # Search for relevant content
       results = tarzi.search_web(query, "webquery", num_results)
       
       # Fetch and process each result
       research_data = []
       for result in results:
           try:
               # Fetch content in JSON format for structured data
               content = tarzi.fetch_url(result.url, "plain_request", "json")
               
               research_data.append({
                   "query": query,
                   "title": result.title,
                   "url": result.url,
                   "snippet": result.snippet,
                   "rank": result.rank,
                   "content": content,
                   "timestamp": "2024-01-01T00:00:00Z"  # Add actual timestamp
               })
           except Exception as e:
               print(f"Failed to process {result.url}: {e}")
       
       return research_data

   # Use the pipeline
   data = research_topic("sustainable energy technologies", 5)
   print(f"Collected {len(data)} research items")

Content Aggregation
~~~~~~~~~~~~~~~~~~~

.. code-block:: python

   import tarzi

   def aggregate_news(topics, articles_per_topic=5):
       """Aggregate news articles on multiple topics."""
       
       all_articles = []
       
       for topic in topics:
           print(f"Searching for: {topic}")
           
           # Search for news articles
           results = tarzi.search_web(f"{topic} news", "webquery", articles_per_topic)
           
           for result in results:
               try:
                   # Convert to markdown for readability
                   content = tarzi.fetch_url(result.url, "plain_request", "markdown")
                   
                   all_articles.append({
                       "topic": topic,
                       "title": result.title,
                       "url": result.url,
                       "content": content
                   })
               except Exception as e:
                   print(f"Skipping {result.url}: {e}")
       
       return all_articles

   # Aggregate news on multiple topics
   topics = ["artificial intelligence", "climate change", "space exploration"]
   articles = aggregate_news(topics, 3)
   print(f"Aggregated {len(articles)} articles")

Next Steps
----------

Now that you understand the basics, explore these advanced topics:

- :doc:`user_guide/index` - Comprehensive usage guide
- :doc:`configuration` - Advanced configuration options
- :doc:`examples/index` - Real-world examples and use cases
- :doc:`python_api/index` - Complete Python API reference
- :doc:`rust_api/index` - Complete Rust API reference

Ready to build something amazing? Check out our :doc:`examples/index` for 
inspiration and practical implementations. 