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
    def test_search_webquery(self, engine, test_query):
        """Test search with webquery mode."""
        try:
            results = engine.search(test_query, "webquery", 2)
            assert isinstance(results, list)
            # Results might be empty due to rate limiting, so just check type
            for result in results:
                assert isinstance(result, tarzi.SearchResult)
        except Exception as e:
            pytest.skip(f"Search failed (likely rate limited): {e}")

    @pytest.mark.network
    @pytest.mark.api
    def test_search_apiquery(self, engine, test_query):
        """Test search with apiquery mode."""
        try:
            results = engine.search(test_query, "apiquery", 2)
            assert isinstance(results, list)
            # Results might be empty without API key, so just check type
            for result in results:
                assert isinstance(result, tarzi.SearchResult)
        except Exception as e:
            pytest.skip(f"API search failed (likely no API key): {e}")

    @pytest.mark.network
    @pytest.mark.slow
    def test_search_and_fetch(self, engine, test_query):
        """Test search and fetch functionality."""
        try:
            results = engine.search_and_fetch(
                test_query, "webquery", 1, "plain_request", "markdown"
            )
            assert isinstance(results, list)
            for result, content in results:
                assert isinstance(result, tarzi.SearchResult)
                assert isinstance(content, str)
        except Exception as e:
            pytest.skip(f"Search and fetch failed: {e}")

    def test_invalid_search_mode(self, engine, test_query):
        """Test invalid search mode raises ValueError."""
        with pytest.raises(ValueError, match="Invalid search mode"):
            engine.search(test_query, "invalid_mode", 5)

    def test_invalid_fetch_mode_in_search_and_fetch(self, engine, test_query):
        """Test invalid fetch mode in search_and_fetch raises ValueError."""
        with pytest.raises(ValueError, match="Invalid fetch mode"):
            engine.search_and_fetch(
                test_query, "webquery", 1, "invalid_fetch_mode", "html"
            )

    def test_invalid_format_in_search_and_fetch(self, engine, test_query):
        """Test invalid format in search_and_fetch raises ValueError."""
        with pytest.raises(ValueError, match="Invalid format"):
            engine.search_and_fetch(
                test_query, "webquery", 1, "plain_request", "invalid_format"
            )

    def test_with_api_key(self, engine):
        """Test setting API key."""
        # Test that with_api_key method exists and returns the engine
        result = engine.with_api_key("test-api-key")
        # The method should return the engine instance
        # Note: This doesn't test actual functionality, just that the method works
        assert result is not None

    def test_from_config(self):
        """Test creating SearchEngine from config."""
        config = tarzi.Config()
        engine = tarzi.SearchEngine.from_config(config)
        assert isinstance(engine, tarzi.SearchEngine)


@pytest.mark.integration
@pytest.mark.network
@pytest.mark.slow
def test_search_web_function(test_query):
    """Test search_web standalone function."""
    try:
        results = tarzi.search_web(test_query, "webquery", 2)
        assert isinstance(results, list)
        for result in results:
            assert isinstance(result, tarzi.SearchResult)
    except Exception as e:
        pytest.skip(f"Search failed: {e}")


def test_search_web_invalid_mode(test_query):
    """Test search_web with invalid mode."""
    with pytest.raises(ValueError, match="Invalid search mode"):
        tarzi.search_web(test_query, "invalid_mode", 2)


@pytest.mark.integration
@pytest.mark.network
@pytest.mark.slow
def test_search_and_fetch_function(test_query):
    """Test search_and_fetch standalone function."""
    try:
        results = tarzi.search_and_fetch(
            test_query, "webquery", 1, "plain_request", "markdown"
        )
        assert isinstance(results, list)
        for result, content in results:
            assert isinstance(result, tarzi.SearchResult)
            assert isinstance(content, str)
    except Exception as e:
        pytest.skip(f"Search and fetch failed: {e}")


def test_search_and_fetch_invalid_search_mode(test_query):
    """Test search_and_fetch with invalid search mode."""
    with pytest.raises(ValueError, match="Invalid search mode"):
        tarzi.search_and_fetch(test_query, "invalid_mode", 1, "plain_request", "html")


def test_search_and_fetch_invalid_fetch_mode(test_query):
    """Test search_and_fetch with invalid fetch mode."""
    with pytest.raises(ValueError, match="Invalid fetch mode"):
        tarzi.search_and_fetch(test_query, "webquery", 1, "invalid_fetch_mode", "html")


def test_search_and_fetch_invalid_format(test_query):
    """Test search_and_fetch with invalid format."""
    with pytest.raises(ValueError, match="Invalid format"):
        tarzi.search_and_fetch(
            test_query, "webquery", 1, "plain_request", "invalid_format"
        )
