# tarsier
Rust-native lite search for AI applications.

# Features

## Goals 0

- [ ] Provide both a native Rust implementation and a Python wrapper, available as a library and a CLI tool.  
- [ ] Convert raw HTML strings into semi-structured formats such as Markdown, JSON, or YAML.  
- [ ] Fetch individual web pages from URLs, with optional JavaScript rendering support.  
- [ ] Perform search engine queries using either browser mode (headless or headed, no token required) or API mode (token-based).  
- [ ] Support proxy usage in both browser-based and API-based modes.  
- [ ] Implement an end-to-end pipeline for querying search engines, parsing SERPs, and extracting target pages for AI applications.

## Goals 1

- [ ] Capture screenshots of web pages.  
- [ ] Define and execute custom workflows for interacting with web pages and crawling structured content.

# Dependencies

* Rust edition 2024
* chromiumoxide: support chrome browser instance
* HTML → Markdown: html2md
* Markdown → JSON: use pulldown-cmark to produce JSON
* JSON → YAML: serde_yaml
* mistral.rs: run local LLM
