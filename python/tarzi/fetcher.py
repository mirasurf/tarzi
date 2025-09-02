"""
Tarzi-based web content fetcher with multiple formatting modes.

Provides web content fetching capabilities using the tarzi library with support for:
- Raw HTML content from tarzi
- Markdown content formatted by tarzi
- Raw content from tarzi then formatted by LLM provider
"""

import logging
from enum import Enum
from typing import Any, Optional

logger = logging.getLogger(__name__)


class ContentMode(Enum):
    """Content formatting modes for fetched web content."""

    RAW_HTML = "raw_html"  # Raw HTML content from tarzi
    MARKDOWN = "markdown"  # Markdown content formatted by tarzi
    LLM_FORMATTED = "llm_formatted"  # Raw content formatted by LLM


class TarziFetcher:
    """
    Web content fetcher using tarzi library with multiple formatting options.

    Supports three content modes:
    1. RAW_HTML: Returns raw HTML content directly from tarzi
    2. MARKDOWN: Returns markdown-formatted content from tarzi
    3. LLM_FORMATTED: Gets raw HTML from tarzi, then formats using LLM (requires cogents)
    """

    def __init__(
        self,
        llm_provider: str = "openrouter",
        fetch_mode: str = "browser_headless",
        timeout: int = 30,
    ):
        """
        Initialize the TarziFetcher.

        Args:
            llm_provider: LLM provider for content formatting ("openrouter", "llamacpp", "openai", "ollama", etc.)
            fetch_mode: Fetch mode for tarzi ("plain_request", "browser_head", "browser_headless")
            timeout: Request timeout in seconds
        """
        self.llm_provider = llm_provider
        self.fetch_mode = fetch_mode
        self.timeout = timeout

        # Initialize LLM client for LLM_FORMATTED mode (optional)
        self._llm_client = None
        self._setup_llm_client()

        # Initialize tarzi components
        self._setup_tarzi()

    def _setup_llm_client(self) -> None:
        """Setup LLM client for content formatting (requires cogents library)."""
        try:
            # Try to import cogents for LLM functionality
            from cogents.common.llm import get_llm_client

            if self.llm_provider:
                self._llm_client = get_llm_client(
                    provider=self.llm_provider,
                )
                logger.info(f"Initialized LLM client with provider: {self.llm_provider}")
            else:
                logger.warning("LLM provider not configured - LLM_FORMATTED mode will be unavailable")
        except ImportError:
            logger.warning("cogents library not available - LLM_FORMATTED mode will be unavailable")
            self._llm_client = None
        except Exception as e:
            logger.error(f"Failed to initialize LLM client: {e}")
            self._llm_client = None

    def _setup_tarzi(self) -> None:
        """Setup tarzi components with configuration."""
        try:
            # Import tarzi Rust bindings directly
            from .tarzi import Config, Converter, WebFetcher

            # Create tarzi configuration
            config_str = f"""
[fetcher]
timeout = {self.timeout}
format = "html"
"""
            self._config = Config.from_str(config_str)
            self._web_fetcher = WebFetcher.from_config(self._config)
            self._converter = Converter()

            logger.info("Initialized tarzi components successfully")
        except ImportError as e:
            logger.error(f"Failed to import tarzi: {e}")
            raise ImportError("tarzi library is required but not available") from e
        except Exception as e:
            logger.error(f"Failed to setup tarzi: {e}")
            raise

    def fetch(
        self,
        url: str,
        content_mode: ContentMode = ContentMode.MARKDOWN,
        **kwargs: Any,
    ) -> str:
        """
        Fetch web content with specified formatting mode.

        Args:
            url: URL to fetch content from
            content_mode: Content formatting mode (default: MARKDOWN)
            **kwargs: Additional arguments for specific modes

        Returns:
            Formatted web content as string

        Raises:
            ValueError: If content_mode is invalid or configuration is missing
            Exception: If fetching fails
        """
        try:
            if content_mode == ContentMode.RAW_HTML:
                return self._fetch_raw_html(url, **kwargs)
            elif content_mode == ContentMode.MARKDOWN:
                return self._fetch_markdown(url, **kwargs)
            elif content_mode == ContentMode.LLM_FORMATTED:
                if self._llm_client is None:
                    logger.warning("LLM client not configured - falling back to MARKDOWN mode")
                    return self._fetch_markdown(url, **kwargs)
                return self._fetch_llm_formatted(url, **kwargs)
            else:
                raise ValueError(f"Unsupported content mode: {content_mode}")

        except Exception as e:
            logger.error(f"Failed to fetch content from {url}: {e}")
            raise

    def _fetch_raw_html(self, url: str, **kwargs: Any) -> str:
        """
        Fetch raw HTML content using tarzi.

        Args:
            url: URL to fetch
            **kwargs: Additional arguments (unused)

        Returns:
            Raw HTML content
        """
        return self._web_fetcher.fetch(url, self.fetch_mode, "html")

    def _fetch_markdown(self, url: str, **kwargs: Any) -> str:
        """
        Fetch content and convert to markdown using tarzi.

        Args:
            url: URL to fetch
            **kwargs: Additional arguments (unused)

        Returns:
            Markdown-formatted content
        """
        return self._web_fetcher.fetch(url, self.fetch_mode, "markdown")

    def _fetch_llm_formatted(self, url: str, format_prompt: Optional[str] = None, **kwargs: Any) -> str:
        """
        Fetch raw content and format using LLM.

        Args:
            url: URL to fetch
            format_prompt: Custom formatting prompt for the LLM
            **kwargs: Additional arguments (unused)

        Returns:
            LLM-formatted content
        """
        if not self._llm_client:
            raise ValueError("LLM client not configured - cannot use LLM_FORMATTED mode")

        # Get raw HTML content
        raw_html = self._fetch_raw_html(url)

        # Convert to clean text first
        clean_content = self._converter.convert(raw_html, "markdown")

        # Use LLM to format the content
        default_prompt = """Please clean up and format the following web content in a readable way.
Remove navigation menus, advertisements, and irrelevant elements.
Focus on the main content and present it in a clear, structured format.

Content:
{content}

Please provide a clean, well-structured version of the main content:"""

        prompt = format_prompt or default_prompt
        full_prompt = prompt.format(content=clean_content[:8000])  # Limit content size

        try:
            response = self._llm_client.completion(
                messages=[{"role": "user", "content": full_prompt}],
                temperature=0.1,
                max_tokens=100000,
            )
            return response.strip()
        except Exception as e:
            logger.error(f"LLM formatting failed: {e}")
            # Fallback to markdown if LLM fails
            logger.warning("Falling back to markdown format due to LLM failure")
            return clean_content
