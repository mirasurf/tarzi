#!/usr/bin/env python3
"""Tarzi MCP Server - Exposes Tarzi search and web functionality via MCP tools."""

import asyncio
import json
import logging
from typing import Any, Dict, List, Optional

from mcp.server.fastmcp import FastMCP
from pydantic import BaseModel, Field

try:
    import tarzi
except ImportError:
    raise ImportError("tarzi library is required. Install with: pip install tarzi")

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

# Create the MCP server
mcp = FastMCP(
    "Tarzi Search Server",
    description="MCP server providing web search, content fetching, and HTML conversion tools using Tarzi"
)


class SearchResult(BaseModel):
    """Structured search result."""
    title: str = Field(description="Page title")
    url: str = Field(description="Page URL")
    snippet: str = Field(description="Page snippet/description")
    rank: int = Field(description="Search result ranking")


class FetchResult(BaseModel):
    """Structured fetch result."""
    content: str = Field(description="Fetched content")
    format: str = Field(description="Content format (html, markdown, json, yaml)")


class ConversionResult(BaseModel):
    """Structured conversion result."""
    converted_content: str = Field(description="Converted content")
    format: str = Field(description="Output format")


@mcp.tool()
def search_web(
    query: str, 
    limit: int = 10, 
    mode: str = "webquery"
) -> List[SearchResult]:
    """
    Search the web using Tarzi search engines.
    
    Args:
        query: Search query string
        limit: Maximum number of results to return (default: 10)
        mode: Search mode - 'webquery' for browser-based search, 'apiquery' for API-based search
        
    Returns:
        List of search results with title, URL, snippet, and rank
    """
    try:
        # Validate mode
        if mode not in ["webquery", "apiquery"]:
            raise ValueError("Mode must be 'webquery' or 'apiquery'")
            
        # Perform search using tarzi
        results = tarzi.search_web(query, mode, limit)
        
        # Convert to structured results
        structured_results = []
        for result in results:
            structured_results.append(SearchResult(
                title=result.title,
                url=result.url,
                snippet=result.snippet,
                rank=result.rank
            ))
            
        logger.info(f"Search completed: {len(structured_results)} results for query '{query}'")
        return structured_results
        
    except Exception as e:
        logger.error(f"Search failed: {str(e)}")
        raise ValueError(f"Search failed: {str(e)}")


@mcp.tool()
def fetch_url(
    url: str, 
    format: str = "html", 
    mode: str = "plain_request"
) -> FetchResult:
    """
    Fetch content from a web URL using Tarzi.
    
    Args:
        url: URL to fetch
        format: Output format - 'html', 'markdown', 'json', or 'yaml'
        mode: Fetch mode - 'plain_request' for simple HTTP, 'browser' for browser automation
        
    Returns:
        Fetched content in the specified format
    """
    try:
        # Validate format
        if format not in ["html", "markdown", "json", "yaml"]:
            raise ValueError("Format must be 'html', 'markdown', 'json', or 'yaml'")
            
        # Validate mode
        if mode not in ["plain_request", "browser"]:
            raise ValueError("Mode must be 'plain_request' or 'browser'")
            
        # Fetch content using tarzi
        content = tarzi.fetch_url(url, mode, format)
        
        logger.info(f"URL fetched successfully: {url} in {format} format")
        return FetchResult(content=content, format=format)
        
    except Exception as e:
        logger.error(f"URL fetch failed: {str(e)}")
        raise ValueError(f"URL fetch failed: {str(e)}")


@mcp.tool()
def convert_html(html_content: str, output_format: str = "markdown") -> ConversionResult:
    """
    Convert HTML content to various formats using Tarzi converter.
    
    Args:
        html_content: HTML content to convert
        output_format: Output format - 'markdown', 'json', or 'yaml'
        
    Returns:
        Converted content in the specified format
    """
    try:
        # Validate format
        if output_format not in ["markdown", "json", "yaml"]:
            raise ValueError("Output format must be 'markdown', 'json', or 'yaml'")
            
        # Convert using tarzi
        converted = tarzi.convert_html(html_content, output_format)
        
        logger.info(f"HTML converted successfully to {output_format}")
        return ConversionResult(converted_content=converted, format=output_format)
        
    except Exception as e:
        logger.error(f"HTML conversion failed: {str(e)}")
        raise ValueError(f"HTML conversion failed: {str(e)}")


