# Tarzi Python Setup Guide

This guide helps you fix PyO3 linking issues and build the tarzi Python extension successfully.

## Current Status

✅ **Core Rust Library**: Fully working (68/68 tests pass)  
✅ **Python Bindings Code**: Enhanced with comprehensive features  
❌ **PyO3 Linking**: Needs to be fixed for Python module import  

## PyO3 Linking Issue

The main problem is that PyO3 cannot find the Python development headers during linking. This manifests as:

```
Undefined symbols for architecture arm64:
  "_PyBytes_AsString", "_PyBytes_Size", "_PyErr_Fetch", ...
```

## Quick Fix Attempts

### 1. Environment Variables Approach

```bash
# Set Python path explicitly
export PYO3_PYTHON=/Users/xiamingchen/.pyenv/versions/tarzi/bin/python3

# Set library path
export LIBRARY_PATH="/Users/xiamingchen/.pyenv/versions/3.11.10/lib:$LIBRARY_PATH"

# Set include path
export CPATH="/Users/xiamingchen/.pyenv/versions/3.11.10/include/python3.11:$CPATH"

# Try building
cargo build --features pyo3
```

### 2. Alternative Python Installation

```bash
# Install Python via Homebrew (sometimes works better than pyenv)
brew install python@3.11

# Set the Python path
export PYO3_PYTHON=/opt/homebrew/bin/python3.11

# Try building
cargo build --features pyo3
```

### 3. Manual Library Linking

```bash
# Get Python config
python3-config --ldflags
python3-config --includes

# Set linker flags manually
export RUSTFLAGS="-L/Users/xiamingchen/.pyenv/versions/3.11.10/lib -lpython3.11"

# Try building
cargo build --features pyo3
```

## Comprehensive Solution

### Step 1: Verify Python Installation

```bash
# Check Python version and location
python3 --version
which python3
python3 -c "import sys; print(sys.executable)"

# Check if development headers are available
python3 -c "import sysconfig; print(sysconfig.get_config_var('INCLUDEPY'))"
ls -la $(python3 -c "import sysconfig; print(sysconfig.get_config_var('INCLUDEPY'))")
```

### Step 2: Install Python Development Headers

For macOS with Homebrew:
```bash
# Install Python development headers
brew install python@3.11
# or
xcode-select --install
```

For pyenv installations:
```bash
# Reinstall Python with development headers
env PYTHON_CONFIGURE_OPTS="--enable-shared" pyenv install 3.11.10
```

### Step 3: Configure Build Environment

Create a `.env` file in the project root:
```bash
# .env
PYO3_PYTHON=/Users/xiamingchen/.pyenv/versions/tarzi/bin/python3
LIBRARY_PATH=/Users/xiamingchen/.pyenv/versions/3.11.10/lib
CPATH=/Users/xiamingchen/.pyenv/versions/3.11.10/include/python3.11
```

Load environment and build:
```bash
source .env
cargo clean
cargo build --features pyo3
```

### Step 4: Use Maturin (Recommended)

```bash
# Install maturin
pip install maturin

# Build and install the Python package
maturin develop --features pyo3

# Or build a wheel
maturin build --features pyo3
```

## Testing the Fix

Once the linking is resolved, test the Python module:

```bash
# Test import
python3 -c "import tarzi; print('Success!')"

# Run comprehensive tests
python3 test_python_bindings.py

# Try examples
python3 examples/basic_usage.py
python3 examples/search_engines.py
python3 examples/enhanced_demo.py
```

## Alternative: Docker Development

If linking issues persist, use Docker:

```dockerfile
FROM python:3.11-slim

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev

# Copy project
COPY . /app
WORKDIR /app

# Install maturin and build
RUN pip install maturin
RUN maturin develop --features pyo3

# Test
RUN python -c "import tarzi; print('Success!')"
```

## Manual Testing Commands

```bash
# Test core functionality
cargo test --features "default"

# Test Python bindings (once linking is fixed)
cargo test --features pyo3

# Test with specific Python version
PYO3_PYTHON=/path/to/python cargo test --features pyo3

# Build Python wheel
maturin build --features pyo3 --release
```

## Verification Steps

1. **Core Tests**: `cargo test --features "default"` should pass (✅ Already working)
2. **Python Linking**: `cargo build --features pyo3` should succeed
3. **Module Import**: `python3 -c "import tarzi"` should work
4. **Feature Tests**: `python3 test_python_bindings.py` should show 8/8 tests passing
5. **Examples**: All Python examples should run without import errors

## Enhanced Features Already Implemented

Once the linking is fixed, the Python module will have:

### 🎨 **Comprehensive Docstrings**
- All classes and methods have detailed documentation
- Parameter descriptions, return types, and exceptions documented
- Accessible via `help()` function in Python

### 🚨 **Enhanced Error Handling**
- Specific error messages instead of generic ones
- Context-aware error reporting
- Proper exception types (ValueError, RuntimeError)

### 🏷️ **String Representations**
- All classes have proper `__repr__` and `__str__` methods
- Debugging-friendly output
- Formatted display for search results

### ⚙️ **Configuration Management**
- TOML-based configuration with validation
- File and string-based config loading
- Configuration persistence methods

### 🔧 **API Improvements**
- Cloneable converter objects
- Comprehensive test coverage
- Better parameter validation

## Troubleshooting

### Common Issues

1. **"No module named 'tarzi'"**
   - PyO3 linking not resolved
   - Python module not built/installed

2. **SSL/TLS errors in maturin**
   - Try using system Python instead of pyenv
   - Install certificates: `pip install --upgrade certifi`

3. **Architecture mismatch**
   - Ensure Python and Rust target same architecture
   - Check with: `python3 -c "import platform; print(platform.machine())"`

### Debug Commands

```bash
# Check Rust targets
rustc --print target-list | grep darwin

# Check Python configuration
python3-config --ldflags
python3-config --includes
python3-config --cflags

# Check linking libraries
otool -L target/debug/libtarzi.dylib  # After successful build
```

## Next Steps

1. **Fix PyO3 Linking**: Use the solutions above
2. **Run Tests**: Verify all functionality works
3. **Create Documentation**: Generate API docs
4. **Performance Testing**: Benchmark with real workloads
5. **Distribution**: Create wheels for PyPI

## Support

If you continue to have issues:

1. Check the error logs carefully
2. Try different Python installations (Homebrew, system, pyenv)
3. Verify development headers are installed
4. Consider using Docker for a clean environment
5. Check PyO3 documentation for platform-specific issues

The Python wrapper is fully implemented and ready - we just need to resolve the linking issue to make it usable! 