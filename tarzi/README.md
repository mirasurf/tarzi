# Tarzi Core Library

This directory contains the core library implementations for the tarzi project.

## Structure

```
tarzi/
├── rust/          # Rust implementation
│   └── src/       # Rust source code
└── python/        # Python bindings
    └── tarzi/     # Python package
```

## Components

### Rust Implementation (`rust/`)

Contains the core Rust library with all the main functionality:

- **Search Engine**: Web search capabilities
- **Fetcher**: Web content fetching with browser automation
- **Converter**: Format conversion utilities
- **Configuration**: Configuration management
- **Error Handling**: Comprehensive error types

### Python Bindings (`python/`)

Python bindings for the Rust library using PyO3:

- **Python Package**: Exposes Rust functionality to Python
- **Type Bindings**: Python-friendly interfaces
- **Async Support**: Asynchronous operations

## Development

Each subdirectory contains its own specific implementation details and can be developed independently while maintaining interface compatibility.

## Building

The build process coordinates between both Rust and Python components:

1. Rust library is built first
2. Python bindings are generated using PyO3
3. Final Python package includes the compiled Rust library

For detailed build instructions, see the main project README.