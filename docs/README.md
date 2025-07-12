# Tarzi Documentation

This directory contains the documentation for the tarzi project, built with Sphinx.

## Building Locally

### Prerequisites

1. Install Python dependencies:
   ```bash
   pip install -r requirements.txt
   ```

2. Ensure you have the tarzi package available (either installed or in development mode):
   ```bash
   # From tarzi subfolder
   cd ../tarzi
   pip install -e .
   # or
   maturin develop --release
   ```

### Build Commands

```bash
# Build HTML documentation
make html

# Build all formats
make all

# Clean build directory
make clean

# Build and serve (with helpful message)
make serve

# Watch for changes and rebuild
make watch
```

### Viewing Documentation

After building, open `_build/html/index.html` in your web browser.

## Structure

- `index.rst` - Main documentation page
- `overview.rst` - Project overview and features
- `installation.rst` - Installation instructions
- `quickstart.rst` - Quick start guide
- `user_guide/` - Detailed usage documentation
- `python_api/` - Python API reference
- `rust_api/` - Rust API reference
- `examples/` - Code examples (located in tarzi/examples/)
- `configuration.rst` - Configuration options
- `development.rst` - Development guide

## ReadTheDocs Integration

The documentation is configured to work with ReadTheDocs. The `conf.py` file includes:

- Proper path configuration for imports
- Theme and extension settings
- Intersphinx mappings
- Custom CSS and static files

## Contributing

When updating documentation:

1. Make your changes to the `.rst` files
2. Build locally to check formatting: `make html`
3. Test that all links work correctly
4. Commit your changes

## Troubleshooting

### Import Errors

If you encounter import errors when building:

1. Ensure tarzi is installed in development mode (from the tarzi/ subfolder)
2. Check that the Python path is correctly set in `conf.py`
3. Verify that all dependencies are installed

### Build Errors

1. Clean the build directory: `make clean`
2. Reinstall dependencies: `pip install -r requirements.txt` 