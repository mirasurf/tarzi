# Re-export everything from the Rust module
# High-level Python API imports
from .fetcher import ContentMode, TarziFetcher
from .searcher import SearchWithContentResult, TarziSearcher
from .tarzi import (
    Config,
    Converter,
    SearchEngine,
    SearchResult,
    WebFetcher,
    convert_html,
    fetch_url,
    search_web,
    search_with_content,
)

# Get version dynamically
try:
    from importlib.metadata import version

    __version__ = version("tarzi")
except ImportError:
    # Fallback for older Python versions
    try:
        import pkg_resources

        __version__ = pkg_resources.get_distribution("tarzi").version
    except (ImportError, pkg_resources.DistributionNotFound):
        # Final fallback - read from pyproject.toml
        try:
            import tomllib
        except ImportError:
            import tomli as tomllib

        try:
            with open("pyproject.toml", "rb") as f:
                pyproject = tomllib.load(f)
                __version__ = pyproject["project"]["version"]
        except (FileNotFoundError, KeyError):
            # Last resort fallback
            __version__ = "unknown"

__all__ = [
    "Config",
    "Converter",
    "WebFetcher",
    "SearchEngine",
    "SearchResult",
    "convert_html",
    "fetch_url",
    "search_web",
    "search_with_content",
    "TarziFetcher",
    "TarziSearcher",
    "ContentMode",
    "SearchWithContentResult",
]
