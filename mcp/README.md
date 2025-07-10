# Tarzi MCP Server

A Model Context Protocol (MCP) server that exposes Tarzi's web search, content fetching, and HTML conversion capabilities as tools for LLM applications.

## Features

The Tarzi MCP Server provides the following tools and resources:

### Tools
- **search_web**: Search the web using various search engines
- **fetch_url**: Fetch content from web URLs with different formats
- **convert_html**: Convert HTML content to markdown, JSON, or YAML
- **search_and_fetch**: Combined search and content fetching

### Resources
- **tarzi://config**: Current Tarzi configuration
- **tarzi://status**: Server health status

## Installation

### Option 1: Python Package Installation

1. Install the package and dependencies:
```bash
cd mcp
pip install -e .
```

2. Run the server:
```bash
tarzi-mcp-server --host 0.0.0.0 --port 8000
```

### Option 2: Docker Installation

1. Build and run with Docker:
```bash
cd mcp
docker build -t tarzi-mcp-server .
docker run -p 8000:8000 tarzi-mcp-server
```

2. Or use docker-compose:
```bash
docker-compose up
```

### Option 3: Docker with Nginx Proxy

```bash
docker-compose --profile proxy up
```

## Usage

### Starting the Server

The server supports multiple transport modes:

#### HTTP Transport (Recommended for Production)
```bash
python -m tarzi_mcp_server.server --transport streamable-http --host 0.0.0.0 --port 8000
```

#### SSE Transport (Legacy)
```bash
python -m tarzi_mcp_server.server --transport sse --host 0.0.0.0 --port 8000
```

#### STDIO Transport (Development/Testing)
```bash
python -m tarzi_mcp_server.server --transport stdio
```

### Testing with the Client

Use the included test client to verify functionality:

```bash
# Test all functionality
python -m tarzi_mcp_server.client --test all

# Test specific functionality
python -m tarzi_mcp_server.client --test search --query "machine learning"
python -m tarzi_mcp_server.client --test fetch --url "https://example.com"
python -m tarzi_mcp_server.client --test convert
python -m tarzi_mcp_server.client --test resources
```

## Tool Reference

### search_web

Search the web using Tarzi search engines.

**Parameters:**
- `query` (string): Search query
- `limit` (integer, default: 10): Maximum results to return
- `mode` (string, default: "webquery"): Search mode ("webquery" or "apiquery")

**Returns:** List of search results with title, URL, snippet, and rank.

**Example:**
```json
{
  "query": "python programming",
  "limit": 3,
  "mode": "webquery"
}
```

### fetch_url

Fetch content from a web URL.

**Parameters:**
- `url` (string): URL to fetch
- `format` (string, default: "html"): Output format ("html", "markdown", "json", "yaml")
- `mode` (string, default: "plain_request"): Fetch mode ("plain_request" or "browser")

**Returns:** Fetched content in the specified format.

**Example:**
```json
{
  "url": "https://example.com",
  "format": "markdown",
  "mode": "plain_request"
}
```

### convert_html

Convert HTML content to other formats.

**Parameters:**
- `html_content` (string): HTML content to convert
- `output_format` (string, default: "markdown"): Output format ("markdown", "json", "yaml")

**Returns:** Converted content.

**Example:**
```json
{
  "html_content": "<h1>Hello</h1><p>World</p>",
  "output_format": "markdown"
}
```

### search_and_fetch

Search and fetch content from results.

**Parameters:**
- `query` (string): Search query
- `limit` (integer, default: 5): Maximum results to process
- `search_mode` (string, default: "webquery"): Search mode
- `fetch_mode` (string, default: "plain_request"): Fetch mode
- `content_format` (string, default: "markdown"): Content format

**Returns:** Search results with fetched content.

## Integration with Claude Desktop

To use this server with Claude Desktop, add the following configuration to your Claude Desktop config file:

**macOS:** `~/Library/Application Support/Claude/claude_desktop_config.json`
**Windows:** `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "tarzi": {
      "command": "python",
      "args": ["-m", "tarzi_mcp_server.server"],
      "env": {}
    }
  }
}
```

For HTTP transport:
```json
{
  "mcpServers": {
    "tarzi": {
      "url": "http://localhost:8000",
      "transport": "streamable-http"
    }
  }
}
```

## Configuration

The server uses Tarzi's default configuration. You can customize behavior by setting environment variables or creating a `tarzi.toml` configuration file.

Example `tarzi.toml`:
```toml
[fetcher]
timeout = 30
user_agent = "Tarzi MCP Server/1.0"

[search]
engine = "bing"
```

## Docker Environment Variables

- `PYTHONUNBUFFERED=1`: Disable Python output buffering
- `TARZI_TIMEOUT`: Override default timeout
- `TARZI_USER_AGENT`: Override default user agent

## Development

### Running Tests

```bash
# Install development dependencies
pip install -e ".[dev]"

# Run tests
pytest

# Run specific tests
python -m tarzi_mcp_server.client --test all
```

### Building Docker Image

```bash
docker build -t tarzi-mcp-server .
```

### Pushing to Registry

```bash
docker tag tarzi-mcp-server your-registry/tarzi-mcp-server:latest
docker push your-registry/tarzi-mcp-server:latest
```

## Troubleshooting

### Common Issues

1. **Import Error**: Ensure Tarzi is installed: `pip install tarzi`
2. **Connection Failed**: Check that the server is running and accessible
3. **Search Failures**: Some search engines may require API keys or have rate limits
4. **Browser Mode Errors**: Browser automation requires additional system dependencies

### Logs

The server logs to stdout by default. In Docker, access logs with:
```bash
docker logs container_name
```

For docker-compose:
```bash
docker-compose logs tarzi-mcp-server
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

This project follows the same license as the Tarzi project (Apache 2.0).

## Support

For issues related to:
- **MCP functionality**: Check the MCP documentation
- **Tarzi functionality**: Check the Tarzi repository
- **This server**: Open an issue in this repository