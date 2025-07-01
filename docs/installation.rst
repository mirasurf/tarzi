Installation
============

tarzi is available as both a Python package and a Rust crate, with optional CLI tools.
Choose the installation method that best fits your use case.

Python Installation
-------------------

PyPI (Recommended)
~~~~~~~~~~~~~~~~~~

The easiest way to install tarzi for Python is via pip:

.. code-block:: bash

   pip install tarzi

This will install the pre-compiled Python wheel with all necessary dependencies.

From Source
~~~~~~~~~~~

If you need to build from source or want the latest development version:

.. code-block:: bash

   # Clone the repository
   git clone https://github.com/mirasurf/tarzi.rs.git
   cd tarzi.rs

   # Install Rust if not already installed
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env

   # Install maturin for building Python wheels
   pip install maturin

   # Build and install in development mode
   maturin develop --release

Virtual Environment (Recommended)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

It's recommended to use a virtual environment:

.. code-block:: bash

   # Using venv
   python -m venv tarzi-env
   source tarzi-env/bin/activate  # On Windows: tarzi-env\Scripts\activate
   pip install tarzi

   # Using conda
   conda create -n tarzi-env python=3.11
   conda activate tarzi-env
   pip install tarzi

Python Requirements
~~~~~~~~~~~~~~~~~~~

- Python 3.10 or higher
- Operating system: Linux, macOS, or Windows

Optional dependencies for development:

.. code-block:: bash

   # Install with development dependencies
   pip install tarzi[dev]

   # Install with test dependencies
   pip install tarzi[test]

Rust Installation
-----------------

As a Rust Crate
~~~~~~~~~~~~~~~~

Add tarzi to your `Cargo.toml`:

.. code-block:: toml

   [dependencies]
   tarzi = "0.0.4"
   tokio = { version = "1.0", features = ["full"] }

Or add it using cargo:

.. code-block:: bash

   cargo add tarzi

CLI Installation
~~~~~~~~~~~~~~~~

Install the command-line interface:

.. code-block:: bash

   cargo install tarzi

This will install the `tarzi` binary to your cargo bin directory.

From Source
~~~~~~~~~~~

.. code-block:: bash

   # Clone the repository
   git clone https://github.com/mirasurf/tarzi.rs.git
   cd tarzi.rs

   # Build the project
   cargo build --release

   # Install the CLI (optional)
   cargo install --path .

Rust Requirements
~~~~~~~~~~~~~~~~~

- Rust 1.70 or higher
- Cargo package manager

System Dependencies
-------------------

Browser Dependencies (Optional)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

For browser-based fetching, you'll need a WebDriver:

**Chrome/Chromium** (Recommended):

.. tabs::

   .. tab:: Linux

      .. code-block:: bash

         # Ubuntu/Debian
         sudo apt-get update
         sudo apt-get install -y chromium-browser

         # Download ChromeDriver
         wget https://chromedriver.storage.googleapis.com/LATEST_RELEASE_114/chromedriver_linux64.zip
         unzip chromedriver_linux64.zip
         sudo mv chromedriver /usr/local/bin/

   .. tab:: macOS

      .. code-block:: bash

         # Using Homebrew
         brew install chromium
         brew install chromedriver

   .. tab:: Windows

      .. code-block:: bash

         # Using Chocolatey
         choco install chromium
         choco install chromedriver

         # Or download manually from:
         # https://chromedriver.chromium.org/

**Firefox** (Alternative):

.. tabs::

   .. tab:: Linux

      .. code-block:: bash

         # Ubuntu/Debian
         sudo apt-get install firefox

         # Download GeckoDriver
         wget https://github.com/mozilla/geckodriver/releases/latest/download/geckodriver-linux64.tar.gz
         tar -xzf geckodriver-linux64.tar.gz
         sudo mv geckodriver /usr/local/bin/

   .. tab:: macOS

      .. code-block:: bash

         # Using Homebrew
         brew install firefox
         brew install geckodriver

   .. tab:: Windows

      .. code-block:: bash

         # Download from Mozilla
         # https://github.com/mozilla/geckodriver/releases

Proxy Configuration (Optional)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

If you're behind a corporate proxy, configure it in your `tarzi.toml`:

.. code-block:: toml

   [fetcher]
   proxy = "http://proxy.company.com:8080"

Or set environment variables:

.. code-block:: bash

   export HTTP_PROXY=http://proxy.company.com:8080
   export HTTPS_PROXY=http://proxy.company.com:8080

Verification
------------

Python Verification
~~~~~~~~~~~~~~~~~~~~

.. code-block:: bash

   python -c "import tarzi; print('tarzi version:', tarzi.__version__)"

Rust Verification
~~~~~~~~~~~~~~~~~

Create a simple test file `test.rs`:

.. code-block:: rust

   use tarzi::Converter;

   #[tokio::main]
   async fn main() -> Result<(), Box<dyn std::error::Error>> {
       let converter = Converter::new();
       let result = converter.convert("<h1>Test</h1>", tarzi::Format::Markdown).await?;
       println!("Converted: {}", result);
       Ok(())
   }

Run it:

.. code-block:: bash

   rustc test.rs && ./test

CLI Verification
~~~~~~~~~~~~~~~~

.. code-block:: bash

   tarzi --version
   tarzi --help

Docker Installation
-------------------

For containerized environments, use our official Docker image:

.. code-block:: bash

   # Pull the image
   docker pull ghcr.io/mirasurf/tarzi:latest

   # Run with Python
   docker run -it --rm ghcr.io/mirasurf/tarzi:latest python -c "import tarzi; print('Ready!')"

   # Run CLI
   docker run -it --rm ghcr.io/mirasurf/tarzi:latest tarzi --help

Dockerfile Example
~~~~~~~~~~~~~~~~~~

.. code-block:: dockerfile

   FROM python:3.11-slim

   # Install system dependencies
   RUN apt-get update && apt-get install -y \
       chromium \
       chromium-driver \
       && rm -rf /var/lib/apt/lists/*

   # Install tarzi
   RUN pip install tarzi

   # Your application
   COPY . /app
   WORKDIR /app

   CMD ["python", "your_app.py"]

Troubleshooting
---------------

Common Issues
~~~~~~~~~~~~~

**Import Error on Python**:

.. code-block:: bash

   # Reinstall with verbose output
   pip install --force-reinstall -v tarzi

**Browser Driver Not Found**:

.. code-block:: bash

   # Check if chromedriver is in PATH
   which chromedriver

   # Or specify the path in configuration
   export CHROMEDRIVER_PATH=/path/to/chromedriver

**Permission Denied on Linux**:

.. code-block:: bash

   # Make chromedriver executable
   chmod +x /usr/local/bin/chromedriver

**Proxy Issues**:

.. code-block:: bash

   # Test without proxy first
   unset HTTP_PROXY HTTPS_PROXY
   python -c "import tarzi; print('Success!')"

Getting Help
~~~~~~~~~~~~

If you encounter issues:

1. Search existing `GitHub issues <https://github.com/mirasurf/tarzi.rs/issues>`_
2. Create a new issue with detailed error information
3. Join our community discussions

Next Steps
----------

Now that you have tarzi installed, check out the :doc:`quickstart` guide to learn 
the basic usage patterns and start building your first application. 