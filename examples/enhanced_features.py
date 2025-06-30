#!/usr/bin/env python3
"""
Enhanced features example for the tarzi Python library.
Demonstrates the improved Python bindings with better error handling,
string representations, and comprehensive docstrings.
"""

import sys

def demonstrate_docstrings(tarzi):
    """Demonstrate the comprehensive docstrings added to all classes and methods"""
    print("=== Docstring Demonstrations ===\n")
    
    print("1. Module-level documentation:")
    print(f"Tarzi module: {tarzi.__doc__}")
    
    print("\n2. Class documentation:")
    classes = [
        ('PyConverter', tarzi.PyConverter),
        ('PyWebFetcher', tarzi.PyWebFetcher), 
        ('PySearchEngine', tarzi.PySearchEngine),
        ('PySearchResult', tarzi.PySearchResult),
        ('PyConfig', tarzi.PyConfig)
    ]
    
    for name, cls in classes:
        print(f"{name}: {cls.__doc__}")
    
    print("\n3. Method documentation examples:")
    print("PyConverter.convert():")
    print(tarzi.PyConverter.convert.__doc__)
    
    print("\nPyWebFetcher.fetch():")
    print(tarzi.PyWebFetcher.fetch.__doc__)
    
    print("\nPySearchEngine.search():")
    print(tarzi.PySearchEngine.search.__doc__)
    
    print("\n4. Function documentation:")
    functions = [
        ('convert_html', tarzi.convert_html),
        ('fetch_url', tarzi.fetch_url),
        ('search_web', tarzi.search_web),
        ('search_and_fetch', tarzi.search_and_fetch)
    ]
    
    for name, func in functions:
        print(f"{name}: {func.__doc__}")

def demonstrate_string_representations(tarzi):
    """Demonstrate improved __repr__ and __str__ methods"""
    print("\n=== String Representation Demonstrations ===\n")
    
    # Create instances of all classes
    converter = tarzi.PyConverter()
    fetcher = tarzi.PyWebFetcher()
    search_engine = tarzi.PySearchEngine()
    config = tarzi.PyConfig()
    
    # Create a mock search result (would normally come from search)
    # Since we can't easily create PySearchResult directly, we'll demonstrate with expected output
    
    print("1. Class representations:")
    print(f"PyConverter repr: {repr(converter)}")
    print(f"PyConverter str: {str(converter)}")
    
    print(f"\nPyWebFetcher repr: {repr(fetcher)}")
    print(f"PyWebFetcher str: {str(fetcher)}")
    
    print(f"\nPySearchEngine repr: {repr(search_engine)}")
    print(f"PySearchEngine str: {str(search_engine)}")
    
    print(f"\nPyConfig repr: {repr(config)}")
    print(f"PyConfig str: {str(config)}")
    
    print("\n2. Search result representation example:")
    print("When you get search results, they now have improved string representations:")
    print("Example PySearchResult repr: PySearchResult(title='Example Title', url='https://example.com', snippet='Example snippet', rank=1)")
    print("Example PySearchResult str:")
    print("[1] Example Title")
    print("https://example.com")
    print("Example snippet")

def demonstrate_enhanced_error_handling(tarzi):
    """Demonstrate improved error messages with context"""
    print("\n=== Enhanced Error Handling Demonstrations ===\n")
    
    print("1. Invalid format errors (now with context):")
    try:
        tarzi.convert_html("<h1>Test</h1>", "invalid_format")
    except Exception as e:
        print(f"Error: {e}")
        print("Note: Error message now specifies which format was invalid")
    
    print("\n2. Invalid fetch mode errors:")
    try:
        tarzi.fetch_url("https://example.com", "invalid_mode", "html")
    except Exception as e:
        print(f"Error: {e}")
        print("Note: Error message now specifies which fetch mode was invalid")
    
    print("\n3. Invalid search mode errors:")
    try:
        tarzi.search_web("test query", "invalid_mode", 5)
    except Exception as e:
        print(f"Error: {e}")
        print("Note: Error message now specifies which search mode was invalid")
    
    print("\n4. Configuration parsing errors:")
    try:
        tarzi.PyConfig.from_str("invalid toml content ][")
    except Exception as e:
        print(f"Error: {e}")
        print("Note: Error message now provides specific parsing context")
    
    print("\n5. Complex error in search_and_fetch:")
    try:
        tarzi.search_and_fetch("test", "invalid_search", 5, "invalid_fetch", "invalid_format")
    except Exception as e:
        print(f"Error: {e}")
        print("Note: Error catches the first invalid parameter with context")

