#!/usr/bin/env python3
"""
Unit tests for updated Config structure in tarzi.
Tests the new configuration without deprecated api_key field.
"""

import pytest
import tarzi


@pytest.fixture
def modern_config():
    """Fixture for modern configuration with specific API keys."""
    return """
[general]
log_level = "debug"
timeout = 60

[fetcher]
mode = "browser_headless"
format = "markdown"
timeout = 30
proxy = "http://proxy.example.com:8080"

[search]
mode = "apiquery"
engine = "brave"
limit = 10
autoswitch = "smart"
brave_api_key = "brave_key_123"
google_serper_api_key = "serper_key_456"
exa_api_key = "exa_key_789"
travily_api_key = "travily_key_000"
"""


@pytest.fixture
def minimal_config():
    """Fixture for minimal configuration."""
    return """
[search]
engine = "duckduckgo"
"""


class TestUpdatedConfig:
    """Test cases for updated configuration structure."""

    def test_config_without_deprecated_api_key(self, modern_config):
        """Test that config loads successfully without deprecated api_key field."""
        config = tarzi.Config.from_str(modern_config)
        assert isinstance(config, tarzi.Config)

    def test_new_api_key_fields_support(self, modern_config):
        """Test that new specific API key fields are supported."""
        config = tarzi.Config.from_str(modern_config)

        # Test that components can be created with new config structure
        search_engine = tarzi.SearchEngine.from_config(config)
        assert isinstance(search_engine, tarzi.SearchEngine)

        fetcher = tarzi.WebFetcher.from_config(config)
        assert isinstance(fetcher, tarzi.WebFetcher)

        converter = tarzi.Converter.from_config(config)
        assert isinstance(converter, tarzi.Converter)

    def test_autoswitch_configuration(self):
        """Test autoswitch configuration options."""
        autoswitch_configs = [
            'autoswitch = "smart"',
            'autoswitch = "none"',
            'autoswitch = "Smart"',
            'autoswitch = "None"',
            'autoswitch = "SMART"',
            'autoswitch = "NONE"',
        ]

        for autoswitch_line in autoswitch_configs:
            config_str = f"""
[search]
engine = "duckduckgo"
{autoswitch_line}
"""
            config = tarzi.Config.from_str(config_str)
            assert isinstance(config, tarzi.Config)

            # Should be able to create search engine
            engine = tarzi.SearchEngine.from_config(config)
            assert isinstance(engine, tarzi.SearchEngine)

    def test_multiple_api_providers_configuration(self):
        """Test configuration with multiple API providers."""
        config_str = """
[search]
engine = "brave"
brave_api_key = "brave_test_key"
google_serper_api_key = "serper_test_key"
exa_api_key = "exa_test_key"
travily_api_key = "travily_test_key"
autoswitch = "smart"
"""
        config = tarzi.Config.from_str(config_str)
        assert isinstance(config, tarzi.Config)

        # Should be able to create components with multiple providers
        engine = tarzi.SearchEngine.from_config(config)
        assert isinstance(engine, tarzi.SearchEngine)

    def test_proxy_configuration_integration(self):
        """Test proxy configuration works with all components."""
        config_str = """
[fetcher]
proxy = "http://proxy.company.com:8080"
timeout = 45

[search]
engine = "brave"
brave_api_key = "test_key"
"""
        config = tarzi.Config.from_str(config_str)
        assert isinstance(config, tarzi.Config)

        # All components should handle proxy configuration
        fetcher = tarzi.WebFetcher.from_config(config)
        assert isinstance(fetcher, tarzi.WebFetcher)

        engine = tarzi.SearchEngine.from_config(config)
        assert isinstance(engine, tarzi.SearchEngine)

    def test_web_driver_configuration(self):
        """Test web driver configuration options."""
        driver_configs = [
            ('web_driver = "geckodriver"', None),
            ('web_driver = "chromedriver"', None),
            ('web_driver = "geckodriver"\nweb_driver_url = "http://selenium-hub:4444"', "http://selenium-hub:4444"),
        ]

        for driver_config, expected_url in driver_configs:
            config_str = f"""
[fetcher]
{driver_config}

[search]
engine = "duckduckgo"
"""
            config = tarzi.Config.from_str(config_str)
            assert isinstance(config, tarzi.Config)

            # Should be able to create fetcher with driver config
            fetcher = tarzi.WebFetcher.from_config(config)
            assert isinstance(fetcher, tarzi.WebFetcher)

    def test_search_engine_options(self):
        """Test different search engine configuration options."""
        engines = ["duckduckgo", "brave", "googleserper", "exa", "travily"]

        for engine in engines:
            config_str = f"""
[search]
engine = "{engine}"
"""
            config = tarzi.Config.from_str(config_str)
            assert isinstance(config, tarzi.Config)

            # Should be able to create search engine
            search_engine = tarzi.SearchEngine.from_config(config)
            assert isinstance(search_engine, tarzi.SearchEngine)

    def test_fetcher_mode_options(self):
        """Test different fetcher mode configuration options."""
        modes = ["plain_request", "browser_headless", "browser_full"]

        for mode in modes:
            config_str = f"""
[fetcher]
mode = "{mode}"

[search]
engine = "duckduckgo"
"""
            config = tarzi.Config.from_str(config_str)
            assert isinstance(config, tarzi.Config)

            # Should be able to create fetcher
            fetcher = tarzi.WebFetcher.from_config(config)
            assert isinstance(fetcher, tarzi.WebFetcher)

    def test_format_options(self):
        """Test different format configuration options."""
        formats = ["html", "markdown", "json", "yaml"]

        for format_type in formats:
            config_str = f"""
[fetcher]
format = "{format_type}"

[search]
engine = "duckduckgo"
"""
            config = tarzi.Config.from_str(config_str)
            assert isinstance(config, tarzi.Config)

            # Should be able to create components
            fetcher = tarzi.WebFetcher.from_config(config)
            assert isinstance(fetcher, tarzi.WebFetcher)

            converter = tarzi.Converter.from_config(config)
            assert isinstance(converter, tarzi.Converter)

    def test_timeout_configurations(self):
        """Test timeout configuration options."""
        config_str = """
[general]
timeout = 120

[fetcher]
timeout = 60

[search]
engine = "duckduckgo"
"""
        config = tarzi.Config.from_str(config_str)
        assert isinstance(config, tarzi.Config)

        # Components should handle timeout configuration
        fetcher = tarzi.WebFetcher.from_config(config)
        assert isinstance(fetcher, tarzi.WebFetcher)

        engine = tarzi.SearchEngine.from_config(config)
        assert isinstance(engine, tarzi.SearchEngine)

    def test_search_limit_configuration(self):
        """Test search limit configuration."""
        limits = [1, 5, 10, 20, 50]

        for limit in limits:
            config_str = f"""
[search]
engine = "duckduckgo"
limit = {limit}
"""
            config = tarzi.Config.from_str(config_str)
            assert isinstance(config, tarzi.Config)

            # Should be able to create search engine
            engine = tarzi.SearchEngine.from_config(config)
            assert isinstance(engine, tarzi.SearchEngine)

    def test_invalid_configuration_handling(self):
        """Test handling of invalid configuration values."""
        invalid_configs = [
            # Invalid autoswitch value should default gracefully
            """
[search]
engine = "duckduckgo"
autoswitch = "invalid_value"
""",
            # Invalid limit should handle gracefully or error appropriately
            """
[search]
engine = "duckduckgo"
limit = -1
""",
            # Invalid timeout should handle gracefully
            """
[general]
timeout = -5

[search]
engine = "duckduckgo"
""",
        ]

        for config_str in invalid_configs:
            try:
                config = tarzi.Config.from_str(config_str)
                # Configuration parsing might succeed even with invalid values
                # (validation happens at runtime)
                assert isinstance(config, tarzi.Config)

                # Try to create components - might succeed or fail
                engine = tarzi.SearchEngine.from_config(config)
                assert isinstance(engine, tarzi.SearchEngine)

            except Exception:
                # Invalid configs may cause failures, which is acceptable
                pass

    def test_empty_configuration_defaults(self):
        """Test that empty configuration uses appropriate defaults."""
        config_str = ""
        config = tarzi.Config.from_str(config_str)
        assert isinstance(config, tarzi.Config)

        # Should be able to create components with default configuration
        fetcher = tarzi.WebFetcher.from_config(config)
        assert isinstance(fetcher, tarzi.WebFetcher)

        engine = tarzi.SearchEngine.from_config(config)
        assert isinstance(engine, tarzi.SearchEngine)

        converter = tarzi.Converter.from_config(config)
        assert isinstance(converter, tarzi.Converter)

    def test_partial_configuration_sections(self):
        """Test configuration with only some sections defined."""
        partial_configs = [
            # Only general section
            """
[general]
log_level = "debug"
""",
            # Only fetcher section
            """
[fetcher]
mode = "plain_request"
""",
            # Only search section
            """
[search]
engine = "brave"
brave_api_key = "test_key"
""",
        ]

        for config_str in partial_configs:
            config = tarzi.Config.from_str(config_str)
            assert isinstance(config, tarzi.Config)

            # Should be able to create components even with partial config
            fetcher = tarzi.WebFetcher.from_config(config)
            assert isinstance(fetcher, tarzi.WebFetcher)

            engine = tarzi.SearchEngine.from_config(config)
            assert isinstance(engine, tarzi.SearchEngine)

            converter = tarzi.Converter.from_config(config)
            assert isinstance(converter, tarzi.Converter)
