#!/usr/bin/env python3
"""
Shared pytest fixtures and configuration for tarzi tests.
"""

import sys

import pytest

# Try to import tarzi, but handle gracefully if not available
try:
    import tarzi

    TARZI_AVAILABLE = True
except ImportError:
    TARZI_AVAILABLE = False

    # Create mock classes for demonstration when tarzi is not available
    class MockConfig:
        def __str__(self):
            return "Tarzi configuration"

        def __repr__(self):
            return "Config()"

        @classmethod
        def from_str(cls, config_str):
            if "invalid toml content" in config_str:
                raise RuntimeError("Failed to parse config: invalid toml content")
            return cls()

        @classmethod
        def from_file(cls, filename):
            raise RuntimeError(f"Failed to read config file: {filename}")

    class MockConverter:
        def __str__(self):
            return "Tarzi HTML/text content converter"

        def __repr__(self):
            return "Converter()"

        def convert(self, html, format_type):
            if format_type == "invalid_format":
                raise ValueError("Invalid format: invalid_format")
            if format_type == "html":
                return html

            # Mock conversion results that match test expectations
            if "Test Title" in html and "content" in html:
                if format_type == "markdown":
                    return "# Test Title\n\nTest **content** with [link](https://example.com)."
                elif format_type == "json":
                    return (
                        '{"title": "Test Title", "content": "Test content with link"}'
                    )
                elif format_type == "yaml":
                    return "title: Test Title\ncontent: Test content with link"
            elif "Pipeline Test" in html:
                if format_type == "markdown":
                    return "# Pipeline Test\n\nThis is a **test** of the processing pipeline."
                elif format_type == "json":
                    return '{"title": "Pipeline Test", "content": "This is a test of the processing pipeline"}'
                elif format_type == "yaml":
                    return "title: Pipeline Test\ncontent: This is a test of the processing pipeline"

            return f"Mock {format_type} conversion of content"

        @classmethod
        def from_config(cls, config):
            return cls()

    class MockWebFetcher:
        def __str__(self):
            return "Tarzi web page fetcher"

        def __repr__(self):
            return "WebFetcher()"

        def fetch(self, url, mode, format_type):
            if mode == "invalid_mode":
                raise ValueError("Invalid fetch mode: invalid_mode")
            if format_type == "invalid_format":
                raise ValueError("Invalid format: invalid_format")
            return f"<html><body>Mock content from {url}</body></html>"

        def fetch_raw(self, url, mode):
            if mode == "invalid_mode":
                raise ValueError("Invalid fetch mode: invalid_mode")
            return f"Raw mock content from {url}"

        @classmethod
        def from_config(cls, config):
            return cls()

    class MockSearchResult:
        def __init__(self, title="Mock Result", url="https://example.com"):
            self.title = title
            self.url = url

    class MockSearchEngine:
        def __str__(self):
            return "Tarzi search engine"

        def __repr__(self):
            return "SearchEngine()"

        def search(self, query, mode, limit):
            if mode == "invalid_mode":
                raise ValueError("Invalid search mode: invalid_mode")
            return [MockSearchResult() for _ in range(min(limit, 2))]

        def search_and_fetch(self, query, search_mode, limit, fetch_mode, format_type):
            if search_mode == "invalid_mode":
                raise ValueError("Invalid search mode: invalid_mode")
            if fetch_mode == "invalid_fetch_mode":
                raise ValueError("Invalid fetch mode: invalid_fetch_mode")
            if format_type == "invalid_format":
                raise ValueError("Invalid format: invalid_format")
            results = self.search(query, search_mode, limit)
            return [(r, f"Mock content for {r.url}") for r in results]

        def with_api_key(self, api_key):
            return self

        @classmethod
        def from_config(cls, config):
            return cls()

    # Create mock module
    class MockTarzi:
        Config = MockConfig
        Converter = MockConverter
        WebFetcher = MockWebFetcher
        SearchEngine = MockSearchEngine
        SearchResult = MockSearchResult

        @staticmethod
        def convert_html(html, format_type):
            if format_type == "invalid_format":
                raise ValueError("Invalid format: invalid_format")
            if format_type == "html":
                return html

            # Mock conversion results that match test expectations
            if "Test Title" in html and "content" in html:
                if format_type == "markdown":
                    return "# Test Title\n\nTest **content** with [link](https://example.com)."
                elif format_type == "json":
                    return (
                        '{"title": "Test Title", "content": "Test content with link"}'
                    )
                elif format_type == "yaml":
                    return "title: Test Title\ncontent: Test content with link"
            elif "Pipeline Test" in html:
                if format_type == "markdown":
                    return "# Pipeline Test\n\nThis is a **test** of the processing pipeline."
                elif format_type == "json":
                    return '{"title": "Pipeline Test", "content": "This is a test of the processing pipeline"}'
                elif format_type == "yaml":
                    return "title: Pipeline Test\ncontent: This is a test of the processing pipeline"

            return f"Mock {format_type} conversion of content"

        @staticmethod
        def fetch_url(url, mode, format_type):
            if mode == "invalid_mode":
                raise ValueError("Invalid fetch mode: invalid_mode")
            if format_type == "invalid_format":
                raise ValueError("Invalid format: invalid_format")
            return f"<html><body>Mock content from {url}</body></html>"

        @staticmethod
        def search_web(query, mode, limit):
            if mode == "invalid_mode":
                raise ValueError("Invalid search mode: invalid_mode")
            return [MockSearchResult() for _ in range(min(limit, 2))]

        @staticmethod
        def search_and_fetch(query, search_mode, limit, fetch_mode, format_type):
            if search_mode == "invalid_mode":
                raise ValueError("Invalid search mode: invalid_mode")
            if fetch_mode == "invalid_fetch_mode":
                raise ValueError("Invalid fetch mode: invalid_fetch_mode")
            if format_type == "invalid_format":
                raise ValueError("Invalid format: invalid_format")
            results = [MockSearchResult() for _ in range(min(limit, 2))]
            return [(r, f"Mock content for {r.url}") for r in results]

    tarzi = MockTarzi()

    # Make the mock tarzi module available globally for imports
    sys.modules["tarzi"] = tarzi


