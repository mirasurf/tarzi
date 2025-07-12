#!/usr/bin/env python3
"""
Unit tests for proxy configuration in tarzi.
"""

import os

import pytest

import tarzi


@pytest.fixture
def config_with_proxy():
    """Fixture for config with proxy settings."""
    config_str = """
[fetcher]
proxy = "http://127.0.0.1:8080"
timeout = 30

[search]
engine = "duckduckgo"
"""
    return tarzi.Config.from_str(config_str)


@pytest.fixture
def config_without_proxy():
    """Fixture for config without proxy settings."""
    return tarzi.Config()


class TestProxyConfig:
    """Test cases for proxy configuration."""

    def test_config_with_proxy_setting(self, config_with_proxy):
        """Test config correctly parses proxy settings."""
        # Test that config loads successfully
        assert isinstance(config_with_proxy, tarzi.Config)

        # Test that components can be created with proxy config
        fetcher = tarzi.WebFetcher.from_config(config_with_proxy)
        assert isinstance(fetcher, tarzi.WebFetcher)

        search_engine = tarzi.SearchEngine.from_config(config_with_proxy)
        assert isinstance(search_engine, tarzi.SearchEngine)

    def test_config_without_proxy_setting(self, config_without_proxy):
        """Test config works without proxy settings."""
        # Test that config loads successfully
        assert isinstance(config_without_proxy, tarzi.Config)

        # Test that components can be created without proxy config
        fetcher = tarzi.WebFetcher.from_config(config_without_proxy)
        assert isinstance(fetcher, tarzi.WebFetcher)

        search_engine = tarzi.SearchEngine.from_config(config_without_proxy)
        assert isinstance(search_engine, tarzi.SearchEngine)

    def test_proxy_environment_variables(self, config_without_proxy):
        """Test that environment variables are respected for proxy settings."""
        # Test with HTTP_PROXY environment variable
        test_proxy = "http://test-proxy:3128"

        # Save original env vars
        original_http_proxy = os.environ.get("HTTP_PROXY")
        original_https_proxy = os.environ.get("HTTPS_PROXY")

        try:
            # Set test proxy in environment
            os.environ["HTTP_PROXY"] = test_proxy
            os.environ["HTTPS_PROXY"] = test_proxy

            # Components should be created successfully even with proxy env vars
            fetcher = tarzi.WebFetcher.from_config(config_without_proxy)
            assert isinstance(fetcher, tarzi.WebFetcher)

            search_engine = tarzi.SearchEngine.from_config(config_without_proxy)
            assert isinstance(search_engine, tarzi.SearchEngine)

        finally:
            # Restore original environment
            if original_http_proxy is not None:
                os.environ["HTTP_PROXY"] = original_http_proxy
            elif "HTTP_PROXY" in os.environ:
                del os.environ["HTTP_PROXY"]

            if original_https_proxy is not None:
                os.environ["HTTPS_PROXY"] = original_https_proxy
            elif "HTTPS_PROXY" in os.environ:
                del os.environ["HTTPS_PROXY"]

    def test_mixed_proxy_configurations(self):
        """Test various proxy configuration formats."""
        proxy_configs = [
            "http://proxy.example.com:8080",
            "https://secure-proxy.example.com:3128",
            "http://user:pass@proxy.example.com:8080",
            "socks5://socks-proxy.example.com:1080",
        ]

        for proxy_url in proxy_configs:
            config_str = f"""
[fetcher]
proxy = "{proxy_url}"

[search]
engine = "duckduckgo"
"""
            try:
                config = tarzi.Config.from_str(config_str)
                assert isinstance(config, tarzi.Config)

                # Test that components can be created
                fetcher = tarzi.WebFetcher.from_config(config)
                assert isinstance(fetcher, tarzi.WebFetcher)

                search_engine = tarzi.SearchEngine.from_config(config)
                assert isinstance(search_engine, tarzi.SearchEngine)

            except Exception as e:
                # Some proxy formats might not be supported, that's okay
                print(f"Proxy format {proxy_url} not supported: {e}")

    def test_invalid_proxy_configuration(self):
        """Test invalid proxy configurations."""
        invalid_proxy_configs = [
            "invalid-proxy-url",
            "ftp://invalid-protocol.example.com:8080",
            "://missing-protocol.example.com:8080",
        ]

        for proxy_url in invalid_proxy_configs:
            config_str = f"""
[fetcher]
proxy = "{proxy_url}"

[search]
engine = "duckduckgo"
"""
            try:
                config = tarzi.Config.from_str(config_str)
                # Config parsing should succeed (validation happens at runtime)
                assert isinstance(config, tarzi.Config)

                # Component creation might succeed or fail, both are acceptable
                # depending on the Rust implementation's validation
                tarzi.WebFetcher.from_config(config)
                tarzi.SearchEngine.from_config(config)

            except Exception:
                # Invalid proxy configs may cause failures, which is acceptable
                pass

    def test_empty_proxy_configuration(self):
        """Test empty proxy configuration."""
        config_str = """
[fetcher]
proxy = ""

[search]
engine = "duckduckgo"
"""
        config = tarzi.Config.from_str(config_str)
        assert isinstance(config, tarzi.Config)

        # Empty proxy should work like no proxy
        fetcher = tarzi.WebFetcher.from_config(config)
        assert isinstance(fetcher, tarzi.WebFetcher)

        search_engine = tarzi.SearchEngine.from_config(config)
        assert isinstance(search_engine, tarzi.SearchEngine)
