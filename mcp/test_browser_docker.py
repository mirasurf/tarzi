#!/usr/bin/env python3
"""Test script to verify browser functionality in Docker environment."""

import asyncio
import logging
import json
import sys
from pathlib import Path

# Set up logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

def test_imports():
    """Test that all required modules can be imported."""
    try:
        logger.info("Testing module imports...")
        
        # Test MCP imports
        from mcp.server.fastmcp import FastMCP
        logger.info("✅ MCP imports successful")
        
        # Test browser automation imports
        try:
            from selenium import webdriver
            from selenium.webdriver.firefox.options import Options
            from selenium.webdriver.firefox.service import Service
            logger.info("✅ Selenium imports successful")
        except ImportError as e:
            logger.warning(f"⚠️  Selenium import failed: {e}")
        
        try:
            from playwright.sync_api import sync_playwright
            logger.info("✅ Playwright imports successful")
        except ImportError as e:
            logger.warning(f"⚠️  Playwright import failed: {e}")
        
        # Test tarzi imports (will fail without actual tarzi package)
        try:
            import tarzi
            logger.info("✅ Tarzi imports successful")
        except ImportError:
            logger.warning("⚠️  Tarzi import failed (expected in demo environment)")
        
        return True
    except Exception as e:
        logger.error(f"❌ Import test failed: {e}")
        return False

def test_browser_components():
    """Test browser components availability."""
    logger.info("Testing browser components...")
    
    # Check Firefox
    firefox_paths = ['/usr/bin/firefox-esr', '/usr/bin/firefox']
    firefox_found = False
    for path in firefox_paths:
        if Path(path).exists():
            logger.info(f"✅ Firefox found at {path}")
            firefox_found = True
            break
    
    if not firefox_found:
        logger.error("❌ Firefox not found")
        return False
    
    # Check geckodriver
    geckodriver_paths = ['/usr/local/bin/geckodriver', '/usr/bin/geckodriver']
    geckodriver_found = False
    for path in geckodriver_paths:
        if Path(path).exists():
            logger.info(f"✅ Geckodriver found at {path}")
            geckodriver_found = True
            break
    
    if not geckodriver_found:
        logger.error("❌ Geckodriver not found")
        return False
    
    return True

def test_selenium_basic():
    """Test basic Selenium functionality."""
    try:
        logger.info("Testing Selenium browser automation...")
        
        from selenium import webdriver
        from selenium.webdriver.firefox.options import Options
        from selenium.webdriver.firefox.service import Service
        
        # Set up Firefox options
        options = Options()
        options.add_argument('--headless')
        options.add_argument('--no-sandbox')
        options.add_argument('--disable-dev-shm-usage')
        
        # Set up service
        service = Service(executable_path='/usr/local/bin/geckodriver')
        
        # Create driver
        driver = webdriver.Firefox(service=service, options=options)
        
        # Test navigation
        driver.get("about:blank")
        success = "about:blank" in driver.current_url
        
        if success:
            logger.info("✅ Selenium basic test successful")
        else:
            logger.error("❌ Selenium navigation test failed")
        
        driver.quit()
        return success
        
    except Exception as e:
        logger.error(f"❌ Selenium test failed: {e}")
        return False

def test_mcp_server_structure():
    """Test MCP server can be created."""
    try:
        logger.info("Testing MCP server structure...")
        
        from mcp.server.fastmcp import FastMCP
        
        # Create server
        mcp = FastMCP("Test Server")
        
        # Add a simple tool
        @mcp.tool()
        def test_tool(message: str) -> str:
            return f"Echo: {message}"
        
        # Add a simple resource
        @mcp.resource("test://status")
        def test_resource() -> str:
            return "Test resource working"
        
        logger.info("✅ MCP server structure test successful")
        return True
        
    except Exception as e:
        logger.error(f"❌ MCP server test failed: {e}")
        return False

async def main():
    """Run all tests."""
    logger.info("🧪 Starting Tarzi MCP Server Docker Browser Tests")
    logger.info("=" * 60)
    
    test_results = {
        "imports": test_imports(),
        "browser_components": test_browser_components(),
        "selenium_basic": test_selenium_basic(),
        "mcp_server": test_mcp_server_structure(),
    }
    
    logger.info("\n" + "=" * 60)
    logger.info("📊 Test Results Summary:")
    
    passed = 0
    total = len(test_results)
    
    for test_name, result in test_results.items():
        status = "✅ PASS" if result else "❌ FAIL"
        logger.info(f"  {test_name}: {status}")
        if result:
            passed += 1
    
    logger.info(f"\nTotal: {passed}/{total} tests passed")
    
    if passed == total:
        logger.info("🎉 All tests passed! Browser automation is ready!")
        return True
    else:
        logger.error(f"⚠️  {total - passed} tests failed. Check configuration.")
        return False

if __name__ == "__main__":
    success = asyncio.run(main())
    sys.exit(0 if success else 1)