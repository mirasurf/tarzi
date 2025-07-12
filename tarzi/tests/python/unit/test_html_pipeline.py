#!/usr/bin/env python3
"""
Unit tests for HTML processing pipeline in tarzi.
"""

import pytest

import tarzi


@pytest.fixture
def converter():
    """Fixture for creating a Converter instance."""
    return tarzi.Converter()


@pytest.fixture
def sample_pipeline_html():
    """Fixture for HTML content used in pipeline tests."""
    return "<h1>Pipeline Test</h1><p>This is a <strong>test</strong> of the processing pipeline.</p>"


class TestHtmlProcessingPipeline:
    """Test a complete HTML processing pipeline."""

    def test_html_processing_pipeline(self, converter, sample_pipeline_html):
        """Test a complete HTML processing pipeline."""
        # Convert to different formats
        markdown = converter.convert(sample_pipeline_html, "markdown")
        assert isinstance(markdown, str)
        assert "Pipeline Test" in markdown

        json_result = converter.convert(sample_pipeline_html, "json")
        assert isinstance(json_result, str)
        assert "Pipeline Test" in json_result

        yaml_result = converter.convert(sample_pipeline_html, "yaml")
        assert isinstance(yaml_result, str)
        assert "Pipeline Test" in yaml_result

    def test_format_consistency(self, converter, sample_pipeline_html):
        """Test that all formats contain expected content."""
        formats = ["html", "markdown", "json", "yaml"]
        results = {}

        for fmt in formats:
            results[fmt] = converter.convert(sample_pipeline_html, fmt)
            assert isinstance(results[fmt], str)
            assert len(results[fmt]) > 0
            assert "Pipeline Test" in results[fmt]

        # HTML should be unchanged
        assert results["html"] == sample_pipeline_html

    def test_empty_content_pipeline(self, converter):
        """Test pipeline with empty content."""
        empty_html = ""
        formats = ["html", "markdown", "json", "yaml"]

        for fmt in formats:
            result = converter.convert(empty_html, fmt)
            assert isinstance(result, str)
            # Empty input should generally produce empty or minimal output
            if fmt == "html":
                assert result == ""
