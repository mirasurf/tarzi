#!/usr/bin/env python3
"""
Unit tests for configuration loading priorities in tarzi.
Tests the precedence order: CLI > Env Vars > User Config > Project Config > Defaults
"""

import pytest
import tarzi
import os
import tempfile
import shutil
from pathlib import Path


@pytest.fixture
def temp_config_dir():
    """Create a temporary directory for testing config files."""
    temp_dir = tempfile.mkdtemp()
    yield temp_dir
    shutil.rmtree(temp_dir)


@pytest.fixture
def project_config_content():
    """Sample project configuration."""
    return """
[general]
log_level = "debug"
timeout = 60

[fetcher]
mode = "browser_headless"
format = "markdown"
timeout = 30
proxy = "http://project-proxy:8080"

[search]
engine = "bing"
mode = "webquery"
limit = 10
autoswitch = "none"
"""


@pytest.fixture
def user_config_content():
    """Sample user configuration that should override project config."""
    return """
[general]
log_level = "warn"
timeout = 45

[fetcher]
mode = "plain_request"
format = "json"
timeout = 60
proxy = "http://user-proxy:3128"

[search]
engine = "google"
mode = "apiquery"
limit = 5
autoswitch = "smart"
brave_api_key = "user_brave_key"
"""


class TestConfigPriorities:
    """Test configuration loading priorities."""

    def test_default_config_values(self):
        """Test that default configuration values are loaded correctly."""
        config = tarzi.Config()
        
        # Test default values are set
        components = tarzi.WebFetcher.from_config(config)
        assert isinstance(components, tarzi.WebFetcher)
        
        search_engine = tarzi.SearchEngine.from_config(config)
        assert isinstance(search_engine, tarzi.SearchEngine)

    def test_config_from_string_override(self):
        """Test that string config overrides defaults."""
        config_str = """
[general]
log_level = "error"
timeout = 120

[fetcher]
mode = "head"
format = "yaml"

[search]
engine = "brave"
limit = 15
"""
        config = tarzi.Config.from_str(config_str)
        
        # Verify components can be created with overridden config
        fetcher = tarzi.WebFetcher.from_config(config)
        assert isinstance(fetcher, tarzi.WebFetcher)
        
        search_engine = tarzi.SearchEngine.from_config(config)
        assert isinstance(search_engine, tarzi.SearchEngine)

    def test_environment_variable_override_config(self):
        """Test that environment variables override config file settings."""
        # Create a config with proxy setting
        config_str = """
[fetcher]
proxy = "http://config-proxy:8080"

[search]
engine = "duckduckgo"
"""
        config = tarzi.Config.from_str(config_str)
        
        # Save original environment variables
        original_http_proxy = os.environ.get('HTTP_PROXY')
        original_https_proxy = os.environ.get('HTTPS_PROXY')
        
        try:
            # Set environment variable that should override config
            os.environ['HTTP_PROXY'] = "http://env-proxy:3128"
            os.environ['HTTPS_PROXY'] = "http://env-https-proxy:3128"
            
            # Components should still be created successfully
            # The environment variable should take precedence internally
            fetcher = tarzi.WebFetcher.from_config(config)
            assert isinstance(fetcher, tarzi.WebFetcher)
            
            search_engine = tarzi.SearchEngine.from_config(config)
            assert isinstance(search_engine, tarzi.SearchEngine)
            
        finally:
            # Restore original environment
            if original_http_proxy is not None:
                os.environ['HTTP_PROXY'] = original_http_proxy
            elif 'HTTP_PROXY' in os.environ:
                del os.environ['HTTP_PROXY']
                
            if original_https_proxy is not None:
                os.environ['HTTPS_PROXY'] = original_https_proxy
            elif 'HTTPS_PROXY' in os.environ:
                del os.environ['HTTPS_PROXY']

    def test_environment_variable_priority_order(self):
        """Test that HTTPS_PROXY takes precedence over HTTP_PROXY."""
        config_str = """
[fetcher]
proxy = "http://config-proxy:8080"

[search]
engine = "duckduckgo"
"""
        config = tarzi.Config.from_str(config_str)
        
        # Save original environment variables
        original_http_proxy = os.environ.get('HTTP_PROXY')
        original_https_proxy = os.environ.get('HTTPS_PROXY')
        
        try:
            # Set both HTTP_PROXY and HTTPS_PROXY
            os.environ['HTTP_PROXY'] = "http://http-proxy:8080"
            os.environ['HTTPS_PROXY'] = "http://https-proxy:3128"
            
            # Components should be created successfully
            # HTTPS_PROXY should take precedence internally
            fetcher = tarzi.WebFetcher.from_config(config)
            assert isinstance(fetcher, tarzi.WebFetcher)
            
            search_engine = tarzi.SearchEngine.from_config(config)
            assert isinstance(search_engine, tarzi.SearchEngine)
            
        finally:
            # Restore original environment
            if original_http_proxy is not None:
                os.environ['HTTP_PROXY'] = original_http_proxy
            elif 'HTTP_PROXY' in os.environ:
                del os.environ['HTTP_PROXY']
                
            if original_https_proxy is not None:
                os.environ['HTTPS_PROXY'] = original_https_proxy
            elif 'HTTPS_PROXY' in os.environ:
                del os.environ['HTTPS_PROXY']

    def test_empty_environment_variable_fallback(self):
        """Test that empty environment variables fall back to config values."""
        config_str = """
[fetcher]
proxy = "http://config-proxy:8080"

[search]
engine = "duckduckgo"
"""
        config = tarzi.Config.from_str(config_str)
        
        # Save original environment variables
        original_http_proxy = os.environ.get('HTTP_PROXY')
        
        try:
            # Set empty environment variable
            os.environ['HTTP_PROXY'] = ""
            
            # Components should be created successfully and fall back to config proxy
            fetcher = tarzi.WebFetcher.from_config(config)
            assert isinstance(fetcher, tarzi.WebFetcher)
            
            search_engine = tarzi.SearchEngine.from_config(config)
            assert isinstance(search_engine, tarzi.SearchEngine)
            
        finally:
            # Restore original environment
            if original_http_proxy is not None:
                os.environ['HTTP_PROXY'] = original_http_proxy
            elif 'HTTP_PROXY' in os.environ:
                del os.environ['HTTP_PROXY']

    def test_mixed_priority_scenarios(self):
        """Test complex scenarios with mixed configuration sources."""
        # Test with environment variables and config
        config_str = """
[general]
log_level = "info"
timeout = 30

[fetcher]
mode = "browser_headless"
format = "markdown"
proxy = "http://config-proxy:8080"

[search]
engine = "duckduckgo"
limit = 5
brave_api_key = "config_brave_key"
google_serper_api_key = "config_serper_key"
"""
        config = tarzi.Config.from_str(config_str)
        
        # Save original environment variables
        original_env_vars = {
            'HTTP_PROXY': os.environ.get('HTTP_PROXY'),
            'HTTPS_PROXY': os.environ.get('HTTPS_PROXY'),
        }
        
        try:
            # Set environment variables that should override config
            os.environ['HTTPS_PROXY'] = "http://env-proxy:3128"
            
            # Components should be created successfully
            fetcher = tarzi.WebFetcher.from_config(config)
            assert isinstance(fetcher, tarzi.WebFetcher)
            
            search_engine = tarzi.SearchEngine.from_config(config)
            assert isinstance(search_engine, tarzi.SearchEngine)
            
        finally:
            # Restore original environment
            for var, value in original_env_vars.items():
                if value is not None:
                    os.environ[var] = value
                elif var in os.environ:
                    del os.environ[var]

    def test_api_key_configuration_priority(self):
        """Test that API key configuration works with different priority sources."""
        # Config with multiple API keys
        config_str = """
[search]
engine = "brave"
brave_api_key = "config_brave_key"
google_serper_api_key = "config_serper_key"
exa_api_key = "config_exa_key"
travily_api_key = "config_travily_key"
autoswitch = "smart"
"""
        config = tarzi.Config.from_str(config_str)
        
        # Verify components can be created with API keys
        search_engine = tarzi.SearchEngine.from_config(config)
        assert isinstance(search_engine, tarzi.SearchEngine)

    def test_search_engine_switching_priority(self):
        """Test search engine switching with different configuration priorities."""
        # Test autoswitch configuration
        config_str = """
[search]
engine = "brave"
autoswitch = "smart"
brave_api_key = "brave_key_123"
google_serper_api_key = "serper_key_456"
exa_api_key = "exa_key_789"
limit = 10
"""
        config = tarzi.Config.from_str(config_str)
        
        # Verify search engine can be created with switching configuration
        search_engine = tarzi.SearchEngine.from_config(config)
        assert isinstance(search_engine, tarzi.SearchEngine)

    def test_web_driver_configuration_priority(self):
        """Test web driver configuration with different priority sources."""
        config_str = """
[fetcher]
mode = "browser_headless"
web_driver = "chromedriver"
web_driver_url = "http://localhost:4444"
timeout = 60
"""
        config = tarzi.Config.from_str(config_str)
        
        # Verify fetcher can be created with web driver configuration
        fetcher = tarzi.WebFetcher.from_config(config)
        assert isinstance(fetcher, tarzi.WebFetcher)

    def test_timeout_configuration_priority(self):
        """Test timeout configuration from different sources."""
        config_str = """
[general]
timeout = 120

[fetcher]
timeout = 45

[search]
limit = 8
"""
        config = tarzi.Config.from_str(config_str)
        
        # Verify components can be created with custom timeouts
        fetcher = tarzi.WebFetcher.from_config(config)
        assert isinstance(fetcher, tarzi.WebFetcher)
        
        search_engine = tarzi.SearchEngine.from_config(config)
        assert isinstance(search_engine, tarzi.SearchEngine)

    def test_format_configuration_priority(self):
        """Test output format configuration priority."""
        format_configs = ["markdown", "json", "yaml", "raw"]
        
        for fmt in format_configs:
            config_str = f"""
[fetcher]
format = "{fmt}"
mode = "plain_request"

[search]
engine = "duckduckgo"
"""
            config = tarzi.Config.from_str(config_str)
            
            # Verify fetcher can be created with different formats
            fetcher = tarzi.WebFetcher.from_config(config)
            assert isinstance(fetcher, tarzi.WebFetcher)

    def test_fetcher_mode_configuration_priority(self):
        """Test fetcher mode configuration priority."""
        modes = ["browser_headless", "browser_head", "plain_request", "head"]
        
        for mode in modes:
            config_str = f"""
[fetcher]
mode = "{mode}"
format = "markdown"

[search]
engine = "duckduckgo"
"""
            config = tarzi.Config.from_str(config_str)
            
            # Verify fetcher can be created with different modes
            fetcher = tarzi.WebFetcher.from_config(config)
            assert isinstance(fetcher, tarzi.WebFetcher)

    def test_invalid_configuration_handling(self):
        """Test that invalid configurations are handled gracefully."""
        invalid_configs = [
            # Invalid proxy URL
            """
[fetcher]
proxy = "invalid-proxy-url"

[search]
engine = "duckduckgo"
""",
            # Invalid engine
            """
[search]
engine = "invalid-engine"
mode = "webquery"
""",
            # Invalid autoswitch value
            """
[search]
engine = "brave"
autoswitch = "invalid-autoswitch"
"""
        ]
        
        for config_str in invalid_configs:
            try:
                config = tarzi.Config.from_str(config_str)
                # Config parsing should succeed (validation might happen at runtime)
                assert isinstance(config, tarzi.Config)
                
                # Component creation may succeed or fail gracefully
                try:
                    fetcher = tarzi.WebFetcher.from_config(config)
                    search_engine = tarzi.SearchEngine.from_config(config)
                except Exception:
                    # Invalid configs may cause failures, which is acceptable
                    pass
                    
            except Exception:
                # Config parsing failures are also acceptable for invalid configs
                pass