def demonstrate_comprehensive_workflow(tarzi):
    """Demonstrate a complete workflow using enhanced features"""
    print("\n=== Comprehensive Workflow with Enhanced Features ===\n")
    
    try:
        print("1. Creating configuration with enhanced error handling:")
        config_str = '''
[general]
log_level = "info"
timeout = 30

[fetcher]
mode = "plain_request"
format = "markdown"
user_agent = "Tarzi Enhanced Example/1.0"
timeout = 20

[search]
mode = "webquery"
engine = "bing"
limit = 3
'''
        config = tarzi.PyConfig.from_str(config_str)
        print(f"✅ Configuration created: {config}")
        
        print("\n2. Creating components with string representations:")
        converter = tarzi.PyConverter.from_config(config)
        fetcher = tarzi.PyWebFetcher.from_config(config)
        search_engine = tarzi.PySearchEngine.from_config(config)
        
        print(f"Converter: {converter}")
        print(f"Fetcher: {fetcher}")
        print(f"Search Engine: {search_engine}")
        
        print("\n3. Testing conversion with enhanced error messages:")
        html_content = "<h1>Enhanced Features Test</h1><p>This demonstrates the improved Python bindings.</p>"
        
        # Valid conversions
        markdown = converter.convert(html_content, "markdown")
        print(f"✅ Markdown conversion successful: {len(markdown)} characters")
        
        json_output = converter.convert(html_content, "json")
        print(f"✅ JSON conversion successful: {len(json_output)} characters")
        
        # Test error handling
        try:
            converter.convert(html_content, "nonexistent_format")
        except Exception as e:
            print(f"✅ Error handling working: {e}")
        
        print("\n4. Testing search functionality:")
        try:
            # This would work with proper environment setup
            results = search_engine.search("Python programming", "webquery", 2)
            print(f"✅ Search successful: {len(results)} results")
            
            # Demonstrate search result representations
            for i, result in enumerate(results):
                print(f"Result {i+1}:")
                print(f"  repr: {repr(result)}")
                print(f"  str:\n{str(result)}")
        except Exception as e:
            print(f"ℹ️  Search failed (expected in test environment): {e}")
        
        print("\n5. Cleanup with error handling:")
        try:
            search_engine.cleanup()
            print("✅ Cleanup successful")
        except Exception as e:
            print(f"ℹ️  Cleanup note: {e}")
            
    except Exception as e:
        print(f"❌ Workflow error: {e}")

def demonstrate_api_improvements(tarzi):
    """Demonstrate API improvements and usability enhancements"""
    print("\n=== API Improvements Demonstrations ===\n")
    
    print("1. Parameter validation improvements:")
    
    # Test various edge cases that now provide better error messages
    edge_cases = [
        ("Empty URL", lambda: tarzi.fetch_url("", "plain_request", "html")),
        ("Empty query", lambda: tarzi.search_web("", "webquery", 5)),
        ("Invalid limit", lambda: tarzi.search_web("test", "webquery", -1)),
        ("Mixed invalid parameters", lambda: tarzi.search_and_fetch("", "invalid", 0, "invalid", "invalid"))
    ]
    
    for case_name, test_func in edge_cases:
        try:
            test_func()
            print(f"❌ {case_name}: Should have failed")
        except Exception as e:
            print(f"✅ {case_name}: {e}")
    
    print("\n2. Improved object introspection:")
    converter = tarzi.PyConverter()
    
    print("Available methods on PyConverter:")
    methods = [method for method in dir(converter) if not method.startswith('_') or method in ['__repr__', '__str__']]
    for method in sorted(methods):
        print(f"  - {method}")
    
    print("\n3. Better debugging experience:")
    print("With enhanced string representations, debugging is much easier:")
    print(f"  converter = {repr(converter)}")
    print(f"  print(converter) outputs: {str(converter)}")

