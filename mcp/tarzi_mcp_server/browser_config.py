#!/usr/bin/env python3
"""Browser configuration module for Tarzi MCP Server."""

import os
import logging
from typing import Dict, Optional, Any
from pathlib import Path

logger = logging.getLogger(__name__)


class BrowserConfig:
    """Configuration for browser automation in Tarzi MCP Server."""
    
    def __init__(self):
        """Initialize browser configuration with environment variables and defaults."""
        self.firefox_binary_path = os.getenv('FIREFOX_BINARY_PATH', '/usr/bin/firefox-esr')
        self.geckodriver_path = os.getenv('GECKODRIVER_PATH', '/usr/local/bin/geckodriver')
        self.display = os.getenv('DISPLAY', ':99')
        self.headless = os.getenv('MOZ_HEADLESS', '1') == '1'
        
        # Browser data directories
        self.browser_data_dir = Path(os.getenv('TARZI_BROWSER_DATA_DIR', '/app/browser-data'))
        self.profile_path = Path(os.getenv('TARZI_BROWSER_PROFILE_PATH', '/app/.mozilla/firefox/tarzi.profile'))
        self.cache_dir = Path(os.getenv('TARZI_BROWSER_CACHE_DIR', '/app/browser-data/cache'))
        self.downloads_dir = Path(os.getenv('TARZI_BROWSER_DOWNLOADS_DIR', '/app/browser-data/downloads'))
        
        # Browser options
        self.timeout = int(os.getenv('TARZI_BROWSER_TIMEOUT', '30'))
        self.window_size = os.getenv('TARZI_BROWSER_WINDOW_SIZE', '1920,1080')
        self.user_agent = os.getenv('TARZI_BROWSER_USER_AGENT', 
                                   'Mozilla/5.0 (X11; Linux x86_64; rv:91.0) Gecko/20100101 Firefox/91.0')
        
        # Selenium specific
        self.selenium_browser = os.getenv('SELENIUM_BROWSER', 'firefox')
        
        # Playwright specific
        self.playwright_browsers_path = os.getenv('PLAYWRIGHT_BROWSERS_PATH', '/app/.playwright')
        
        self._setup_directories()
    
    def _setup_directories(self):
        """Create necessary directories for browser operation."""
        try:
            self.browser_data_dir.mkdir(parents=True, exist_ok=True)
            self.profile_path.mkdir(parents=True, exist_ok=True)
            self.cache_dir.mkdir(parents=True, exist_ok=True)
            self.downloads_dir.mkdir(parents=True, exist_ok=True)
            
            logger.info(f"Browser directories set up successfully")
        except Exception as e:
            logger.warning(f"Failed to create browser directories: {e}")
    
    def get_selenium_options(self) -> Dict[str, Any]:
        """Get Selenium Firefox options for browser automation."""
        from selenium.webdriver.firefox.options import Options
        
        options = Options()
        
        # Basic headless setup
        if self.headless:
            options.add_argument('--headless')
        
        # Window size
        width, height = self.window_size.split(',')
        options.add_argument(f'--width={width}')
        options.add_argument(f'--height={height}')
        
        # Performance and security options
        options.add_argument('--no-sandbox')
        options.add_argument('--disable-dev-shm-usage')
        options.add_argument('--disable-gpu')
        options.add_argument('--disable-web-security')
        options.add_argument('--disable-features=VizDisplayCompositor')
        options.add_argument('--disable-background-timer-throttling')
        options.add_argument('--disable-renderer-backgrounding')
        options.add_argument('--disable-backgrounding-occluded-windows')
        
        # Set profile path
        if self.profile_path.exists():
            options.add_argument(f'--profile={self.profile_path}')
        
        # Set binary path
        if Path(self.firefox_binary_path).exists():
            options.binary_location = self.firefox_binary_path
        
        # Set user agent
        options.set_preference("general.useragent.override", self.user_agent)
        
        # Download preferences
        options.set_preference("browser.download.dir", str(self.downloads_dir))
        options.set_preference("browser.download.folderList", 2)
        options.set_preference("browser.helperApps.neverAsk.saveToDisk", 
                              "application/octet-stream,text/csv,text/xml,application/xml")
        
        return {
            'options': options,
            'executable_path': self.geckodriver_path if Path(self.geckodriver_path).exists() else None
        }
    
    def get_playwright_config(self) -> Dict[str, Any]:
        """Get Playwright Firefox configuration for browser automation."""
        config = {
            'headless': self.headless,
            'viewport': {
                'width': int(self.window_size.split(',')[0]),
                'height': int(self.window_size.split(',')[1])
            },
            'user_agent': self.user_agent,
            'ignore_https_errors': True,
            'timeout': self.timeout * 1000,  # Playwright uses milliseconds
        }
        
        # Set Firefox binary path if available
        if Path(self.firefox_binary_path).exists():
            config['executable_path'] = self.firefox_binary_path
        
        return config
    
    def get_tarzi_browser_config(self) -> Dict[str, Any]:
        """Get configuration for tarzi browser automation."""
        return {
            'browser_type': 'firefox',
            'headless': self.headless,
            'binary_path': self.firefox_binary_path,
            'driver_path': self.geckodriver_path,
            'profile_path': str(self.profile_path),
            'data_dir': str(self.browser_data_dir),
            'cache_dir': str(self.cache_dir),
            'downloads_dir': str(self.downloads_dir),
            'timeout': self.timeout,
            'window_size': self.window_size,
            'user_agent': self.user_agent,
            'display': self.display,
        }
    
    def is_browser_available(self) -> bool:
        """Check if browser components are available."""
        firefox_available = Path(self.firefox_binary_path).exists()
        geckodriver_available = Path(self.geckodriver_path).exists()
        
        logger.info(f"Browser availability check:")
        logger.info(f"  Firefox ({self.firefox_binary_path}): {firefox_available}")
        logger.info(f"  Geckodriver ({self.geckodriver_path}): {geckodriver_available}")
        
        return firefox_available and geckodriver_available
    
    def test_browser_connection(self) -> bool:
        """Test if browser can be started successfully."""
        try:
            from selenium import webdriver
            from selenium.webdriver.firefox.service import Service
            
            selenium_config = self.get_selenium_options()
            service = Service(executable_path=selenium_config['executable_path'])
            
            # Create a test browser instance
            driver = webdriver.Firefox(
                service=service,
                options=selenium_config['options']
            )
            
            # Test basic navigation
            driver.get("about:blank")
            success = "about:blank" in driver.current_url
            
            driver.quit()
            logger.info(f"Browser connection test: {'SUCCESS' if success else 'FAILED'}")
            return success
            
        except Exception as e:
            logger.error(f"Browser connection test failed: {e}")
            return False
    
    def get_environment_info(self) -> Dict[str, Any]:
        """Get environment information for debugging."""
        return {
            'display': self.display,
            'headless': self.headless,
            'firefox_binary': self.firefox_binary_path,
            'firefox_exists': Path(self.firefox_binary_path).exists(),
            'geckodriver_path': self.geckodriver_path,
            'geckodriver_exists': Path(self.geckodriver_path).exists(),
            'profile_path': str(self.profile_path),
            'profile_exists': self.profile_path.exists(),
            'browser_data_dir': str(self.browser_data_dir),
            'data_dir_exists': self.browser_data_dir.exists(),
            'window_size': self.window_size,
            'timeout': self.timeout,
            'user_agent': self.user_agent[:50] + '...' if len(self.user_agent) > 50 else self.user_agent,
        }


# Global browser configuration instance
browser_config = BrowserConfig()


def get_browser_config() -> BrowserConfig:
    """Get the global browser configuration instance."""
    return browser_config


def test_browser_setup():
    """Test the complete browser setup."""
    config = get_browser_config()
    
    print("🔧 Browser Configuration Test")
    print("=" * 40)
    
    # Check availability
    available = config.is_browser_available()
    print(f"Browser Available: {'✅ YES' if available else '❌ NO'}")
    
    # Show environment info
    env_info = config.get_environment_info()
    for key, value in env_info.items():
        status = "✅" if key.endswith('_exists') and value else "❌" if key.endswith('_exists') else "ℹ️"
        print(f"{status} {key}: {value}")
    
    # Test connection if available
    if available:
        print("\n🧪 Testing browser connection...")
        connection_ok = config.test_browser_connection()
        print(f"Connection Test: {'✅ SUCCESS' if connection_ok else '❌ FAILED'}")
    
    return available


if __name__ == "__main__":
    test_browser_setup()