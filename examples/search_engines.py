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
            results = tarzi.search_web(term, "webquery", 3)
            print(f"Found {len(results)} results:")
            for i, result in enumerate(results):
                print(f"  {i+1}. {result.title}")
                print(f"     URL: {result.url}")
                print(f"     Rank: {result.rank}")
                print(f"     Snippet: {result.snippet[:100]}...")
        except Exception as e:
            print(f"Search failed for '{term}': {e}")

    # 2. Search with different modes
    print("\n2. Search with different modes:")

    try:
        # Web query mode (browser-based)
        print("\nWeb query mode (browser-based):")
        results = tarzi.search_web("Python web scraping", "webquery", 2)
        print(f"Found {len(results)} results")
        for i, result in enumerate(results):
            print(f"  {i+1}. {result.title}")

        # API query mode (API-based)
        print("\nAPI query mode (API-based):")
        results = tarzi.search_web("Rust async programming", "apiquery", 2)
        print(f"Found {len(results)} results")
        for i, result in enumerate(results):
            print(f"  {i+1}. {result.title}")

    except Exception as e:
        print(f"Search modes failed: {e}")

    # 3. Search and fetch functionality
    print("\n3. Search and fetch functionality:")

    try:
        # Search and fetch with plain request mode
        print("\nSearch and fetch with plain request mode:")
        results_with_content = tarzi.search_and_fetch("web development", "webquery", 2, "plain_request", "markdown")
        print(f"Found {len(results_with_content)} results with content:")
        for i, (result, content) in enumerate(results_with_content):
            print(f"  {i+1}. {result.title}")
            print(f"     Content length: {len(content)} characters")
            print(f"     Content preview: {content[:150]}...")

        # Search and fetch with browser headless mode
        print("\nSearch and fetch with browser headless mode:")
        results_with_content = tarzi.search_and_fetch(
            "JavaScript frameworks", "webquery", 2, "browser_headless", "json"
        )
        print(f"Found {len(results_with_content)} results with content:")
        for i, (result, content) in enumerate(results_with_content):
            print(f"  {i+1}. {result.title}")
            print(f"     Content length: {len(content)} characters")
            print(f"     Content preview: {content[:150]}...")

    except Exception as e:
        print(f"Search and fetch failed: {e}")

    # 4. Using SearchEngine class with different configurations
    print("\n4. Using SearchEngine class with different configurations:")

    try:
        # Basic search engine
        search_engine = tarzi.SearchEngine()
        results = search_engine.search("blockchain technology", "webquery", 2)
        print(f"Basic search engine found {len(results)} results")

        # Search engine with API key (commented out as it requires actual API key)
        # Note: API keys are now configured per provider in the configuration
        # api_search_engine = tarzi.SearchEngine()
        # api_results = api_search_engine.search("cryptocurrency", "apiquery", 2)
        # print(f"API search engine found {len(api_results)} results")

        # Search with proxy (commented out as it requires actual proxy)
        # proxy_results = search_engine.search_with_proxy(
        #     "privacy tools", "webquery", 2, "http://proxy.example.com:8080"
        # )
        # print(f"Proxy search found {len(proxy_results)} results")

    except Exception as e:
        print(f"SearchEngine class usage failed: {e}")

    # 5. Configuration-based search engines
    print("\n5. Configuration-based search engines:")

    try:
        # Create different configurations
        configs = {
            "Bing": """
[fetcher]
timeout = 30
user_agent = "Tarzi Bing Search/1.0"
format = "html"
proxy = ""

[search]
engine = "bing"
# API keys are now configured per provider in the configuration
# For API-based search, configure the specific provider's API key
""",
            "DuckDuckGo": """
[fetcher]
timeout = 30
user_agent = "Tarzi DuckDuckGo Search/1.0"
format = "markdown"
proxy = ""

[search]
engine = "duckduckgo"
# API keys are now configured per provider in the configuration
# For API-based search, configure the specific provider's API key
""",
            "Google": """
[fetcher]
timeout = 30
user_agent = "Tarzi Google Search/1.0"
format = "json"
proxy = ""

[search]
engine = "google"
# API keys are now configured per provider in the configuration
# For API-based search, configure the specific provider's API key
""",
        }

        for engine_name, config_str in configs.items():
            print(f"\nTesting {engine_name} configuration:")
            try:
                config = tarzi.Config.from_str(config_str)
                search_engine = tarzi.SearchEngine.from_config(config)
                results = search_engine.search("open source software", "webquery", 1)
                print(f"  Found {len(results)} results")
                if results:
                    print(f"  First result: {results[0].title}")
            except Exception as e:
                print(f"  {engine_name} configuration failed: {e}")

    except Exception as e:
        print(f"Configuration-based search failed: {e}")

    # 6. Error handling examples
    print("\n6. Error handling examples:")

    # Test invalid search mode
    try:
        results = tarzi.search_web("test query", "invalid_mode", 5)
        print("Unexpected: search with invalid mode succeeded")
    except Exception as e:
        print(f"Expected error for invalid search mode: {e}")

    # Test invalid fetch mode
    try:
        results = tarzi.search_and_fetch("test query", "webquery", 5, "invalid_fetch_mode", "html")
        print("Unexpected: search_and_fetch with invalid fetch mode succeeded")
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
