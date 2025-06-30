Python API Reference
====================

Complete reference for the tarzi Python API.

.. toctree::
   :maxdepth: 2
   :caption: Python API:

   converter
   fetcher
   search
   config
   errors

Quick Reference
---------------

**Core Functions**
   - :func:`tarzi.convert_html` - Convert HTML to various formats
   - :func:`tarzi.fetch_url` - Fetch web page content
   - :func:`tarzi.search_web` - Search the web

**Classes**
   - :class:`tarzi.Converter` - HTML conversion
   - :class:`tarzi.WebFetcher` - Web page fetching
   - :class:`tarzi.SearchEngine` - Web search
   - :class:`tarzi.Config` - Configuration management

**Data Types**
   - :class:`tarzi.SearchResult` - Search result data
   - :class:`tarzi.TarziError` - Error types

Installation
------------

.. code-block:: bash

   pip install tarzi

Basic Usage
-----------

.. code-block:: python

   import tarzi

   # Convert HTML
   markdown = tarzi.convert_html("<h1>Hello</h1>", "markdown")

   # Fetch web page
   content = tarzi.fetch_url("https://example.com")

   # Search web
   results = tarzi.search_web("python programming", "webquery", 10) 