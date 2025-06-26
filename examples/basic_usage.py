#!/usr/bin/env python3
"""
Basic usage example for the tarsier Python library.
"""

import tarsier

def main():
    print("=== Tarsier Python Basic Usage Example ===\n")

    # 1. HTML to Markdown conversion
    print("1. Converting HTML to Markdown:")
    html_input = """
        <html>
            <head><title>Example Page</title></head>
            <body>
                <h1>Welcome to Tarsier</h1>
                <p>This is a <strong>test</strong> page with <a href="https://example.com">a link</a>.</p>
                <img src="image.jpg" alt="Test image">
            </body>
        </html>
    """
    
    markdown = tarsier.convert_html(html_input, "markdown")
    print(f"Markdown output:\n{markdown}\n")

    # 2. HTML to JSON conversion
    print("2. Converting HTML to JSON:")
    json_output = tarsier.convert_html(html_input, "json")
    print(f"JSON output:\n{json_output}\n")

    # 3. Web page fetching (without JavaScript)
    print("3. Fetching web page (without JavaScript):")
    try:
        content = tarsier.fetch_url("https://httpbin.org/html", js=False)
        print(f"Successfully fetched page ({len(content)} characters)")
        markdown = tarsier.convert_html(content, "markdown")
        print(f"Converted to markdown (first 200 chars):\n{markdown[:200]}...\n")
    except Exception as e:
        print(f"Failed to fetch page: {e}\n")

    # 4. Search functionality (browser mode)
    print("4. Search functionality (browser mode):")
    try:
        results = tarsier.search_web("Rust programming", "browser", 3)
        print(f"Found {len(results)} search results:")
        for i, result in enumerate(results):
            print(f"  {i+1}. {result.title} ({result.url})")
            print(f"     {result.snippet}")
    except Exception as e:
        print(f"Search failed: {e}\n")

    # 5. Using the class-based API
    print("5. Using class-based API:")
    
    # Converter
    converter = tarsier.PyConverter()
    yaml_output = converter.convert(html_input, "yaml")
    print(f"YAML output:\n{yaml_output}\n")
    
    # WebFetcher
    fetcher = tarsier.PyWebFetcher()
    try:
        content = fetcher.fetch("https://httpbin.org/html")
        print(f"Fetched content length: {len(content)}")
    except Exception as e:
        print(f"Fetch failed: {e}")
    
    # SearchEngine
    search_engine = tarsier.PySearchEngine()
    try:
        results = search_engine.search("Python programming", "api", 2)
        print(f"Found {len(results)} API search results:")
        for i, result in enumerate(results):
            print(f"  {i+1}. {result.title} ({result.url})")
            print(f"     {result.snippet}")
    except Exception as e:
        print(f"API search failed: {e}")

    print("\n=== Example completed successfully! ===")

if __name__ == "__main__":
    main() 