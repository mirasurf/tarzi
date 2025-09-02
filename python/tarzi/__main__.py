#!/usr/bin/env python3
"""Main entry point for tarzi CLI that calls Rust functions directly."""

import argparse
import sys


def main():
    """Main entry point that mimics the Rust CLI using Python bindings."""
    parser = argparse.ArgumentParser(
        prog="pytarzi", description="Rust-native lite search for AI applications (Python CLI)"
    )

    subparsers = parser.add_subparsers(dest="command", help="Available commands")

    # Convert subcommand
    convert_parser = subparsers.add_parser("convert", help="Convert HTML to various formats")
    convert_parser.add_argument("-i", "--input", required=True, help="Input HTML string or file path")
    convert_parser.add_argument("-f", "--format", default="markdown", help="Output format: markdown, json, or yaml")
    convert_parser.add_argument("-o", "--output", help="Output file path (optional)")
    convert_parser.add_argument("-v", "--verbose", action="store_true", help="Enable verbose logging")

    # Fetch subcommand
    fetch_parser = subparsers.add_parser("fetch", help="Fetch web page content")
    fetch_parser.add_argument("-u", "--url", required=True, help="URL to fetch")
    fetch_parser.add_argument("-f", "--format", default="html", help="Output format: html, markdown, json, or yaml")
    fetch_parser.add_argument("-o", "--output", help="Output file path (optional)")
    fetch_parser.add_argument("-v", "--verbose", action="store_true", help="Enable verbose logging")

    # Search subcommand
    search_parser = subparsers.add_parser("search", help="Search using search engines")
    search_parser.add_argument("-q", "--query", required=True, help="Search query")
    search_parser.add_argument("-l", "--limit", type=int, default=10, help="Number of results to return")
    search_parser.add_argument("-f", "--format", default="json", help="Output format: json or yaml")
    search_parser.add_argument("-o", "--output", help="Output file path (optional)")
    search_parser.add_argument("-v", "--verbose", action="store_true", help="Enable verbose logging")

    # Search and fetch subcommand
    search_fetch_parser = subparsers.add_parser("search-with-content", help="Search and fetch content for each result")
    search_fetch_parser.add_argument("-q", "--query", required=True, help="Search query")
    search_fetch_parser.add_argument("-l", "--limit", type=int, default=5, help="Number of results to return")
    search_fetch_parser.add_argument(
        "-f", "--format", default="markdown", help="Output format: html, markdown, json, or yaml"
    )
    search_fetch_parser.add_argument("-o", "--output", help="Output file path (optional)")
    search_fetch_parser.add_argument("-v", "--verbose", action="store_true", help="Enable verbose logging")

    # Parse arguments
    args = parser.parse_args()

    if not args.command:
        parser.print_help()
        sys.exit(1)

    try:
        import os

        import tarzi

        # Load configuration with proper precedence:
        # 1. CLI parameters (highest priority - applied later)
        # 2. ~/.tarzi.toml (user config)
        # 3. tarzi.toml (project config)
        # 4. Default values (lowest priority)
        config = None

        # Try to load from project config (tarzi.toml) first
        project_config_path = "tarzi.toml"
        if os.path.exists(project_config_path):
            try:
                config = tarzi.Config.from_file(project_config_path)
                if args.verbose:
                    print(f"Loaded project configuration from {project_config_path}")
            except Exception as e:
                if args.verbose:
                    print(f"Warning: Failed to load project config from {project_config_path}: {e}")

        # Try to load from user config (~/.tarzi.toml) - overrides project config
        user_config_path = os.path.expanduser("~/.tarzi.toml")
        if os.path.exists(user_config_path):
            try:
                user_config = tarzi.Config.from_file(user_config_path)
                if config is None:
                    config = user_config
                else:
                    # Merge user config into project config (user config takes precedence)
                    # For simplicity, we'll just use the user config as it has higher priority
                    config = user_config
                if args.verbose:
                    print(f"Loaded user configuration from {user_config_path}")
            except Exception as e:
                if args.verbose:
                    print(f"Warning: Failed to load user config from {user_config_path}: {e}")

        # If no config was loaded, create a default one
        if config is None:
            config = tarzi.Config()
            if args.verbose:
                print("Using default configuration")

        if args.command == "convert":
            converter = tarzi.Converter()
            result = converter.convert(args.input, args.format)

            if args.output:
                with open(args.output, "w") as f:
                    f.write(result)
                if args.verbose:
                    print(f"Output written to file: {args.output}")
            else:
                print(result)

        elif args.command == "fetch":
            if config:
                fetcher = tarzi.WebFetcher.from_config(config)
            else:
                fetcher = tarzi.WebFetcher()
            # Use config mode or default to plain_request
            mode = config.fetcher.mode if config else "plain_request"
            result = fetcher.fetch(args.url, mode, args.format)

            if args.output:
                with open(args.output, "w") as f:
                    f.write(result)
                if args.verbose:
                    print(f"Output written to file: {args.output}")
            else:
                print(result)

        elif args.command == "search":
            if config:
                engine = tarzi.SearchEngine.from_config(config)
            else:
                engine = tarzi.SearchEngine()
            # Use config limit or default
            limit = config.search.limit if config else 5

            # Perform search
            results = engine.search(args.query, limit)

            # Convert results to the requested format
            if args.format == "json":
                import json

                result_data = [{"title": r.title, "url": r.url, "snippet": r.snippet, "rank": r.rank} for r in results]
                result = json.dumps(result_data, indent=2)
            elif args.format == "yaml":
                import yaml

                result_data = [{"title": r.title, "url": r.url, "snippet": r.snippet, "rank": r.rank} for r in results]
                result = yaml.dump(result_data, default_flow_style=False)
            else:
                result = "\n".join([f"{r.rank}. {r.title}\n   {r.url}\n   {r.snippet}\n" for r in results])

            if args.output:
                with open(args.output, "w") as f:
                    f.write(result)
                if args.verbose:
                    print(f"Output written to file: {args.output}")
            else:
                print(result)

        elif args.command == "search-with-content":
            if config:
                engine = tarzi.SearchEngine.from_config(config)
            else:
                engine = tarzi.SearchEngine()
            # Use config fetch mode or default
            fetch_mode = config.fetcher.mode if config else "plain_request"
            results_with_content = engine.search_with_content(args.query, args.limit, fetch_mode, args.format)

            # Format the combined results
            if args.format == "json":
                import json

                result_data = []
                for search_result, content in results_with_content:
                    result_data.append(
                        {
                            "search_result": {
                                "title": search_result.title,
                                "url": search_result.url,
                                "snippet": search_result.snippet,
                                "rank": search_result.rank,
                            },
                            "content": content,
                        }
                    )
                result = json.dumps(result_data, indent=2)
            elif args.format == "yaml":
                import yaml

                result_data = []
                for search_result, content in results_with_content:
                    result_data.append(
                        {
                            "search_result": {
                                "title": search_result.title,
                                "url": search_result.url,
                                "snippet": search_result.snippet,
                                "rank": search_result.rank,
                            },
                            "content": content,
                        }
                    )
                result = yaml.dump(result_data, default_flow_style=False)
            else:
                # Default to markdown format
                result_parts = []
                for search_result, content in results_with_content:
                    result_parts.append(
                        f"# {search_result.title}\n\n**URL:** {search_result.url}\n\n**Snippet:** {search_result.snippet}\n\n## Content\n\n{content}\n\n---\n"
                    )
                result = "\n".join(result_parts)

            if args.output:
                with open(args.output, "w") as f:
                    f.write(result)
                if args.verbose:
                    print(f"Output written to file: {args.output}")
            else:
                print(result)

    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
