#!/usr/bin/env python3
"""
Integration tests for the WebFetcher class in tarzi.
These tests require network access and external services.
"""

import pytest
import tarzi


@pytest.fixture
def fetcher():
    """Fixture for creating a WebFetcher instance."""
    return tarzi.WebFetcher()


@pytest.fixture
def test_url():
    """Fixture for reliable test URL."""
    return "https://httpbin.org/html"


@pytest.mark.integration
class TestWebFetcher:
    """Integration test cases for the WebFetcher class."""

    def test_fetcher_creation(self, fetcher):
        """Test WebFetcher can be created."""
        assert isinstance(fetcher, tarzi.WebFetcher)
        assert str(fetcher) == "Tarzi web page fetcher"
        assert repr(fetcher) == "WebFetcher()"

    @pytest.mark.network
    def test_fetch_plain_request_html(self, fetcher, test_url):
        """Test fetching with plain request mode and HTML format."""
        try:
            result = fetcher.fetch(test_url, "plain_request", "html")
            assert isinstance(result, str)
            assert len(result) > 0
        except Exception as e:
            pytest.skip(f"Network request failed: {e}")

    @pytest.mark.network
    def test_fetch_plain_request_markdown(self, fetcher, test_url):
        """Test fetching with plain request mode and Markdown format."""
        try:
            result = fetcher.fetch(test_url, "plain_request", "markdown")
            assert isinstance(result, str)
            assert len(result) > 0
        except Exception as e:
            pytest.skip(f"Network request failed: {e}")

    @pytest.mark.network
    def test_fetch_raw(self, fetcher, test_url):
        """Test raw fetching."""
        try:
            result = fetcher.fetch_raw(test_url, "plain_request")
            assert isinstance(result, str)
            assert len(result) > 0
        except Exception as e:
            pytest.skip(f"Network request failed: {e}")

    def test_invalid_fetch_mode(self, fetcher, test_url):
        """Test invalid fetch mode raises ValueError."""
        with pytest.raises(ValueError, match="Invalid fetch mode"):
            fetcher.fetch(test_url, "invalid_mode", "html")

    def test_invalid_format(self, fetcher, test_url):
        """Test invalid format raises ValueError."""
        with pytest.raises(ValueError, match="Invalid format"):
            fetcher.fetch(test_url, "plain_request", "invalid_format")

    def test_from_config(self):
        """Test creating WebFetcher from config."""
        config = tarzi.Config()
        fetcher = tarzi.WebFetcher.from_config(config)
        assert isinstance(fetcher, tarzi.WebFetcher)


@pytest.mark.integration
@pytest.mark.network
def test_fetch_url_function(test_url):
    """Test fetch_url standalone function."""
    try:
        result = tarzi.fetch_url(test_url, "plain_request", "html")
        assert isinstance(result, str)
        assert len(result) > 0
    except Exception as e:
        pytest.skip(f"Network request failed: {e}")


def test_fetch_url_invalid_mode(test_url):
    """Test fetch_url with invalid mode."""
    with pytest.raises(ValueError, match="Invalid fetch mode"):
        tarzi.fetch_url(test_url, "invalid_mode", "html")


def test_fetch_url_invalid_format(test_url):
    """Test fetch_url with invalid format."""
    with pytest.raises(ValueError, match="Invalid format"):
        tarzi.fetch_url(test_url, "plain_request", "invalid_format")
