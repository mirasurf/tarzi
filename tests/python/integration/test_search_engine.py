#!/usr/bin/env python3
"""
Integration tests for the SearchEngine class in tarzi.
These tests require network access and may require API keys.
"""

import pytest

import tarzi


@pytest.fixture
def engine():
    """Fixture for creating a SearchEngine instance."""
    return tarzi.SearchEngine()


@pytest.fixture
def test_query():
    """Fixture for test search query."""
    return "python programming"


@pytest.mark.integration
class TestSearchEngine:
    """Integration test cases for the SearchEngine class."""

    def test_engine_creation(self, engine):
        """Test SearchEngine can be created."""
        assert isinstance(engine, tarzi.SearchEngine)
        assert str(engine) == "Tarzi search engine"
        assert repr(engine) == "SearchEngine()"

    @pytest.mark.network
    @pytest.mark.slow
    def test_search_api(self, engine, test_query):
        """Test search with API mode (no longer supported, should skip)."""
        pytest.skip("API search mode no longer supported")

    def test_search_and_fetch(self, engine, test_query):
        """Test search and fetch functionality."""
        try:
            results = engine.search_with_content(test_query, 1, "plain_request", "markdown")
            assert len(results) > 0, "Should return at least one result with content"
            for result, content in results:
                assert result.title, "Result should have a title"
                assert result.url, "Result should have a URL"
                assert content, "Should have fetched content"
        except Exception as e:
            pytest.fail(f"Search and fetch failed: {e}")

    def test_search_and_fetch_invalid_fetch_mode(self, engine, test_query):
        """Test search and fetch with invalid fetch mode."""
        with pytest.raises(Exception):
            engine.search_with_content(test_query, 1, "invalid_fetch_mode", "html")

    def test_search_and_fetch_invalid_format(self, engine, test_query):
        """Test search and fetch with invalid format."""
        with pytest.raises(Exception):
            engine.search_with_content(test_query, 1, "plain_request", "invalid_format")

    def test_with_api_key(self, engine):
        """Test setting API key."""
        # The with_api_key method is not implemented in the current version
        # This test is skipped as it's not a core functionality requirement
        pytest.skip("with_api_key method not implemented in current version")

    def test_from_config(self):
        """Test creating SearchEngine from config."""
        config = tarzi.Config()
        engine = tarzi.SearchEngine.from_config(config)
        assert isinstance(engine, tarzi.SearchEngine)


@pytest.mark.integration
@pytest.mark.network
@pytest.mark.slow
def test_search_web_function(test_query):
    """Test the search_web function."""
    try:
        results = tarzi.search_web(test_query, 2)
        assert len(results) > 0, "Should return at least one result"
    except Exception as e:
        pytest.fail(f"search_web function failed: {e}")


@pytest.mark.integration
def test_search_web_invalid_mode(test_query):
    """Test search_web with invalid mode."""
    with pytest.raises(ValueError, match="Invalid search mode"):
        tarzi.search_web(test_query, "invalid_mode", 2)


@pytest.mark.integration
@pytest.mark.network
@pytest.mark.slow
def test_search_and_fetch_function(test_query):
    """Test the search_and_fetch function."""
    try:
        results = tarzi.search_with_content(test_query, 1, "plain_request", "markdown")
        assert len(results) > 0, "Should return at least one result with content"
    except Exception as e:
        pytest.fail(f"search_with_content function failed: {e}")


@pytest.mark.integration
def test_search_and_fetch_invalid_fetch_mode(test_query):
    """Test search_and_fetch with invalid fetch mode."""
    with pytest.raises(Exception):
        tarzi.search_with_content(test_query, 1, "invalid_fetch_mode", "html")


@pytest.mark.integration
def test_search_and_fetch_invalid_format(test_query):
    """Test search_and_fetch with invalid format."""
    with pytest.raises(Exception):
        tarzi.search_with_content(test_query, 1, "plain_request", "invalid_format")
