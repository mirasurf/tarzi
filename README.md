<div align="center">
  <img src="static/tarzi-320.png" alt="Tarzi Logo" width="200" height="200">
</div>

<h1 align="center">tarzi.rs</h1>

<div align="center">
  Rust-native lite search for your AI applications.
</div>

<p align="center">
  <!-- Rust crate: version and download count -->
  <a href="https://crates.io/crates/tarzi">
    <img src="https://img.shields.io/crates/v/tarzi.svg?style=flat-square" alt="Crate Version" />
  </a>
  <a href="https://crates.io/crates/tarzi">
    <img src="https://img.shields.io/crates/d/tarzi.svg?style=flat-square" alt="Crate Downloads" />
  </a>
  <a href="https://github.com/mirasurf/tarzi.rs/actions/workflows/rust-ci.yml">
    <img src="https://github.com/mirasurf/tarzi.rs/actions/workflows/rust-ci.yml/badge.svg" alt="Rust CI" />
  </a>
  <!-- PyPI package: version and monthly downloads -->
  <a href="https://pypi.org/project/tarzi/">
    <img src="https://img.shields.io/pypi/v/tarzi.svg?style=flat-square" alt="PyPI Version" />
  </a>
  <a href="https://pypistats.org/packages/tarzi">
    <img src="https://img.shields.io/pypi/dm/tarzi.svg?style=flat-square" alt="PyPI Downloads" />
  </a>
  <a href="https://github.com/mirasurf/tarzi.rs/actions/workflows/python-ci.yml">
    <img src="https://github.com/mirasurf/tarzi.rs/actions/workflows/python-ci.yml/badge.svg" alt="Python CI" />
  </a>
  <!-- License -->
  <a href="https://www.apache.org/licenses/LICENSE-2.0">
    <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg?style=flat-square" alt="License" />
  </a>
  <!-- X (formerly Twitter) -->
  <a href="https://x.com/mirasurf_ai">
    <img src="https://img.shields.io/twitter/follow/mirasurf_ai?label=@mirasurf_ai&style=flat-square" alt="X Follow" />
  </a>
</p>

<div align="center">
  <img src="static/tariz-workflow.png" alt="Tarzi Logo" width="100%">
</div>

**Tarzi** 🐒 is a unified search interface designed for **Retrieval-Augmented Generation (RAG)** and **agentic systems** built on large language models. Search is a core functionality in these systems, yet most search engine providers impose API paywalls or strict rate limits—even for light or research-driven usage.

**Tarzi** 🐒 removes these barriers by supporting both token-based APIs and free web queries across multiple search engines. With a single dependency, you can integrate and switch between different Search Engine Providers (SEPs) as needed—seamlessly and efficiently.


## ⚙️ Core Capabilities

- 🦀 **Dual Implementation**: Native Rust library and Python wrapper with CLI tools  
- 🔄 **Content Conversion**: Convert raw HTML into Markdown, JSON, or YAML  
- 🌐 **Web Fetching**: Fetch web pages with optional JavaScript rendering  
- 🔍 **Search Integration**: Query search engines via browser (token-free) or API (token-required) mode 
- 🧠 **Multi-Engine Support**: Works with Bing, Google, DuckDuckGo, Brave Search, Tavily, and custom engines  
- 🛡️ **Proxy Support**: Bypass network bans using proxy support
- 🚀 **End-to-End Workflow**: Full pipeline from search to content extraction for AI and automation use cases


## 🧪 Advanced Features (Coming Soon)

- 🖥️ **Custom Browser Controls**: Set screen size, viewport, and locale for realistic behavior  
- 🕵️‍♂️ **Anti-Bot Evasion**: Use fingerprint spoofing, proxy rotation, and human-like actions to avoid detection  
- 🧠 **Smarter Queries**: Improve search results with prompt rewriting and intent-aware queries  
- 🔗 **Workflow Automation**: Chain steps like search, click, form fill, and scraping into automated flows  
- 🤖 **Agent Integration (MCP)**: Connect with agent frameworks for context-aware, distributed task execution  
- 📊 **Observability**: Monitor success rate, latency, CAPTCHA frequency, and export logs for analysis

## Usage Examples

* Examples in Python and Rust: [examples](/examples/)

## License

Apache License 2.0 - see LICENSE file for details.

## Contributors

Thank you ❤ all human and non-human contributors.

[![tarzi contributors](https://contrib.rocks/image?repo=mirasurf/tarzi.rs "tarzi contributors")](https://github.com/mirasurf/tarzi.rs/graphs/contributors)