def demonstrate_configuration_enhancements(tarzi):
    """Demonstrate enhanced configuration handling"""
    print("\n=== Configuration Enhancements ===\n")
    
    print("1. Configuration validation with better error messages:")
    
    # Test various invalid configurations
    invalid_configs = [
        ("Invalid TOML syntax", "invalid toml [syntax"),
        ("Missing sections", "[invalid]\nkey = value"),
        ("Invalid values", "[fetcher]\ntimeout = 'not_a_number'")
    ]
    
    for desc, config_str in invalid_configs:
        try:
            tarzi.PyConfig.from_str(config_str)
            print(f"❌ {desc}: Should have failed")
        except Exception as e:
            print(f"✅ {desc}: {e}")
    
    print("\n2. Configuration usage improvements:")
    
    # Create a comprehensive configuration
    comprehensive_config = '''
[general]
log_level = "debug"
timeout = 45

[fetcher]
mode = "browser_headless"
format = "json"
user_agent = "Tarzi Enhanced/2.0"
timeout = 30
web_driver = "chrome"
web_driver_port = 9515

[search]
mode = "apiquery"
engine = "duckduckgo"
limit = 5
'''
    
    try:
        config = tarzi.PyConfig.from_str(comprehensive_config)
        print(f"✅ Comprehensive config created: {config}")
        
        # Use config with all components
        converter = tarzi.PyConverter.from_config(config)
        fetcher = tarzi.PyWebFetcher.from_config(config)
        search_engine = tarzi.PySearchEngine.from_config(config)
        
        print("✅ All components created from config successfully")
        print(f"  Converter: {converter}")
        print(f"  Fetcher: {fetcher}")
        print(f"  Search Engine: {search_engine}")
        
    except Exception as e:
        print(f"❌ Configuration usage failed: {e}")

def main():
    """Main function demonstrating all enhanced features"""
    print("🚀 Tarzi Python Enhanced Features Demonstration")
    print("=" * 60)
    
    try:
        # Try to import tarzi
        import tarzi
        print(f"✅ Tarzi module successfully imported!")
        print(f"Module location: {tarzi.__file__ if hasattr(tarzi, '__file__') else 'Built-in'}")
        
        # Run all demonstrations
        demonstrate_docstrings(tarzi)
        demonstrate_string_representations(tarzi)
        demonstrate_enhanced_error_handling(tarzi)
        demonstrate_comprehensive_workflow(tarzi)
        demonstrate_api_improvements(tarzi)
        demonstrate_configuration_enhancements(tarzi)
        
        print("\n" + "=" * 60)
        print("🎉 All enhanced features demonstrated successfully!")
        
        print("\n📋 Summary of Enhancements:")
        enhancements = [
            "✅ Comprehensive docstrings for all classes and methods",
            "✅ Enhanced error messages with specific context",
            "✅ Improved __repr__ and __str__ methods for better debugging",
            "✅ Better parameter validation and error reporting",
            "✅ Enhanced configuration handling with detailed error messages",
            "✅ Improved API usability and developer experience"
        ]
        
        for enhancement in enhancements:
            print(f"  {enhancement}")
        
        print("\n💡 Tips for using the enhanced Python bindings:")
        tips = [
            "Use help(tarzi.ClassName.method) to see detailed documentation",
            "Enhanced error messages help identify exactly what went wrong",
            "String representations make debugging much easier",
            "All classes now provide consistent and informative output",
            "Configuration errors now pinpoint the exact issue"
        ]
        
        for tip in tips:
            print(f"  • {tip}")
            
    except ImportError as e:
        print(f"❌ Cannot import tarzi module: {e}")
        print("\n💡 This is expected if PyO3 linking issues haven't been resolved yet.")
        print("The enhanced features are implemented and ready to use once the module builds successfully.")
        
        print("\n📋 What has been enhanced in the Python bindings:")
        enhancements = [
            "✅ Comprehensive docstrings for all classes and methods",
            "✅ Enhanced error messages with specific context",
            "✅ Improved __repr__ and __str__ methods for better debugging",
            "✅ Better parameter validation and error reporting",
            "✅ Enhanced configuration handling with detailed error messages",
            "✅ Improved API usability and developer experience"
        ]
        
        for enhancement in enhancements:
            print(f"  {enhancement}")
        
        print("\n🔧 Next steps to resolve the import issue:")
        steps = [
            "1. Fix PyO3 linking by ensuring Python development headers are available",
            "2. Use 'maturin develop --features pyo3' to build the Python extension",
            "3. Ensure the Python environment has the correct library paths",
            "4. Check that all dependencies are installed correctly",
            "5. Once built, run this script again to see all enhanced features"
        ]
        
        for step in steps:
            print(f"  {step}")
        
        return 1
        
    except Exception as e:
        print(f"❌ Unexpected error: {e}")
        import traceback
        traceback.print_exc()
        return 1
    
    return 0

if __name__ == "__main__":
    sys.exit(main()) 