#!/usr/bin/env python3
"""
Basic usage example for the tarzi Python library.
"""

import tarzi


def main():
    print("=== Tarzi Python Basic Usage Example ===\n")

    # 1. HTML to Markdown conversion
    print("1. Converting HTML to Markdown:")
    html_input = """
        <html>
            <head><title>Example Page</title></head>
            <body>
                <h1>Welcome to Tarzi</h1>
                <p>This is a <strong>test</strong> page with <a href="https://example.com">a link</a>.</p>
                <img src="image.jpg" alt="Test image">
            </body>
        </html>
    """

    markdown = tarzi.convert_html(html_input, "markdown")
    print(f"Markdown output:\n{markdown}\n")

    # 2. HTML to JSON conversion
    print("2. Converting HTML to JSON:")
    json_output = tarzi.convert_html(html_input, "json")
    print(f"JSON output:\n{json_output}\n")

    # 3. Web page fetching with different modes
    print("3. Fetching web page with different modes:")
    try:
        # Plain request mode
        content = tarzi.fetch_url("https://httpbin.org/html", "plain_request", "html")
        print(f"Plain request mode - Successfully fetched page ({len(content)} characters)")

        # Convert to markdown
        markdown = tarzi.convert_html(content, "markdown")
        print(f"Converted to markdown (first 200 chars):\n{markdown[:200]}...\n")
    except Exception as e:
        print(f"Failed to fetch page: {e}\n")

    # 4. Search functionality with different modes
    print("4. Search functionality:")
    try:
        # Basic search
        results = tarzi.search_web("Agentic AI", 3)
        print(f"Found {len(results)} results for 'Agentic AI'")
        for i, result in enumerate(results):
            print(f"{i+1}. {result.title}")
            print(f"   URL: {result.url}")
            print(f"   Snippet: {result.snippet[:100]}...")

        # Search with different limit
        results = tarzi.search_web("Agent LLM", 2)
        print(f"\nFound {len(results)} results for 'Agent LLM'")
        for i, result in enumerate(results):
            print(f"{i+1}. {result.title}")

        # Search and fetch content
        results_with_content = tarzi.search_with_content("web scraping", 2, "plain_request", "markdown")
        print(f"\nFetched content for {len(results_with_content)} results")
        for i, (result, content) in enumerate(results_with_content):
            print(f"{i+1}. {result.title}")
            print(f"   Content length: {len(content)} characters")
            print(f"   First 100 chars: {content[:100]}...")

    except Exception as e:
        print(f"Search failed: {e}\n")

    # 5. Search and fetch functionality
    print("5. Search and fetch functionality:")
    try:
        results_with_content = tarzi.search_and_fetch("web scraping", "webquery", 2, "plain_request", "markdown")
        print(f"Found {len(results_with_content)} results with content:")
        for i, (result, content) in enumerate(results_with_content):
            print(f"  {i+1}. {result.title} ({result.url})")
            print(f"     Content preview: {content[:100]}...")
        print()
    except Exception as e:
        print(f"Search and fetch failed: {e}\n")

    # 6. Using the class-based API
    print("6. Using class-based API:")

    # Converter
    converter = tarzi.Converter()
    yaml_output = converter.convert(html_input, "yaml")
    print(f"YAML output:\n{yaml_output}\n")

    # WebFetcher with different modes
    fetcher = tarzi.WebFetcher()
    try:
        # Plain request mode
        content = fetcher.fetch("https://httpbin.org/html", "plain_request", "html")
        print(f"Plain request - Fetched content length: {len(content)}")

        # Raw fetch mode
        raw_content = fetcher.fetch_url("https://httpbin.org/html", "plain_request")
        print(f"Raw fetch - Content length: {len(raw_content)}")
    except Exception as e:
        print(f"Fetch failed: {e}")

    # SearchEngine
    search_engine = tarzi.SearchEngine()
    try:
        # Note: API keys are now configured per provider in the configuration
        # See the configuration example below for how to set up API keys

        results = search_engine.search("machine learning", 2)
        print(f"Found {len(results)} search results:")
        for i, result in enumerate(results):
            print(f"  {i+1}. {result.title} ({result.url})")
            print(f"     {result.snippet}")
    except Exception as e:
        print(f"Search failed: {e}")

    # 7. Configuration-based usage
    print("\n7. Configuration-based usage:")
    try:
        # Create config from string
        config_str = """
[fetcher]
timeout = 30
user_agent = "Tarzi Python Example/1.0"
format = "markdown"
proxy = ""

[search]
engine = "bing"
api_key = ""
query_pattern = "https://www.bing.com/search?q={query}"
"""
        config = tarzi.Config.from_str(config_str)
        print("Created config from string successfully")

        # Use config with fetcher
        tarzi.WebFetcher.from_config(config)
        print("Created fetcher from config successfully")

        # Use config with search engine
        tarzi.SearchEngine.from_config(config)
        print("Created search engine from config successfully")

    except Exception as e:
        print(f"Configuration usage failed: {e}")

    print("\n=== Example completed successfully! ===")


if __name__ == "__main__":
    main()
