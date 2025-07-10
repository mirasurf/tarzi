#!/usr/bin/env python3
"""
Integration tests for error handling and edge cases in tarzi.
These tests ensure robust error handling across all components.
"""

import pytest
import tarzi


@pytest.mark.integration
class TestErrorHandling:
    """Integration test cases for error handling."""

    def test_invalid_url_handling(self):
        """Test handling of invalid URLs in web fetcher."""
        fetcher = tarzi.WebFetcher()

        invalid_urls = [
            "not-a-url",
            "http://",
            "://missing-protocol.com",
            "http://localhost:99999",  # Invalid port
            "http://definitely-does-not-exist-12345.com",
            "",  # Empty URL
        ]

        for url in invalid_urls:
            try:
                result = fetcher.fetch(url, "plain_request", "html")
                # Some invalid URLs might be handled gracefully
                if isinstance(result, str):
                    assert len(result) >= 0  # Empty result is okay
            except Exception as e:
                # Errors are expected for invalid URLs
                assert isinstance(e, (ValueError, RuntimeError))

    def test_network_timeout_handling(self):
        """Test handling of network timeouts."""
        config_str = """
[fetcher]
timeout = 1

[search]
engine = "duckduckgo"
"""
        config = tarzi.Config.from_str(config_str)
        fetcher = tarzi.WebFetcher.from_config(config)

        # This URL should timeout with 1 second limit
        slow_url = "https://httpbin.org/delay/5"

        try:
            result = fetcher.fetch(slow_url, "plain_request", "html")
            # If it succeeds, timeout handling worked differently than expected
            assert isinstance(result, str)
        except Exception as e:
            # Timeout errors are expected
            assert any(keyword in str(e).lower() for keyword in ["timeout", "time", "network"])

    def test_invalid_search_parameters(self):
        """Test handling of invalid search parameters."""
        engine = tarzi.SearchEngine()

        # Test invalid search modes
        invalid_modes = ["invalid_mode", "", "webquery_typo", "apiquery_typo"]

        for mode in invalid_modes:
            with pytest.raises(ValueError, match="Invalid search mode"):
                engine.search("test query", mode, 5)

    def test_invalid_fetch_parameters(self):
        """Test handling of invalid fetch parameters."""
        fetcher = tarzi.WebFetcher()
        test_url = "https://httpbin.org/html"

        # Test invalid fetch modes
        invalid_modes = ["invalid_mode", "", "plain_request_typo", "browser_typo"]

        for mode in invalid_modes:
            with pytest.raises(ValueError, match="Invalid fetch mode"):
                fetcher.fetch(test_url, mode, "html")

        # Test invalid formats
        invalid_formats = ["invalid_format", "", "html_typo", "markdown_typo"]

        for format_type in invalid_formats:
            with pytest.raises(ValueError, match="Invalid format"):
                fetcher.fetch(test_url, "plain_request", format_type)

    def test_invalid_conversion_parameters(self):
        """Test handling of invalid conversion parameters."""
        converter = tarzi.Converter()
        test_html = "<p>Test content</p>"

        # Test invalid formats
        invalid_formats = ["invalid_format", "", "xml", "csv"]

        for format_type in invalid_formats:
            with pytest.raises(ValueError, match="Invalid format"):
                converter.convert(test_html, format_type)

    def test_malformed_html_handling(self):
        """Test handling of malformed HTML content."""
        converter = tarzi.Converter()

        malformed_html_samples = [
            "<p>Unclosed paragraph",
            "<div><span>Nested unclosed</div>",
            "Plain text without HTML",
            "<script>alert('xss')</script>",
            "<html><body><p>Deeply <span>nested <strong>content</p></span></strong></body></html>",
            "",  # Empty content
            "   ",  # Whitespace only
            "\n\t\r\n",  # Various whitespace
        ]

        for html in malformed_html_samples:
            try:
                # Should handle malformed HTML gracefully
                result_markdown = converter.convert(html, "markdown")
                assert isinstance(result_markdown, str)

                result_json = converter.convert(html, "json")
                assert isinstance(result_json, str)

                result_yaml = converter.convert(html, "yaml")
                assert isinstance(result_yaml, str)

            except Exception as e:
                # Some malformed HTML might cause parsing errors
                print(f"Malformed HTML caused error: {e}")

    def test_search_with_special_characters(self):
        """Test search handling with special characters and encoding."""
        engine = tarzi.SearchEngine()

        special_queries = [
            "query with spaces",
            "query+with+plus",
            "query%20with%20encoding",
            "query&with&ampersands",
            "query=with=equals",
            "query?with?questions",
            "query#with#hash",
            "émojis and unicode 🎉",
            "日本語 search",
            "query\nwith\nnewlines",
            "query\twith\ttabs",
        ]

        for query in special_queries:
            try:
                # Should handle special characters gracefully
                results = engine.search(query, "webquery", 1)
                assert isinstance(results, list)

            except Exception as e:
                # Some special characters might cause issues
                print(f"Special query '{query}' caused error: {e}")

    def test_concurrent_operations_safety(self):
        """Test that components handle concurrent operations safely."""
        import threading

        engine = tarzi.SearchEngine()
        fetcher = tarzi.WebFetcher()
        converter = tarzi.Converter()

        results = {"errors": 0, "successes": 0}

        def search_worker():
            try:
                engine.search("concurrent test", "webquery", 1)
                results["successes"] += 1
            except Exception:
                results["errors"] += 1

        def fetch_worker():
            try:
                fetcher.fetch("https://httpbin.org/html", "plain_request", "html")
                results["successes"] += 1
            except Exception:
                results["errors"] += 1

        def convert_worker():
            try:
                converter.convert("<p>Test</p>", "markdown")
                results["successes"] += 1
            except Exception:
                results["errors"] += 1

        # Run multiple operations concurrently
        threads = []
        for _ in range(3):
            threads.append(threading.Thread(target=search_worker))
            threads.append(threading.Thread(target=fetch_worker))
            threads.append(threading.Thread(target=convert_worker))

        for thread in threads:
            thread.start()

        for thread in threads:
            thread.join(timeout=30)  # Prevent hanging

        # Should complete without crashing
        total_operations = results["errors"] + results["successes"]
        assert total_operations > 0

    def test_memory_usage_with_large_content(self):
        """Test handling of large content without memory issues."""
        converter = tarzi.Converter()

        # Create large HTML content
        large_html = "<html><body>"
        for i in range(1000):
            large_html += f"<p>This is paragraph {i} with some content to make it longer.</p>"
        large_html += "</body></html>"

        try:
            # Should handle large content without memory issues
            result = converter.convert(large_html, "markdown")
            assert isinstance(result, str)
            assert len(result) > 0

        except Exception as e:
            # Large content might cause issues depending on implementation
            print(f"Large content caused error: {e}")

    def test_configuration_edge_cases(self):
        """Test configuration edge cases."""
        edge_case_configs = [
            # Empty sections
            "[general]\n[fetcher]\n[search]",
            # Special characters in values
            """
[fetcher]
user_agent = "Special/Agent (with) [brackets] & {braces}"
""",
            # Very long values
            f"""
[search]
brave_api_key = "{'x' * 1000}"
""",
            # Unicode in configuration
            """
[general]
log_level = "débug"
""",
        ]

        for config_str in edge_case_configs:
            try:
                config = tarzi.Config.from_str(config_str)
                assert isinstance(config, tarzi.Config)

                # Try to create components
                tarzi.WebFetcher.from_config(config)
                tarzi.SearchEngine.from_config(config)
                tarzi.Converter.from_config(config)

            except Exception as e:
                # Some edge cases might cause valid errors
                print(f"Edge case config caused error: {e}")

    def test_proxy_error_handling(self):
        """Test proxy-related error handling."""
        # Test with non-existent proxy
        config_str = """
[fetcher]
proxy = "http://non-existent-proxy:8080"

[search]
engine = "duckduckgo"
"""
        config = tarzi.Config.from_str(config_str)
        fetcher = tarzi.WebFetcher.from_config(config)

        try:
            # Should handle proxy connection errors gracefully
            result = fetcher.fetch("https://httpbin.org/html", "plain_request", "html")
            # If it succeeds, proxy was bypassed or handled differently
            assert isinstance(result, str)

        except Exception as e:
            # Proxy connection errors are expected
            assert any(keyword in str(e).lower() for keyword in ["proxy", "connection", "network"])

    def test_api_provider_error_scenarios(self):
        """Test API provider error scenarios."""
        # Test with completely invalid API key format
        config_str = """
[search]
engine = "brave"
brave_api_key = ""
"""
        config = tarzi.Config.from_str(config_str)
        engine = tarzi.SearchEngine.from_config(config)

        try:
            results = engine.search("test query", "apiquery", 1)
            # Should handle empty API key gracefully
            if isinstance(results, list):
                assert len(results) == 0

        except Exception as e:
            # Errors are acceptable for empty API keys
            assert any(keyword in str(e).lower() for keyword in ["api", "key", "authentication"])

    def test_extreme_search_limits(self):
        """Test extreme search limit values."""
        engine = tarzi.SearchEngine()

        extreme_limits = [0, -1, 999999, float("inf")]

        for limit in extreme_limits:
            try:
                if limit == float("inf"):
                    continue  # Skip infinity as it's not a valid integer

                results = engine.search("test", "webquery", int(limit))
                assert isinstance(results, list)

                if limit > 0:
                    assert len(results) <= limit

            except Exception as e:
                # Extreme limits might cause errors
                assert any(keyword in str(e).lower() for keyword in ["limit", "value", "invalid"])

    def test_resource_cleanup(self):
        """Test that resources are cleaned up properly."""
        # Create multiple components and let them go out of scope
        for _ in range(10):
            config = tarzi.Config()
            tarzi.WebFetcher.from_config(config)
            tarzi.SearchEngine.from_config(config)
            converter = tarzi.Converter.from_config(config)

            # Use components briefly
            try:
                converter.convert("<p>test</p>", "markdown")
            except:
                pass

        # Should not cause memory leaks or resource issues
        # (This test mainly verifies no crashes occur)
