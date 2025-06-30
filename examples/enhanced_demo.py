#!/usr/bin/env python3
"""
Enhanced Demo for Tarzi Python Bindings

This script demonstrates all the enhanced features of the tarzi Python wrapper:
- Comprehensive docstrings and help system
- Enhanced error handling with specific messages
- Proper string representations
- Configuration management
- Real-world usage patterns
"""

import sys
import traceback

def demo_docstrings_and_help():
    """Demonstrate the comprehensive docstring system"""
    print("=" * 60)
    print("📚 DOCSTRINGS AND HELP SYSTEM DEMO")
    print("=" * 60)
    
    print("The tarzi Python bindings now include comprehensive docstrings.")
    print("Here's what you can do once the module is available:\n")
    
    # Example of how to access help
    help_examples = [
        "help(tarzi)",
        "help(tarzi.PyConverter)",
        "help(tarzi.PyConverter.convert)",
        "help(tarzi.PyWebFetcher.fetch)",
        "help(tarzi.PySearchEngine.search)",
        "help(tarzi.convert_html)",
        "help(tarzi.search_web)"
    ]
    
    for example in help_examples:
        print(f"  >>> {example}")
        print("      # Shows comprehensive documentation with:")
        print("      # - Method description")
        print("      # - Parameter types and descriptions")
        print("      # - Return value information")
        print("      # - Exception details")
        print()

def demo_error_handling():
    """Demonstrate enhanced error handling"""
    print("=" * 60)
    print("🚨 ENHANCED ERROR HANDLING DEMO")
    print("=" * 60)
    
    print("The tarzi bindings now provide specific, helpful error messages:")
    print()
    
    error_scenarios = [
        {
            "description": "Invalid format in convert_html",
            "code": 'tarzi.convert_html("<h1>Test</h1>", "invalid_format")',
            "expected": "ValueError: Invalid format 'invalid_format': unsupported format"
        },
        {
            "description": "Invalid fetch mode",
            "code": 'tarzi.fetch_url("https://example.com", "invalid_mode", "html")',
            "expected": "ValueError: Invalid fetch mode 'invalid_mode': unsupported mode"
        },
        {
            "description": "Invalid search mode",
            "code": 'tarzi.search_web("test", "invalid_mode", 5)',
            "expected": "ValueError: Invalid search mode 'invalid_mode': must be 'webquery' or 'apiquery'"
        },
        {
            "description": "Failed URL fetch",
            "code": 'tarzi.fetch_url("https://nonexistent-domain-12345.com", "plain_request", "html")',
            "expected": "RuntimeError: Failed to fetch 'https://nonexistent-domain-12345.com': network error"
        }
    ]
    
    for scenario in error_scenarios:
        print(f"🔍 {scenario['description']}:")
        print(f"   Code: {scenario['code']}")
        print(f"   Error: {scenario['expected']}")
        print()

def demo_string_representations():
    """Demonstrate improved string representations"""
    print("=" * 60)
    print("🎨 STRING REPRESENTATIONS DEMO")
    print("=" * 60)
    
    print("All classes now have proper __repr__ and __str__ methods:")
    print()
    
    representations = [
        {
            "class": "PyConverter",
            "repr": "PyConverter()",
            "str": "Tarzi HTML/text content converter"
        },
        {
            "class": "PyWebFetcher", 
            "repr": "PyWebFetcher()",
            "str": "Tarzi web page fetcher"
        },
        {
            "class": "PySearchEngine",
            "repr": "PySearchEngine()",
            "str": "Tarzi search engine"
        },
        {
            "class": "PyConfig",
            "repr": "PyConfig()",
            "str": "Tarzi configuration"
        },
        {
            "class": "PySearchResult",
            "repr": "PySearchResult(title='Example', url='https://example.com', snippet='Test', rank=1)",
            "str": "[1] Example\nhttps://example.com\nTest"
        }
    ]
    
    for rep in representations:
        print(f"🏷️  {rep['class']}:")
        print(f"   repr(): {rep['repr']}")
        print(f"   str():  {rep['str']}")
        print()

def demo_configuration_features():
    """Demonstrate enhanced configuration features"""
    print("=" * 60)
    print("⚙️  CONFIGURATION FEATURES DEMO")
    print("=" * 60)
    
    sample_config = '''
[general]
log_level = "info"
timeout = 30

[fetcher]
mode = "plain_request"
format = "markdown"
user_agent = "Tarzi Enhanced Demo/1.0"
timeout = 20
web_driver = "chrome"
web_driver_port = 9515
proxy = ""

[search]
mode = "webquery"
engine = "bing"
query_pattern = "https://www.bing.com/search?q={query}"
limit = 10
api_key = ""
'''
    
    print("📝 Sample TOML Configuration:")
    print(sample_config)
    
    print("🔧 Configuration Usage Examples:")
    print("   # Create from string")
    print("   config = tarzi.PyConfig.from_str(config_toml)")
    print()
    print("   # Create from file")
    print("   config = tarzi.PyConfig.from_file('config.toml')")
    print()
    print("   # Use with components")
    print("   converter = tarzi.PyConverter.from_config(config)")
    print("   fetcher = tarzi.PyWebFetcher.from_config(config)")
    print("   search_engine = tarzi.PySearchEngine.from_config(config)")
    print()
    print("   # Save configuration")
    print("   config.save()      # Save to default location")
    print("   config.save_dev()  # Save to development location")
    print()

