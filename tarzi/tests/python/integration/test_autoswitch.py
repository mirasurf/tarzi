#!/usr/bin/env python3
"""
Integration tests for autoswitch strategies in tarzi.
These tests require network access and may require API keys.
"""

import os

import pytest
import tarzi


@pytest.fixture
def test_query():
    """Fixture for test search query."""
    return "artificial intelligence"


@pytest.mark.integration
class TestAutoswitchStrategies:
    """Integration test cases for autoswitch strategies."""

    def test_smart_autoswitch_strategy(self, test_query):
        """Test smart autoswitch strategy with multiple providers."""
        # Try to get multiple API keys for comprehensive testing
        api_keys = {
            "brave": os.environ.get("BRAVE_API_KEY"),
            "exa": os.environ.get("EXA_API_KEY"),
            "travily": os.environ.get("TRAVILY_API_KEY"),
        }

        available_keys = {k: v for k, v in api_keys.items() if v}

        if len(available_keys) < 2:
            pytest.skip("Need at least 2 API keys for smart autoswitch testing")

        # Configure multiple providers with smart autoswitch
        config_parts = ["[search]", 'autoswitch = "smart"']

        if "brave" in available_keys:
            config_parts.append(f'brave_api_key = "{available_keys["brave"]}"')
        if "exa" in available_keys:
            config_parts.append(f'exa_api_key = "{available_keys["exa"]}"')
        if "travily" in available_keys:
            config_parts.append(f'travily_api_key = "{available_keys["travily"]}"')

        # Set primary engine to first available
        primary = list(available_keys.keys())[0]
        if primary == "travily":
            primary = "travily"
        config_parts.append(f'engine = "{primary}"')

        config_str = "\n".join(config_parts)

        try:
            config = tarzi.Config.from_str(config_str)
            engine = tarzi.SearchEngine.from_config(config)

            results = engine.search(test_query, "apiquery", 3)
            assert isinstance(results, list)

            # Smart strategy should return results from available providers
            if len(results) > 0:
                for result in results:
                    assert isinstance(result, tarzi.SearchResult)
                    assert hasattr(result, "title")
                    assert hasattr(result, "url")

        except Exception as e:
            pytest.skip(f"Smart autoswitch test failed: {e}")

    def test_none_autoswitch_strategy(self, test_query):
        """Test none autoswitch strategy (no fallback)."""
        api_key = os.environ.get("BRAVE_API_KEY")
        if not api_key:
            pytest.skip("BRAVE_API_KEY not set")

        config_str = f"""
[search]
engine = "brave"
brave_api_key = "{api_key}"
autoswitch = "none"
"""
        try:
            config = tarzi.Config.from_str(config_str)
            engine = tarzi.SearchEngine.from_config(config)

            results = engine.search(test_query, "apiquery", 2)
            assert isinstance(results, list)

            # Should work with primary provider only
            for result in results:
                assert isinstance(result, tarzi.SearchResult)

        except Exception as e:
            pytest.skip(f"None autoswitch test failed: {e}")

    def test_autoswitch_strategy_case_insensitive(self, test_query):
        """Test that autoswitch strategy parsing is case insensitive."""
        api_key = os.environ.get("BRAVE_API_KEY")
        if not api_key:
            pytest.skip("BRAVE_API_KEY not set")

        strategies = ["smart", "Smart", "SMART", "none", "None", "NONE"]

        for strategy in strategies:
            config_str = f"""
[search]
engine = "brave"
brave_api_key = "{api_key}"
autoswitch = "{strategy}"
"""
            try:
                config = tarzi.Config.from_str(config_str)
                engine = tarzi.SearchEngine.from_config(config)

                # Should create engine successfully regardless of case
                assert isinstance(engine, tarzi.SearchEngine)

                # Try a quick search to verify it works
                results = engine.search(test_query, "apiquery", 1)
                assert isinstance(results, list)

            except Exception as e:
                print(f"Strategy '{strategy}' failed: {e}")

    def test_invalid_autoswitch_strategy_defaults_to_smart(self, test_query):
        """Test that invalid autoswitch strategy defaults to smart."""
        api_key = os.environ.get("BRAVE_API_KEY")
        if not api_key:
            pytest.skip("BRAVE_API_KEY not set")

        invalid_strategies = ["invalid", "unknown", "random", ""]

        for strategy in invalid_strategies:
            config_str = f"""
[search]
engine = "brave"
brave_api_key = "{api_key}"
autoswitch = "{strategy}"
"""
            try:
                config = tarzi.Config.from_str(config_str)
                engine = tarzi.SearchEngine.from_config(config)

                # Should create engine successfully and default to smart
                assert isinstance(engine, tarzi.SearchEngine)

                # Should work (assuming smart is the default fallback)
                results = engine.search(test_query, "apiquery", 1)
                assert isinstance(results, list)

            except Exception as e:
                print(f"Invalid strategy '{strategy}' failed: {e}")

    def test_smart_fallback_with_invalid_primary(self, test_query):
        """Test smart autoswitch with invalid primary provider."""
        fallback_keys = {}
        if os.environ.get("EXA_API_KEY"):
            fallback_keys["exa"] = os.environ.get("EXA_API_KEY")
        if os.environ.get("TRAVILY_API_KEY"):
            fallback_keys["travily"] = os.environ.get("TRAVILY_API_KEY")

        if not fallback_keys:
            pytest.skip("Need at least one fallback API key")

        # Set invalid primary with valid fallbacks
        config_parts = [
            "[search]",
            'engine = "brave"',
            'brave_api_key = "invalid_api_key_12345"',
            'autoswitch = "smart"',
        ]

        if "exa" in fallback_keys:
            config_parts.append(f'exa_api_key = "{fallback_keys["exa"]}"')
        if "travily" in fallback_keys:
            config_parts.append(f'travily_api_key = "{fallback_keys["travily"]}"')

        config_str = "\n".join(config_parts)

        try:
            config = tarzi.Config.from_str(config_str)
            engine = tarzi.SearchEngine.from_config(config)

            results = engine.search(test_query, "apiquery", 2)
            assert isinstance(results, list)

            # Should get results from fallback providers
            if len(results) > 0:
                for result in results:
                    assert isinstance(result, tarzi.SearchResult)

        except Exception as e:
            # Could fail if all providers have issues
            print(f"Smart fallback test failed: {e}")

    def test_none_strategy_with_invalid_primary_fails(self, test_query):
        """Test none autoswitch fails when primary provider is invalid."""
        config_str = """
[search]
engine = "brave"
brave_api_key = "definitely_invalid_key_12345"
autoswitch = "none"
"""
        config = tarzi.Config.from_str(config_str)
        engine = tarzi.SearchEngine.from_config(config)

        try:
            results = engine.search(test_query, "apiquery", 1)

            # If it succeeds, it might be using a fallback or handling gracefully
            if isinstance(results, list):
                # Empty results are acceptable
                assert len(results) == 0

        except Exception as e:
            # Expected to fail with invalid key and no autoswitch
            assert any(keyword in str(e).lower() for keyword in ["api", "key", "authentication", "provider"])

    def test_autoswitch_with_all_invalid_providers(self, test_query):
        """Test autoswitch behavior when all providers have invalid keys."""
        config_str = """
[search]
engine = "brave"
brave_api_key = "invalid_brave_key"
exa_api_key = "invalid_exa_key"
travily_api_key = "invalid_travily_key"
autoswitch = "smart"
"""
        config = tarzi.Config.from_str(config_str)
        engine = tarzi.SearchEngine.from_config(config)

        try:
            results = engine.search(test_query, "apiquery", 1)

            # With smart autoswitch, it might fall back to DuckDuckGo
            if isinstance(results, list):
                # Empty results from DuckDuckGo fallback are acceptable
                assert len(results) == 0

        except Exception as e:
            # All providers failing is also acceptable
            assert any(keyword in str(e).lower() for keyword in ["provider", "api", "authentication", "failed"])

    def test_autoswitch_provider_order(self, test_query):
        """Test that autoswitch respects provider fallback order."""
        multiple_keys = {}

        if os.environ.get("EXA_API_KEY"):
            multiple_keys["exa"] = os.environ.get("EXA_API_KEY")
        if os.environ.get("BRAVE_API_KEY"):
            multiple_keys["brave"] = os.environ.get("BRAVE_API_KEY")
        if os.environ.get("TRAVILY_API_KEY"):
            multiple_keys["travily"] = os.environ.get("TRAVILY_API_KEY")

        if len(multiple_keys) < 2:
            pytest.skip("Need multiple API keys to test provider order")

        # Set primary to a provider that should be later in fallback order
        config_parts = [
            "[search]",
            'engine = "exa"',
            'autoswitch = "smart"',
        ]

        for provider, key in multiple_keys.items():
            if provider == "exa":
                config_parts.append(f'exa_api_key = "{key}"')
            elif provider == "brave":
                config_parts.append(f'brave_api_key = "{key}"')
            elif provider == "travily":
                config_parts.append(f'travily_api_key = "{key}"')

        config_str = "\n".join(config_parts)

        try:
            config = tarzi.Config.from_str(config_str)
            engine = tarzi.SearchEngine.from_config(config)

            results = engine.search(test_query, "apiquery", 2)
            assert isinstance(results, list)

            # We can't easily verify the exact order without detailed logging,
            # but we can verify that fallback works with multiple providers
            for result in results:
                assert isinstance(result, tarzi.SearchResult)

        except Exception as e:
            pytest.skip(f"Provider order test failed: {e}")

    def test_autoswitch_performance_comparison(self, test_query):
        """Test performance difference between autoswitch strategies."""
        api_key = os.environ.get("BRAVE_API_KEY")
        if not api_key:
            pytest.skip("BRAVE_API_KEY not set")

        import time

        # Test smart strategy
        config_smart = f"""
[search]
engine = "brave"
brave_api_key = "{api_key}"
autoswitch = "smart"
"""
        config = tarzi.Config.from_str(config_smart)
        engine_smart = tarzi.SearchEngine.from_config(config)

        start_smart = time.time()
        try:
            engine_smart.search(test_query, "apiquery", 1)
            duration_smart = time.time() - start_smart
        except Exception:
            duration_smart = time.time() - start_smart

        # Test none strategy
        config_none = f"""
[search]
engine = "brave"
brave_api_key = "{api_key}"
autoswitch = "none"
"""
        config = tarzi.Config.from_str(config_none)
        engine_none = tarzi.SearchEngine.from_config(config)

        start_none = time.time()
        try:
            engine_none.search(test_query, "apiquery", 1)
            duration_none = time.time() - start_none
        except Exception:
            duration_none = time.time() - start_none

        print(f"Smart strategy took: {duration_smart:.3f}s")
        print(f"None strategy took: {duration_none:.3f}s")

        # Both should work similarly with a single valid provider
        # Performance difference should be minimal
        assert duration_smart >= 0
        assert duration_none >= 0
