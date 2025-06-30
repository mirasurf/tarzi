# Tarzi Python Wrapper - Quick Start

Fast setup guide for building and using the tarzi Python extension.

## Status ✅

✅ **Core Rust Library**: Working (68/68 tests pass)  
✅ **Python Bindings**: Complete with enhanced features  
✅ **PyO3 Build**: Fixed and working  

## Quick Setup

### 1. Build the Python Extension

```bash
# Get your Python library info
python3 -c "import sysconfig; print('Library:', sysconfig.get_config_var('LIBDIR')); print('Python lib:', sysconfig.get_config_var('LDLIBRARY'))"

# Set environment variables (adjust paths for your system)
export RUSTFLAGS="-L/Users/xiamingchen/.pyenv/versions/3.11.10/lib -lpython3.11"
export PYO3_PYTHON=/Users/xiamingchen/.pyenv/versions/3.11.10/envs/tarzi/bin/python3

# Build
cargo clean
cargo build --features pyo3
```

### 2. Test the Module

```bash
# Test import
python3 -c "import tarzi; print('✅ Success!')"

# Test basic functionality
python3 -c "
import tarzi
converter = tarzi.PyConverter()
result = converter.convert('<h1>Test</h1>', 'markdown')
print('Result:', result)
"
```

## Quick Usage Examples

### Basic HTML Conversion
```python
import tarzi

# Create converter
converter = tarzi.PyConverter()

# Convert HTML to markdown
html = '<h1>Hello</h1><p>World!</p>'
markdown = converter.convert(html, 'markdown')
print(markdown)  # # Hello\n\nWorld!
```

### Web Fetching
```python
import tarzi

# Create web fetcher
fetcher = tarzi.PyWebFetcher()

# Fetch and convert a webpage
content = fetcher.fetch('https://example.com', 'plain_request', 'markdown')
print(content)
```

### Web Search
```python
import tarzi

# Create search engine
engine = tarzi.PySearchEngine()

# Search the web
results = engine.search('python programming', 'webquery', 5)
for result in results:
    print(f"{result.title}: {result.url}")
```

## Available Classes and Functions

### Classes
- `PyConverter()` - HTML/text conversion
- `PyWebFetcher()` - Web page fetching  
- `PySearchEngine()` - Web search functionality
- `PyConfig()` - Configuration management

### Standalone Functions
- `convert_html(html, format)` - Quick HTML conversion
- `fetch_url(url, mode, format)` - Quick URL fetching
- `search_web(query, mode, limit)` - Quick web search

### Supported Formats
- `html` - Raw HTML
- `markdown` - Markdown text
- `json` - JSON structure
- `yaml` - YAML format

### Fetch Modes
- `plain_request` - Simple HTTP request
- `browser_head` - Browser with head (faster)
- `browser_headless` - Full headless browser

## Development Commands

```bash
# Run Rust tests
cargo test --features "default"

# Test Python bindings
cargo test --features pyo3

# Build release wheel
maturin build --features pyo3 --release

# Run examples
python3 examples/basic_usage.py
python3 examples/search_engines.py
```

## Troubleshooting

### Build Issues
If you get linking errors, adjust the paths in RUSTFLAGS:
```bash
# For Homebrew Python
export RUSTFLAGS="-L/opt/homebrew/lib -lpython3.11"
export PYO3_PYTHON=/opt/homebrew/bin/python3.11

# For system Python
export RUSTFLAGS="-L/usr/lib -lpython3.11"
export PYO3_PYTHON=/usr/bin/python3
```

### Import Issues
```bash
# Check if module was built
ls target/debug/deps/libtarzi.dylib

# Check Python can find it
python3 -c "import sys; print(sys.path)"
```

**Ready to use!** 🚀 