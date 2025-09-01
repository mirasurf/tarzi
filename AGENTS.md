# Tarzi Development Rules & Guidelines

This document contains the development rules and guidelines for **Tarzi**—a rust-native lite search for AI applications.

## General Considerations

- We want to build a Rust project that is still runnable after 10 years. Keep this as the basic rule to manage build toolchain.
- After generation, always make sure the code format and linting are performed well.
- Always DO NOT change unrelated parts beyond user requirements.
- Always make sure the existing unittests and integration tests are passed.
- Always prefer modularized design for each module.
- Always declare constants value modules to contain magic values. DO NOT treat error messages as constants.
- For Makefile, always classify subcommands into category and unify subcommands pattern: `make foo` (to run foo for Rust and Python), `make foo-python` (to run foo for python), `make foo-rust` (to run foo for Rust)
- DO NOT declare magic values used in multiple positions. Declare constant values instead.

## Implementation Considerations for Rust

- **Minimize external dependencies**: Use only well-maintained, stable crates with a strong community or institutional backing. Prefer the Rust standard library for core functionality like file I/O, threading, and networking.
- **Audit dependencies**: Use `cargo tree` or `cargo audit` to review dependencies and their transitive dependencies. Avoid crates with excessive dependencies or those that haven't been updated recently.
- **Use stable Rust features**: Avoid experimental or nightly-only features (e.g., unstable APIs or `#![feature(...)]`). Stick to the stable Rust release channel to ensure compatibility with future Rust versions.
- **Lock Rust Version and Toolchain**: For example, pin the Rust toolchain, document toolchain installation, consider vendoring the toolchain with `cargo vendor`, and regularly update vendored dependencies
- **Write Robust, Portable Code**: Write code that avoids platform-specific assumptions, avoids external system dependencies. For platform-specific code, use `#[cfg(...)]` to handle differences gracefully.
- Always use `cargo fmt` and `cargo clippy` to format and lint code.

## Implementation Considerations for Python

- Always manage python project with `pyproject.toml`
- Always run `make check` and fix errors before finishing code modification.

## Search Engine Architecture

Here is the definition and logics about search engines:

- Each engine provider can serve either webquery or apiquery, or both.
- The webquery mode always requires no api-key.
- The apiquery mode of some engines may require api-key.

### Known Engine List

| Engine        | Web Query | API Query | API Key Required |
|---------------|-----------|-----------|------------------|
| Bing          | Yes       | No        | N/A              |
| Google        | Yes       | Yes       | Yes              |
| Brave         | Yes       | Yes       | Yes              |
| DuckDuckGo    | Yes       | Yes       | No               |
| Exa           | Yes       | Yes       | Yes              |
| Travily       | No        | Yes       | Yes              |
| Baidu         | Yes       | Yes       | Yes              |

### Engine Implementation Guidelines

- Engine `google` and `google_serpe` are different search providers.
- Each engine has specific query pattern, and webquery and apiquery modes always have different query patterns.
- Each engine should have specific parser for webquery and apiquery modes.
- Each engine should have different implementation about search functionalities in webquery or apiquery mode.
