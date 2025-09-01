#!/usr/bin/env python3
"""
Search engines example for the tarzi Python library.
Demonstrates different search engine configurations and modes.
"""

import tarzi


def main():
    print("=== Tarzi Python Search Engines Example ===\n")

    # 1. Basic search with different engines
    print("1. Basic search with different engines:")

    search_terms = ["artificial intelligence", "machine learning", "data science"]

    for term in search_terms:
        try:
            print(f"\nSearching for: '{term}'")
            results = tarzi.search_web(term, 3)
            print(f"Found {len(results)} results:")
            for i, result in enumerate(results):
                print(f"  {i+1}. {result.title}")
                print(f"     URL: {result.url}")
                print(f"     Rank: {result.rank}")
                print(f"     Snippet: {result.snippet[:100]}...")
        except Exception as e:
            print(f"Search failed for '{term}': {e}")

    # 2. Search with different modes
    print("\n2. Search functionality:")

    try:
        # Web search
        print("\nWeb search:")
        results = tarzi.search_web("Python web scraping", 2)
        print(f"Found {len(results)} results")
        for i, result in enumerate(results):
            print(f"  {i+1}. {result.title}")

        # Search and fetch functionality
        print("\nSearch and fetch functionality:")

        # Search and fetch with plain request mode
        print("\nSearch and fetch with plain request mode:")
        results_with_content = tarzi.search_with_content("web development", 2, "plain_request", "markdown")
        print(f"Fetched content for {len(results_with_content)} results")
        for i, (result, content) in enumerate(results_with_content):
            print(f"  {i+1}. {result.title}")
            print(f"     Content length: {len(content)} characters")

        # Search and fetch with browser mode
        print("\nSearch and fetch with browser mode:")
        results_with_content = tarzi.search_with_content("JavaScript frameworks", 2, "browser_headless", "json")
        print(f"Fetched content for {len(results_with_content)} results")
        for i, (result, content) in enumerate(results_with_content):
            print(f"  {i+1}. {result.title}")
            print(f"     Content length: {len(content)} characters")

    except Exception as e:
        print(f"Search functionality failed: {e}")

    # 3. Search engine configuration
    print("\n3. Search engine configuration:")

    try:
        # Create a search engine with custom configuration
        config = tarzi.Config()
        config.search.engine = "bing"
        config.search.limit = 2
        config.fetcher.user_agent = "Custom Tarzi Bot/1.0"

        search_engine = tarzi.SearchEngine.from_config(config)
        results = search_engine.search("blockchain technology", 2)
        print(f"Custom config search found {len(results)} results")

        # Note: API search is no longer supported
        # The search engine now only supports web search

    except Exception as e:
        print(f"Configuration test failed: {e}")

    # 4. Error handling examples
    print("\n4. Error handling examples:")

    try:
        # Test with invalid fetch mode
        results_with_content = tarzi.search_with_content("test query", 5, "invalid_fetch_mode", "html")
        print("Unexpected: search_with_content with invalid fetch mode succeeded")
    except Exception as e:
        print(f"Expected error for invalid fetch mode: {e}")

    # Test invalid format
    try:
        content = tarzi.fetch_url("https://example.com", "plain_request", "invalid_format")
        print("Unexpected: fetch with invalid format succeeded")
    except Exception as e:
        print(f"Expected error for invalid format: {e}")

    print("\n=== Search Engines Example completed successfully! ===")


if __name__ == "__main__":
    main()
