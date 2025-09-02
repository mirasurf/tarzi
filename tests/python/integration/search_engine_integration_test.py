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

    def test_search_with_content(self, engine, test_query):
        """Test search and fetch functionality."""
        try:
            results = engine.search_with_content(test_query, 1, "plain_request", "markdown")
            assert len(results) > 0, "Should return at least one result with content"
            for result, content in results:
                assert result.title, "Result should have a title"
                assert result.url, "Result should have a URL"
                # Content might be empty for some results, that's okay
                assert isinstance(content, str), "Content should be a string"
        except Exception as e:
            pytest.fail(f"Search and fetch failed: {e}")

    def test_search_with_content_invalid_fetch_mode(self, engine, test_query):
        """Test search and fetch with invalid fetch mode."""
        with pytest.raises(Exception):
            engine.search_with_content(test_query, 1, "invalid_fetch_mode", "html")

    def test_search_with_content_invalid_format(self, engine, test_query):
        """Test search and fetch with invalid format."""
        with pytest.raises(Exception):
            engine.search_with_content(test_query, 1, "plain_request", "invalid_format")

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
@pytest.mark.network
@pytest.mark.slow
def test_search_web_basic_functionality(test_query):
    """Test search_web basic functionality."""
    try:
        results = tarzi.search_web(test_query, 1)
        assert len(results) > 0, "Should return at least one result"
        for result in results:
            assert result.title, "Result should have a title"
            assert result.url, "Result should have a URL"
    except Exception as e:
        pytest.fail(f"search_web basic functionality failed: {e}")


@pytest.mark.integration
@pytest.mark.network
@pytest.mark.slow
def test_search_with_content_function(test_query):
    """Test the search_with_content function."""
    try:
        results = tarzi.search_with_content(test_query, 1, "plain_request", "markdown")
        assert len(results) > 0, "Should return at least one result with content"
    except Exception as e:
        pytest.fail(f"search_with_content function failed: {e}")


@pytest.mark.integration
def test_search_with_content_invalid_fetch_mode(test_query):
    """Test search_with_content with invalid fetch mode."""
    with pytest.raises(Exception):
        tarzi.search_with_content(test_query, 1, "invalid_fetch_mode", "html")


@pytest.mark.integration
def test_search_with_content_invalid_format(test_query):
    """Test search_with_content with invalid format."""
    with pytest.raises(Exception):
        tarzi.search_with_content(test_query, 1, "plain_request", "invalid_format")