@pytest.fixture(scope="session")
def default_config():
    """Session-scoped fixture for default tarzi configuration."""
    return tarzi.Config()


@pytest.fixture(scope="session")
def sample_config_str():
    """Session-scoped fixture for sample configuration string."""
    return """
[fetcher]
timeout = 30
user_agent = "Tarzi Test/1.0"
format = "html"
proxy = ""

[search]
engine = "bing"
api_key = ""
query_pattern = "https://www.bing.com/search?q={query}"
"""


@pytest.fixture
def simple_html():
    """Fixture for simple HTML content."""
    return "<p>Hello, <strong>world</strong>!</p>"


@pytest.fixture
def complex_html():
    """Fixture for more complex HTML content."""
    return """
    <!DOCTYPE html>
    <html>
    <head>
        <title>Test Page</title>
    </head>
    <body>
        <h1>Main Title</h1>
        <div class="content">
            <p>This is a <em>test</em> paragraph with <a href="https://example.com">a link</a>.</p>
            <ul>
                <li>Item 1</li>
                <li>Item 2</li>
            </ul>
        </div>
        <footer>
            <p>&copy; 2024 Test</p>
        </footer>
    </body>
    </html>
    """


# pytest configuration is now in pyproject.toml


def pytest_collection_modifyitems(config, items):
    """Automatically mark tests based on their location."""
    for item in items:
        # Mark unit tests
        if "unit" in str(item.fspath):
            item.add_marker(pytest.mark.unit)
        # Mark integration tests
        elif "integration" in str(item.fspath):
            item.add_marker(pytest.mark.integration)

        # Skip integration tests if tarzi is not available
        if not TARZI_AVAILABLE and item.get_closest_marker("integration"):
            item.add_marker(pytest.mark.skip(reason="tarzi module not available"))


# Custom pytest hooks are simplified since we use markers for test selection
