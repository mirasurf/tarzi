# Search Result Parsers

This submodule provides a flexible and extensible system for parsing search results from different search engines. It supports both built-in parsers for popular search engines and custom parsers for user-defined engines.

## Architecture

### Core Components

1. **`SearchResultParser` trait** - Defines the interface for all parsers
2. **`ParserFactory`** - Factory pattern for creating and managing parsers
3. **Built-in parsers** - Pre-implemented parsers for popular search engines
4. **Custom parser system** - Extensible system for user-defined parsers

### Built-in Search Engine Parsers

- **BingParser** - Parses Bing search results
- **GoogleParser** - Parses Google search results  
- **DuckDuckGoParser** - Parses DuckDuckGo search results
- **BraveParser** - Parses Brave Search results
- **SearchApiParser** - Parses SearchApi results

### Custom Parser Support

- **CustomParser** - Configurable parser for custom search engines
- **CustomParserConfig** - Configuration structure for custom CSS selectors
- Support for registering completely custom parser implementations

## Usage Examples

### Basic Usage with Built-in Parsers

```rust
use tarzi::search::{ParserFactory, SearchEngineType};

let factory = ParserFactory::new();
let parser = factory.get_parser(&SearchEngineType::Google);
let results = parser.parse(html_content, 10)?;
```

### Custom Parser Configuration

```rust
use tarzi::search::{CustomParser, CustomParserConfig};

let mut config = CustomParserConfig::default();
config.result_container_selector = ".search-result".to_string();
config.title_selector = "h3 a".to_string();
config.url_selector = ".result-url".to_string();
config.snippet_selector = ".result-snippet".to_string();

let parser = CustomParser::with_config("MyEngine".to_string(), config);
let results = parser.parse(html_content, 10)?;
```

### Registering Custom Parsers

```rust
use tarzi::search::{SearchEngine, SearchResultParser};

// Implement your own parser
struct MyCustomParser;
impl SearchResultParser for MyCustomParser {
    // Implementation details...
}

let mut search_engine = SearchEngine::new();
search_engine.register_custom_parser(
    "MyEngine".to_string(), 
    Box::new(MyCustomParser)
);
```

## Implementation Notes

### Current Implementation Status

- **Mock Implementation**: All parsers currently provide mock/dummy results for demonstration
- **Real Implementation**: In production, parsers would use HTML parsing libraries like `scraper` or `html5ever`
- **CSS Selectors**: Custom parsers support configurable CSS selectors for different HTML structures

### Real-world Implementation

For production use, you would typically:

1. **HTML Parsing**: Use `scraper` crate with CSS selectors
2. **Error Handling**: Robust error handling for malformed HTML
3. **Rate Limiting**: Respect search engine rate limits
4. **User Agent Management**: Proper user agent rotation
5. **Anti-bot Detection**: Handle CAPTCHA and anti-bot measures

Example real implementation structure:
```rust
use scraper::{Html, Selector};

fn parse_real_html(&self, html: &str, limit: usize) -> Result<Vec<SearchResult>> {
    let document = Html::parse_document(html);
    let result_selector = Selector::parse(".result")?;
    let title_selector = Selector::parse("h3 a")?;
    // ... parse using selectors
}
```

## Testing

The parser system includes comprehensive tests covering:
- Individual parser functionality
- Parser factory operations
- Custom parser configuration
- Parser capability validation
- Multiple search engines with different limits

Run tests with:
```bash
cargo test search::parser
```

## Extension Points

### Adding New Search Engines

1. Create a new parser file (e.g., `newsearchengine.rs`)
2. Implement the `SearchResultParser` trait
3. Add the parser to `ParserFactory::get_parser()`
4. Update `SearchEngineType` enum if needed

### Custom CSS Selectors

The `CustomParserConfig` structure allows configuration of:
- Result container selector
- Title element selector  
- URL element selector
- Snippet element selector
- Additional custom rules via HashMap

This provides flexibility for parsing different HTML structures without code changes. 