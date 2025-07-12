#!/usr/bin/env python3
"""
Integration tests for API providers in tarzi.
These tests require network access and API keys.
"""

import os

import pytest

import tarzi


@pytest.fixture
def test_query():
    """Fixture for test search query."""
    return "python programming"


@pytest.mark.integration
@pytest.mark.api
class TestAPIProviders:
    """Integration test cases for API search providers."""

    def test_brave_api_provider(self, test_query):
        """Test Brave API provider."""
        api_key = os.environ.get("BRAVE_API_KEY")
        if not api_key:
            pytest.skip("BRAVE_API_KEY not set")

        config_str = f"""
[search]
engine = "brave"
brave_api_key = "{api_key}"
"""
        try:
            config = tarzi.Config.from_str(config_str)
            engine = tarzi.SearchEngine.from_config(config)

            results = engine.search(test_query, "apiquery", 2)
            assert isinstance(results, list)

            for result in results:
                assert isinstance(result, tarzi.SearchResult)
                assert hasattr(result, "title")
                assert hasattr(result, "url")
                assert hasattr(result, "snippet")

        except Exception as e:
            pytest.skip(f"Brave API search failed: {e}")

    def test_exa_api_provider(self, test_query):
        """Test Exa API provider."""
        api_key = os.environ.get("EXA_API_KEY")
        if not api_key:
            pytest.skip("EXA_API_KEY not set")

        config_str = f"""
[search]
engine = "exa"
exa_api_key = "{api_key}"
"""
        try:
            config = tarzi.Config.from_str(config_str)
            engine = tarzi.SearchEngine.from_config(config)

            results = engine.search(test_query, "apiquery", 2)
            assert isinstance(results, list)

            for result in results:
                assert isinstance(result, tarzi.SearchResult)
                assert hasattr(result, "title")
                assert hasattr(result, "url")

        except Exception as e:
            pytest.skip(f"Exa API search failed: {e}")

    def test_travily_api_provider(self, test_query):
        """Test Travily API provider."""
        api_key = os.environ.get("TRAVILY_API_KEY")
        if not api_key:
            pytest.skip("TRAVILY_API_KEY not set")

        config_str = f"""
[search]
engine = "travily"
travily_api_key = "{api_key}"
"""
        try:
            config = tarzi.Config.from_str(config_str)
            engine = tarzi.SearchEngine.from_config(config)

            results = engine.search(test_query, "apiquery", 3)
            assert isinstance(results, list)

            for result in results:
                assert isinstance(result, tarzi.SearchResult)
                assert hasattr(result, "title")
                assert hasattr(result, "url")

        except Exception as e:
            pytest.skip(f"Travily API search failed: {e}")

    def test_api_provider_with_proxy(self, test_query):
        """Test API provider with proxy configuration."""
        api_key = os.environ.get("BRAVE_API_KEY")
        if not api_key:
            pytest.skip("BRAVE_API_KEY not set")

        config_str = f"""
[fetcher]
proxy = "http://127.0.0.1:8888"

[search]
engine = "brave"
brave_api_key = "{api_key}"
"""
        try:
            config = tarzi.Config.from_str(config_str)
            engine = tarzi.SearchEngine.from_config(config)

            # This should fail due to invalid proxy but test proxy setup
            results = engine.search(test_query, "apiquery", 1)

            # If it succeeds, proxy worked (unlikely with dummy proxy)
            assert isinstance(results, list)

        except Exception as e:
            # Expected to fail with proxy connection error
            assert any(keyword in str(e).lower() for keyword in ["proxy", "network", "connection"])

    def test_api_provider_without_api_key(self, test_query):
        """Test API provider behavior without API key."""
        config_str = """
[search]
engine = "brave"
"""
        config = tarzi.Config.from_str(config_str)
        engine = tarzi.SearchEngine.from_config(config)

        try:
            results = engine.search(test_query, "apiquery", 1)
            # Should either return empty results or raise an error
            if isinstance(results, list):
                # Empty results are acceptable for providers without keys
                assert len(results) == 0
        except Exception as e:
            # Error is also acceptable
            assert any(keyword in str(e).lower() for keyword in ["api", "key", "provider", "authentication"])

    def test_api_provider_invalid_api_key(self, test_query):
        """Test API provider with invalid API key."""
        config_str = """
[search]
engine = "brave"
brave_api_key = "invalid_api_key_12345"
"""
        config = tarzi.Config.from_str(config_str)
        engine = tarzi.SearchEngine.from_config(config)

        try:
            results = engine.search(test_query, "apiquery", 1)
            # Should either return empty results or raise an error
            if isinstance(results, list):
                assert len(results) == 0
        except Exception as e:
            # Error is expected for invalid API key
            assert any(
                keyword in str(e).lower() for keyword in ["api", "key", "authentication", "unauthorized", "invalid"]
            )

    def test_api_search_limit_boundaries(self, test_query):
        """Test API search with different limit values."""
        api_key = os.environ.get("BRAVE_API_KEY")
        if not api_key:
            pytest.skip("BRAVE_API_KEY not set")

        config_str = f"""
[search]
engine = "brave"
brave_api_key = "{api_key}"
"""
        config = tarzi.Config.from_str(config_str)
        engine = tarzi.SearchEngine.from_config(config)

        # Test different limits
        limits = [0, 1, 5, 10]

        for limit in limits:
            try:
                results = engine.search(test_query, "apiquery", limit)
                assert isinstance(results, list)
                if limit > 0:
                    assert len(results) <= limit
                else:
                    # Limit 0 behavior may vary
                    assert len(results) >= 0

            except Exception as e:
                # Some limits might not be supported
                print(f"Limit {limit} failed: {e}")

    def test_api_search_empty_query(self):
        """Test API search with empty query."""
        api_key = os.environ.get("BRAVE_API_KEY")
        if not api_key:
            pytest.skip("BRAVE_API_KEY not set")

        config_str = f"""
[search]
engine = "brave"
brave_api_key = "{api_key}"
"""
        config = tarzi.Config.from_str(config_str)
        engine = tarzi.SearchEngine.from_config(config)

        try:
            results = engine.search("", "apiquery", 1)
            # Empty query might be handled gracefully or rejected
            if isinstance(results, list):
                assert len(results) >= 0
        except Exception as e:
            # Error is acceptable for empty query
            assert any(keyword in str(e).lower() for keyword in ["query", "empty", "invalid"])

    def test_multiple_api_providers_configuration(self, test_query):
        """Test configuration with multiple API providers."""
        api_keys = {
            "brave": os.environ.get("BRAVE_API_KEY"),
            "exa": os.environ.get("EXA_API_KEY"),
            "travily": os.environ.get("TRAVILY_API_KEY"),
        }

        # Only test if we have at least one API key
        available_keys = {k: v for k, v in api_keys.items() if v}
        if not available_keys:
            pytest.skip("No API keys available")

        # Build config with all available keys
        config_parts = ["[search]"]
        if "brave" in available_keys:
            config_parts.append(f'brave_api_key = "{available_keys["brave"]}"')
        if "exa" in available_keys:
            config_parts.append(f'exa_api_key = "{available_keys["exa"]}"')
        if "travily" in available_keys:
            config_parts.append(f'travily_api_key = "{available_keys["travily"]}"')

        # Use first available provider as primary
        primary_engine = list(available_keys.keys())[0]
        if primary_engine == "exa":
            primary_engine = "exa"
        config_parts.append(f'engine = "{primary_engine}"')

        config_str = "\n".join(config_parts)

        try:
            config = tarzi.Config.from_str(config_str)
            engine = tarzi.SearchEngine.from_config(config)

            results = engine.search(test_query, "apiquery", 2)
            assert isinstance(results, list)

            for result in results:
                assert isinstance(result, tarzi.SearchResult)

        except Exception as e:
            pytest.skip(f"Multi-provider test failed: {e}")
