#!/usr/bin/env python3
"""
Unit tests for the Converter class in tarzi.
"""

import pytest

import tarzi


@pytest.fixture
def converter():
    """Fixture for creating a Converter instance."""
    return tarzi.Converter()


@pytest.fixture
def sample_html():
    """Fixture for sample HTML content."""
    return '<h1>Test Title</h1><p>Test <strong>content</strong> with <a href="https://example.com">link</a>.</p>'


@pytest.mark.unit
class TestConverter:
    """Test cases for the Converter class."""

    def test_converter_creation(self, converter):
        """Test Converter can be created."""
        assert isinstance(converter, tarzi.Converter)
        assert str(converter) == "Tarzi HTML/text content converter"
        assert repr(converter) == "Converter()"

    def test_html_conversion(self, converter, sample_html):
        """Test HTML format conversion (should return unchanged)."""
        result = converter.convert(sample_html, "html")
        assert result == sample_html

    def test_markdown_conversion(self, converter, sample_html):
        """Test HTML to Markdown conversion."""
        result = converter.convert(sample_html, "markdown")
        assert isinstance(result, str)
        assert len(result) > 0
        # Should contain markdown elements
        assert "Test Title" in result

    def test_json_conversion(self, converter, sample_html):
        """Test HTML to JSON conversion."""
        result = converter.convert(sample_html, "json")
        assert isinstance(result, str)
        assert len(result) > 0
        # Should contain JSON-like structure
        assert "Test Title" in result
        assert "content" in result  # More flexible check for content presence

    def test_yaml_conversion(self, converter, sample_html):
        """Test HTML to YAML conversion."""
        result = converter.convert(sample_html, "yaml")
        assert isinstance(result, str)
        assert len(result) > 0
        # Should contain YAML-like structure
        assert "Test Title" in result

    def test_invalid_format(self, converter, sample_html):
        """Test invalid format raises ValueError."""
        with pytest.raises(ValueError, match="Invalid format"):
            converter.convert(sample_html, "invalid_format")

    def test_empty_html(self, converter):
        """Test conversion with empty HTML."""
        result = converter.convert("", "html")
        assert result == ""

    def test_from_config(self):
        """Test creating Converter from config."""
        config = tarzi.Config()
        converter = tarzi.Converter.from_config(config)
        assert isinstance(converter, tarzi.Converter)


@pytest.mark.unit
def test_convert_html_function(sample_html):
    """Test convert_html standalone function."""
    result = tarzi.convert_html(sample_html, "html")
    assert result == sample_html

    result = tarzi.convert_html(sample_html, "markdown")
    assert isinstance(result, str)
    assert len(result) > 0


@pytest.mark.unit
def test_convert_html_invalid_format(sample_html):
    """Test convert_html with invalid format."""
    with pytest.raises(ValueError, match="Invalid format"):
        tarzi.convert_html(sample_html, "invalid_format")
