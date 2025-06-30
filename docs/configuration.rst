Configuration
=============

tarzi can be configured through configuration files, environment variables, and programmatic configuration.

Configuration File
------------------

Create a ``tarzi.toml`` file in your project root:

.. code-block:: toml

   # General settings
   [general]
   log_level = "info"
   timeout = 30

   # Web fetcher configuration
   [fetcher]
   mode = "browser_headless"
   format = "markdown"
   user_agent = "tarzi/0.0.4"
   proxy = ""
   timeout = 30

   # Search engine configuration
   [search]
   engine = "bing"
   mode = "webquery"
   limit = 10
   api_key = ""

Configuration Sections
----------------------

General Section
~~~~~~~~~~~~~~~

.. code-block:: toml

   [general]
   log_level = "info"    # trace, debug, info, warn, error
   timeout = 30          # Default timeout in seconds

Fetcher Section
~~~~~~~~~~~~~~~

.. code-block:: toml

   [fetcher]
   mode = "browser_headless"           # plain_request, browser_headless, browser_head
   format = "markdown"                 # markdown, json, yaml, html
   user_agent = "tarzi/0.0.4"         # Custom user agent
   proxy = "http://proxy.example.com:8080"  # Proxy URL
   timeout = 30                        # Request timeout

Search Section
~~~~~~~~~~~~~~

.. code-block:: toml

   [search]
   engine = "bing"       # bing, google, duckduckgo, brave, tavily
   mode = "webquery"     # webquery, apiquery
   limit = 10            # Maximum results per search
   api_key = ""          # API key for apiquery mode

Environment Variables
---------------------

Currently supported environment variables:

.. code-block:: bash

   # WebDriver configuration
   export TARZI_WEBDRIVER_URL=http://localhost:9515

   # Proxy configuration (standard environment variables)
   export HTTP_PROXY=http://proxy.example.com:8080
   export HTTPS_PROXY=http://proxy.example.com:8080

   # Debug mode (for development/testing)
   export TARZI_DEBUG=1

**Note**: Most configuration options must be set via the TOML configuration file or programmatically. 
Environment variable support for other settings is planned for future releases.

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

**Note**: Environment variables currently only override proxy settings and WebDriver URL. 
All other configuration must be set via TOML file or programmatically.

Search Engine Specific Configuration
------------------------------------

Bing Search
~~~~~~~~~~~

.. code-block:: toml

   [search]
   engine = "bing"
   query_pattern = "https://www.bing.com/search?q={query}"

Google Search
~~~~~~~~~~~~~

.. code-block:: toml

   [search]
   engine = "google"
   query_pattern = "https://www.google.com/search?q={query}"
   # For API mode:
   api_key = "your-google-api-key"

DuckDuckGo
~~~~~~~~~~

.. code-block:: toml

   [search]
   engine = "duckduckgo"
   query_pattern = "https://duckduckgo.com/?q={query}"

Custom Search Engine
~~~~~~~~~~~~~~~~~~~~

.. code-block:: toml

   [search]
   engine = "custom"
   query_pattern = "https://search.example.com/search?q={query}"

Proxy Configuration
-------------------

HTTP/HTTPS Proxy
~~~~~~~~~~~~~~~~~

.. code-block:: toml

   [fetcher]
   proxy = "http://proxy.example.com:8080"

SOCKS Proxy
~~~~~~~~~~~

.. code-block:: toml

   [fetcher]
   proxy = "socks5://proxy.example.com:1080"

Authenticated Proxy
~~~~~~~~~~~~~~~~~~~

.. code-block:: toml

   [fetcher]
   proxy = "http://username:password@proxy.example.com:8080"

Browser Configuration
---------------------

WebDriver URL
~~~~~~~~~~~~~

You can specify a custom WebDriver URL using either configuration or environment variable:

.. code-block:: bash

   # Environment variable (highest priority)
   export TARZI_WEBDRIVER_URL=http://localhost:9515

.. code-block:: toml

   # Configuration file
   [fetcher]
   web_driver_port = 9515

Chrome/Chromium Options
~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: toml

   [fetcher.browser]
   executable_path = "/usr/bin/chromium"
   args = ["--no-sandbox", "--disable-dev-shm-usage"]
   headless = true

Firefox Options
~~~~~~~~~~~~~~~

.. code-block:: toml

   [fetcher.browser]
   executable_path = "/usr/bin/firefox"
   driver_path = "/usr/local/bin/geckodriver" 