"""
Tarzi-based web search functionality with content fetching support.

Provides web search capabilities using the tarzi library with support for:
- Basic web search operations
- Search with content fetching using multiple formatting modes
"""

import logging
from dataclasses import dataclass
from typing import List

from .fetcher import ContentMode, TarziFetcher

logger = logging.getLogger(__name__)


@dataclass
class SearchResult:
    """Represents a single search result."""

    title: str
    url: str
    snippet: str
    rank: int = 0


@dataclass
class SearchWithContentResult:
    """Represents a search result with fetched content."""

    result: SearchResult
    content: str
    content_mode: ContentMode


class TarziSearcher:
    """
    Web search functionality using tarzi library.

    Provides both basic search operations and search with content fetching,
    supporting multiple content formatting modes through TarziFetcher integration.
    """

    def __init__(
        self,
        search_engine: str = "bing",
    ):
        """
        Initialize the TarziSearcher.

        Args:
            search_engine: Search engine to use ("google", "bing", "duckduckgo", etc.)
        """
        self.search_engine = search_engine
        self.search_mode = "webquery"  # Always use webquery mode

        # Initialize tarzi components
        self._setup_tarzi()

    def _setup_tarzi(self) -> None:
        """Setup tarzi search components with configuration."""
        try:
            # Import tarzi Rust bindings directly
            from .tarzi import Config, SearchEngine

            # Create tarzi configuration with search engine settings
            config_str = f"""
[search]
engine = "{self.search_engine}"
autoswitch = "smart"
limit = 10
"""
            self._config = Config.from_str(config_str)
            self._search_engine = SearchEngine.from_config(self._config)

            logger.info(f"Initialized tarzi search engine with: {self.search_engine}")
        except ImportError as e:
            logger.error(f"Failed to import tarzi: {e}")
            raise ImportError("tarzi library is required but not available") from e
        except Exception as e:
            logger.error(f"Failed to setup tarzi search: {e}")
            raise

    def search(
        self,
        query: str,
        max_results: int = 10,
    ) -> List[SearchResult]:
        """
        Perform a web search and return search results.

        Args:
            query: Search query string
            max_results: Maximum number of results to return

        Returns:
            List of SearchResult objects

        Raises:
            Exception: If search fails
        """
        logger.info(f"Searching for '{query}' with mode: {self.search_mode}, max_results: {max_results}")

        try:
            # Use tarzi search_web function for direct search
            from .tarzi import search_web

            tarzi_results = search_web(query, self.search_mode, max_results)

            # Convert tarzi results to our SearchResult format
            results = []
            for i, result in enumerate(tarzi_results):
                search_result = SearchResult(
                    title=result.title,
                    url=result.url,
                    snippet=result.snippet,
                    rank=getattr(result, "rank", i + 1),  # Use rank if available, else use index
                )
                results.append(search_result)

            logger.info(f"Found {len(results)} search results")
            return results

        except Exception as e:
            logger.error(f"Search failed for query '{query}': {e}")
            raise

    def search_with_content(
        self,
        query: str,
        max_results: int = 5,
        content_mode: ContentMode = ContentMode.MARKDOWN,
        fetch_mode: str = "plain_request",
    ) -> List[SearchWithContentResult]:
        """
        Perform a web search and fetch content from the results.

        Args:
            query: Search query string
            max_results: Maximum number of results to return
            content_mode: Content formatting mode for fetched content
            fetch_mode: Fetch mode for tarzi ("plain_request", "browser_head", "browser_headless")

        Returns:
            List of SearchWithContentResult objects

        Raises:
            Exception: If search or fetching fails
        """
        try:
            if content_mode in [ContentMode.RAW_HTML, ContentMode.MARKDOWN]:
                return self._search_with_content_tarzi_native(query, max_results, content_mode, fetch_mode)
            else:
                return self._search_with_content_separate(query, max_results, content_mode, fetch_mode)

        except Exception as e:
            logger.error(f"Search with content failed for query '{query}': {e}")
            raise

    def _search_with_content_tarzi_native(
        self,
        query: str,
        max_results: int,
        content_mode: ContentMode,
        fetch_mode: str,
    ) -> List[SearchWithContentResult]:
        """
        Use tarzi's native search_with_content function.

        Args:
            query: Search query
            max_results: Maximum results
            content_mode: Content mode
            fetch_mode: Fetch mode

        Returns:
            List of SearchWithContentResult objects
        """
        from .tarzi import search_with_content

        # Map our content modes to tarzi formats
        tarzi_format = "html" if content_mode == ContentMode.RAW_HTML else "markdown"

        # Use tarzi's search_with_content
        results_with_content = search_with_content(query, self.search_mode, max_results, fetch_mode, tarzi_format)

        # Convert to our format
        search_results = []
        for i, (result, content) in enumerate(results_with_content):
            search_result = SearchResult(
                title=result.title,
                url=result.url,
                snippet=result.snippet,
                rank=getattr(result, "rank", i + 1),
            )
            search_with_content = SearchWithContentResult(
                result=search_result, content=content, content_mode=content_mode
            )
            search_results.append(search_with_content)

        return search_results

    def _search_with_content_separate(
        self,
        query: str,
        max_results: int,
        content_mode: ContentMode,
        fetch_mode: str = "plain_request",
    ) -> List[SearchWithContentResult]:
        """
        Search first, then fetch content separately using TarziFetcher.

        Args:
            query: Search query
            max_results: Maximum results
            content_mode: Content mode
            fetch_mode: Fetch mode for tarzi ("plain_request", "browser_head", "browser_headless")

        Returns:
            List of SearchWithContentResult objects
        """
        # First, perform the search
        search_results = self.search(query, max_results)

        # Then, fetch content for each result
        results_with_content = []
        for result in search_results:
            try:
                fetcher = TarziFetcher(fetch_mode=fetch_mode)
                content = fetcher.fetch(result.url, content_mode=content_mode)
                search_with_content = SearchWithContentResult(result=result, content=content, content_mode=content_mode)
                results_with_content.append(search_with_content)
            except Exception as e:
                logger.warning(f"Failed to fetch content from {result.url}: {e}")
                # Add result with empty content rather than skipping
                search_with_content = SearchWithContentResult(
                    result=result,
                    content=f"Failed to fetch content: {str(e)}",
                    content_mode=content_mode,
                )
                results_with_content.append(search_with_content)

        return results_with_content
