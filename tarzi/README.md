# Tarzi Library

This directory contains the core Tarzi library implementation in both Rust and Python.

## Structure

```
tarzi/
├── Cargo.toml          # Rust package configuration
├── pyproject.toml      # Python package configuration
├── README.md           # This file
├── Makefile            # Build and test commands
├── src/                # Rust source code
│   ├── lib.rs          # Library entry point
│   ├── main.rs         # Binary entry point
│   ├── config.rs       # Configuration management
│   ├── error.rs        # Error types
│   ├── constants.rs    # Constants and defaults
│   ├── utils.rs        # Utility functions
│   ├── converter.rs    # Data conversion utilities
│   ├── python.rs       # Python bindings
│   ├── search/         # Search engine implementations
│   └── fetcher/        # Web fetching implementations
└── python/             # Python package source
    └── tarzi/          # Python module
        ├── __init__.py # Module initialization
        └── __main__.py # CLI entry point
```

## Building

### Rust
```bash
# From tarzi subfolder
cargo build -p tarzi
cargo build --release -p tarzi
```

### Python
```bash
# From tarzi subfolder
maturin build --release
maturin develop --release
```

## Testing

### Rust
```bash
# From tarzi subfolder
cargo test -p tarzi
cargo test --test '*' --features test-helpers -p tarzi
```

### Python
```bash
# From tarzi subfolder
pytest tarzi/tests/python
```

## Development

This library is part of the main tarzi workspace. See the root README.md for complete development instructions. 