@mcp.tool()
def search_and_fetch(
    query: str,
    limit: int = 5,
    search_mode: str = "webquery",
    fetch_mode: str = "plain_request",
    content_format: str = "markdown"
) -> List[Dict[str, Any]]:
    """
    Search the web and fetch content from each result using Tarzi.
    
    Args:
        query: Search query string
        limit: Maximum number of results to process (default: 5)
        search_mode: Search mode - 'webquery' or 'apiquery'
        fetch_mode: Fetch mode - 'plain_request' or 'browser'
        content_format: Content format - 'html', 'markdown', 'json', or 'yaml'
        
    Returns:
        List of search results with fetched content
    """
    try:
        # Validate parameters
        if search_mode not in ["webquery", "apiquery"]:
            raise ValueError("Search mode must be 'webquery' or 'apiquery'")
        if fetch_mode not in ["plain_request", "browser"]:
            raise ValueError("Fetch mode must be 'plain_request' or 'browser'")
        if content_format not in ["html", "markdown", "json", "yaml"]:
            raise ValueError("Content format must be 'html', 'markdown', 'json', or 'yaml'")
            
        # Perform search and fetch using tarzi
        results_with_content = tarzi.search_and_fetch(
            query, search_mode, limit, fetch_mode, content_format
        )
        
        # Convert to structured format
        structured_results = []
        for search_result, content in results_with_content:
            structured_results.append({
                "search_result": {
                    "title": search_result.title,
                    "url": search_result.url,
                    "snippet": search_result.snippet,
                    "rank": search_result.rank
                },
                "content": content,
                "content_format": content_format
            })
            
        logger.info(f"Search and fetch completed: {len(structured_results)} results for query '{query}'")
        return structured_results
        
    except Exception as e:
        logger.error(f"Search and fetch failed: {str(e)}")
        raise ValueError(f"Search and fetch failed: {str(e)}")


@mcp.resource("tarzi://config")
def get_config() -> str:
    """Get current Tarzi configuration."""
    try:
        # Try to get default config
        config = tarzi.Config()
        return f"""Tarzi Configuration:
- Version: {tarzi.__version__ if hasattr(tarzi, '__version__') else 'unknown'}
- Default timeout: 30s
- Default user agent: Tarzi Search Client
- Available modes: webquery, apiquery, plain_request, browser
- Supported formats: html, markdown, json, yaml
"""
    except Exception as e:
        return f"Error getting config: {str(e)}"


@mcp.resource("tarzi://status")
def get_status() -> str:
    """Get Tarzi service status."""
    try:
        # Basic health check - try to create components
        converter = tarzi.Converter()
        fetcher = tarzi.WebFetcher()
        search_engine = tarzi.SearchEngine()
        
        return """Tarzi MCP Server Status: HEALTHY
- Converter: Available
- WebFetcher: Available  
- SearchEngine: Available
- MCP Server: Running
"""
    except Exception as e:
        return f"Tarzi MCP Server Status: ERROR - {str(e)}"


def main():
    """Main entry point for the server."""
    import argparse
    
    parser = argparse.ArgumentParser(description="Tarzi MCP Server")
    parser.add_argument("--host", default="127.0.0.1", help="Host to bind to")
    parser.add_argument("--port", type=int, default=8000, help="Port to bind to")
    parser.add_argument("--transport", default="streamable-http", choices=["stdio", "sse", "streamable-http"], 
                       help="Transport type")
    
    args = parser.parse_args()
    
    logger.info(f"Starting Tarzi MCP Server on {args.host}:{args.port} with {args.transport} transport")
    
    if args.transport == "stdio":
        # For stdio transport (development/testing)
        mcp.run(transport="stdio")
    elif args.transport == "sse":
        # For SSE transport (legacy)
        mcp.run(transport="sse", host=args.host, port=args.port)
    else:
        # For HTTP transport (production)
        mcp.run(transport="streamable-http", host=args.host, port=args.port)


if __name__ == "__main__":
    main()