def demo_comprehensive_workflow():
    """Demonstrate a complete workflow using all features"""
    print("=" * 60)
    print("🚀 COMPREHENSIVE WORKFLOW DEMO")
    print("=" * 60)
    
    workflow_code = '''
# 1. Create configuration
config_toml = """
[fetcher]
format = "markdown"
timeout = 30
user_agent = "My App/1.0"

[search]
engine = "bing"
mode = "webquery"
limit = 5
"""

config = tarzi.PyConfig.from_str(config_toml)
print(f"Configuration: {config}")

# 2. Initialize components
converter = tarzi.PyConverter.from_config(config)
fetcher = tarzi.PyWebFetcher.from_config(config)
search_engine = tarzi.PySearchEngine.from_config(config)

print(f"Converter: {converter}")
print(f"Fetcher: {fetcher}")
print(f"Search Engine: {search_engine}")

# 3. Perform search
query = "Python web scraping best practices"
results = search_engine.search(query, "webquery", 3)

print(f"\\nSearch Results for '{query}':")
for result in results:
    print(result)  # Uses enhanced __str__ method
    print("-" * 40)

# 4. Fetch and convert content
for result in results[:2]:  # Process first 2 results
    try:
        # Fetch content
        html_content = fetcher.fetch_raw(result.url, "plain_request")
        print(f"\\nFetched {len(html_content)} characters from {result.url}")
        
        # Convert to different formats
        markdown = converter.convert(html_content, "markdown")
        json_data = converter.convert(html_content, "json")
        
        print(f"Converted to markdown: {len(markdown)} characters")
        print(f"Converted to JSON: {len(json_data)} characters")
        
    except Exception as e:
        print(f"Error processing {result.url}: {e}")

# 5. One-liner search and fetch
print("\\n=== One-liner search and fetch ===")
results_with_content = tarzi.search_and_fetch(
    "machine learning tutorials", 
    "webquery", 
    2, 
    "plain_request", 
    "markdown"
)

for result, content in results_with_content:
    print(f"\\nResult: {result}")
    print(f"Content preview: {content[:200]}...")
'''

    print("💻 Complete Workflow Example:")
    print(workflow_code)

def demo_advanced_features():
    """Demonstrate advanced features and best practices"""
    print("=" * 60)
    print("🌟 ADVANCED FEATURES AND BEST PRACTICES")
    print("=" * 60)
    
    print("🔧 Best Practices:")
    print("1. Always use try-except blocks for network operations")
    print("2. Use configuration objects for consistent settings")
    print("3. Check result properties before processing")
    print("4. Use appropriate fetch modes for different scenarios")
    print("5. Convert content to appropriate formats based on use case")
    print()
    
    print("🎯 Performance Tips:")
    print("1. Reuse converter, fetcher, and search engine instances")
    print("2. Use 'plain_request' mode for simple pages")
    print("3. Use 'browser_headless' for JavaScript-heavy pages")
    print("4. Limit search results to what you actually need")
    print("5. Cache configuration objects")
    print()
    
    print("🛡️  Error Handling Tips:")
    print("1. Catch specific exceptions (ValueError, RuntimeError)")
    print("2. Check network connectivity before operations")
    print("3. Validate input parameters early")
    print("4. Use timeouts to avoid hanging operations")
    print("5. Log errors for debugging")
    print()

def demo_comparison_with_other_tools():
    """Show how tarzi compares to other tools"""
    print("=" * 60)
    print("⚡ TARZI VS OTHER TOOLS")
    print("=" * 60)
    
    comparisons = [
        {
            "feature": "HTML to Markdown Conversion",
            "tarzi": "Built-in with tarzi.convert_html(html, 'markdown')",
            "others": "Requires separate libraries like html2text or markdownify"
        },
        {
            "feature": "Web Scraping + Search",
            "tarzi": "Integrated: search_and_fetch() does both in one call",
            "others": "Need separate tools: requests + BeautifulSoup + search API"
        },
        {
            "feature": "Multiple Output Formats",
            "tarzi": "HTML, Markdown, JSON, YAML built-in",
            "others": "Usually only support one format"
        },
        {
            "feature": "Browser Rendering",
            "tarzi": "Built-in headless browser support",
            "others": "Need separate Selenium/Playwright setup"
        },
        {
            "feature": "Configuration Management",
            "tarzi": "TOML-based configuration with validation",
            "others": "Manual configuration or JSON/YAML"
        }
    ]
    
    for comp in comparisons:
        print(f"🏆 {comp['feature']}:")
        print(f"   Tarzi: {comp['tarzi']}")
        print(f"   Others: {comp['others']}")
        print()

def main():
    """Run all demonstrations"""
    print("🎉 TARZI PYTHON BINDINGS - ENHANCED FEATURES DEMO")
    print("=" * 80)
    print()
    print("This demo showcases all the enhanced features of the tarzi Python wrapper.")
    print("Note: Actual code execution requires resolving PyO3 linking issues first.")
    print()
    
    try:
        demo_docstrings_and_help()
        demo_error_handling()
        demo_string_representations()
        demo_configuration_features()
        demo_comprehensive_workflow()
        demo_advanced_features()
        demo_comparison_with_other_tools()
        
        print("=" * 80)
        print("✅ DEMO COMPLETED SUCCESSFULLY!")
        print("=" * 80)
        print()
        print("🔧 To use these features:")
        print("1. Fix PyO3 linking issues (see setup guide)")
        print("2. Install the tarzi Python package")
        print("3. Import tarzi and start using the enhanced features!")
        print()
        print("📚 For more examples, see:")
        print("- examples/basic_usage.py")
        print("- examples/search_engines.py")
        print("- test_python_bindings.py")
        
    except Exception as e:
        print(f"❌ Demo failed with error: {e}")
        traceback.print_exc()
        return 1
    
    return 0

if __name__ == "__main__":
    sys.exit(main()) 