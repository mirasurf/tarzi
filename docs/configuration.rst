Configuration
=============

tarzi can be configured through configuration files, environment variables, and programmatic configuration.

.. note::
   tarzi supports only Linux and macOS. Windows is not supported.

Configuration File
------------------

tarzi reads configuration from the following sources in order:

1. `.tarzi.toml` under user home directory
2. `tarzi.toml` in current project root

You can refer to `tarzi.toml <https://github.com/mirasurf/tarzi.rs/blob/main/tarzi.toml>`_ for the default values.

Environment Variables
---------------------

Currently supported environment variables:

.. code-block:: bash

   # Proxy configuration (standard environment variables)
   export http_proxy=http://proxy.example.com:8080
   export https_proxy=http://proxy.example.com:8080

   # Debug mode (for development/testing)
   export TARZI_DEBUG=1

Programmatic Configuration
--------------------------

Python
~~~~~~

.. code-block:: python

   import tarzi

   # Load from file
   config = tarzi.Config.from_file("tarzi.toml")

   # Create from string
   config_str = """
   [fetcher]
   timeout = 60
   format = "json"
   """
   config = tarzi.Config.from_str(config_str)

   # Use with components
   fetcher = tarzi.WebFetcher.from_config(config)
   search_engine = tarzi.SearchEngine.from_config(config)

Rust
~~~~

.. code-block:: rust

   use tarzi::{Config, WebFetcher, SearchEngine};

   // Load from file
   let config = Config::from_file("tarzi.toml")?;

   // Create programmatically
   let mut config = Config::default();
   config.fetcher.timeout = 60;
   config.fetcher.format = Format::Json;

   // Use with components
   let fetcher = WebFetcher::from_config(&config);
   let search_engine = SearchEngine::from_config(&config);

Configuration Precedence
-------------------------

Configuration values are applied in the following order (highest to lowest priority):

1. Programmatic configuration
2. Environment variables (limited support - see above)
3. Configuration file
4. Default values

**Note**: Environment variables currently only override proxy settings and API keys. 
All other configuration must be set via TOML file or programmatically.

API Search Configuration
------------------------

tarzi supports multiple API search providers with automatic fallback capabilities:

**Supported Providers:**
- **Brave Search API**: Fast, privacy-focused search results
- **Google Serper API**: Google search results via official API
- **Exa Search API**: AI-powered semantic search with enhanced relevance
- **Travily API**: Specialized travel and location-based search
- **DuckDuckGo API**: Privacy-focused search (limited functionality, no API key required)

**Autoswitch Strategies:**
- **smart**: Automatically fallback to available providers if primary fails
- **none**: Only use the configured primary search engine

**Configuration Example:**

.. code-block:: toml

   [search]
   engine = "brave"
   mode = "apiquery"
   autoswitch = "smart"
   limit = 10
   
   # API keys for different providers
   brave_api_key = "your-brave-api-key"
   google_serper_api_key = "your-google-serper-api-key"
   exa_api_key = "your-exa-api-key"
   travily_api_key = "your-travily-api-key"
