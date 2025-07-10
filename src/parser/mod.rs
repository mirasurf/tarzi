use crate::search::types::{SearchEngineType, SearchMode, SearchResult};
use crate::Result;

pub mod baidu;
pub mod base;
pub mod bing;
pub mod brave;
pub mod custom;
pub mod duckduckgo;
pub mod google;

#[cfg(test)]
mod tests;

pub use baidu::{BaiduApiParser, BaiduParser};
pub use bing::BingParser;
pub use brave::{BraveApiParser, BraveParser};
pub use custom::{CustomParser, CustomParserConfig, ExaApiParser, TravilyApiParser};
pub use duckduckgo::{DuckDuckGoApiParser, DuckDuckGoParser};
pub use google::{GoogleApiParser, GoogleParser, GoogleSerperApiParser};

use base::{ApiSearchParser, BaseSearchParser, WebSearchParser};

/// Unified parser that can handle both web and API queries
pub struct UnifiedParser {
    web_parser: Box<dyn WebSearchParser>,
    api_parser: Option<Box<dyn ApiSearchParser>>,
}

impl UnifiedParser {
    pub fn new(
        web_parser: Box<dyn WebSearchParser>,
        api_parser: Option<Box<dyn ApiSearchParser>>,
    ) -> Self {
        Self {
            web_parser,
            api_parser,
        }
    }

    pub fn web_only(web_parser: Box<dyn WebSearchParser>) -> Self {
        Self {
            web_parser,
            api_parser: None,
        }
    }

    pub fn api_only(api_parser: Box<dyn ApiSearchParser>) -> Self {
        Self {
            web_parser: Box::new(DummyWebParser::new()),
            api_parser: Some(api_parser),
        }
    }
}

impl SearchResultParser for UnifiedParser {
    fn parse(&self, content: &str, limit: usize) -> Result<Vec<SearchResult>> {
        // Try to detect if content is JSON or HTML
        if content.trim().starts_with('{') || content.trim().starts_with('[') {
            // Likely JSON content
            if let Some(ref api_parser) = self.api_parser {
                api_parser.parse_json(content, limit)
            } else {
                // Fallback to web parser if no API parser available
                self.web_parser.parse_html(content, limit)
            }
        } else {
            // Likely HTML content
            self.web_parser.parse_html(content, limit)
        }
    }

    fn name(&self) -> &str {
        self.web_parser.name()
    }

    fn supports(&self, engine_type: &SearchEngineType) -> bool {
        self.web_parser.supports(engine_type)
            || self
                .api_parser
                .as_ref()
                .is_some_and(|p| p.supports(engine_type))
    }
}

/// Dummy web parser for API-only engines
struct DummyWebParser {
    name: String,
    engine_type: SearchEngineType,
}

impl DummyWebParser {
    fn new() -> Self {
        Self {
            name: "DummyWebParser".to_string(),
            engine_type: SearchEngineType::Custom("dummy".to_string()),
        }
    }
}

impl BaseSearchParser for DummyWebParser {
    fn name(&self) -> &str {
        &self.name
    }

    fn engine_type(&self) -> SearchEngineType {
        self.engine_type.clone()
    }
}

impl WebSearchParser for DummyWebParser {
    fn parse_html(&self, _html: &str, _limit: usize) -> Result<Vec<SearchResult>> {
        Ok(Vec::new()) // Return empty results for API-only engines
    }
}

/// Trait for parsing search results from HTML content (legacy compatibility)
pub trait SearchResultParser: Send + Sync {
    /// Parse search results from HTML content
    fn parse(&self, html: &str, limit: usize) -> Result<Vec<SearchResult>>;

    /// Get the name of the parser
    fn name(&self) -> &str;

    /// Check if this parser can handle the given search engine type
    fn supports(&self, engine_type: &SearchEngineType) -> bool;
}

/// Factory for creating parsers based on search engine type
pub struct ParserFactory {
    custom_parsers: std::collections::HashMap<String, Box<dyn SearchResultParser>>,
}

impl ParserFactory {
    pub fn new() -> Self {
        Self {
            custom_parsers: std::collections::HashMap::new(),
        }
    }

    /// Register a custom parser
    pub fn register_custom_parser(&mut self, name: String, parser: Box<dyn SearchResultParser>) {
        self.custom_parsers.insert(name, parser);
    }

    /// Get a parser for the given search engine type and mode
    pub fn get_parser(
        &self,
        engine_type: &SearchEngineType,
        mode: SearchMode,
    ) -> Box<dyn SearchResultParser> {
        match (engine_type, mode) {
            // Web query parsers (HTML-based)
            (SearchEngineType::Bing, SearchMode::WebQuery) => Box::new(BingParser::new()),
            (SearchEngineType::DuckDuckGo, SearchMode::WebQuery) => {
                Box::new(DuckDuckGoParser::new())
            }
            (SearchEngineType::Google, SearchMode::WebQuery) => Box::new(GoogleParser::new()),
            (SearchEngineType::BraveSearch, SearchMode::WebQuery) => Box::new(BraveParser::new()),
            (SearchEngineType::Baidu, SearchMode::WebQuery) => Box::new(BaiduParser::new()),

            // API query parsers (JSON-based)
            (SearchEngineType::DuckDuckGo, SearchMode::ApiQuery) => {
                Box::new(DuckDuckGoApiParser::new())
            }
            (SearchEngineType::Google, SearchMode::ApiQuery) => Box::new(GoogleApiParser::new()),
            (SearchEngineType::BraveSearch, SearchMode::ApiQuery) => {
                Box::new(BraveApiParser::new())
            }
            (SearchEngineType::Baidu, SearchMode::ApiQuery) => Box::new(BaiduApiParser::new()),
            (SearchEngineType::Exa, SearchMode::ApiQuery) => Box::new(ExaApiParser::new()),
            (SearchEngineType::Travily, SearchMode::ApiQuery) => Box::new(TravilyApiParser::new()),
            (SearchEngineType::GoogleSerper, SearchMode::ApiQuery) => {
                Box::new(GoogleSerperApiParser::new())
            }

            // Fallback for unsupported combinations
            (SearchEngineType::Bing, SearchMode::ApiQuery) => {
                // Bing doesn't support API queries, but we'll provide a fallback parser
                Box::new(CustomParser::new("BingApiFallback".to_string()))
            }
            (SearchEngineType::Exa, SearchMode::WebQuery) => {
                // Exa is API-only, but we'll provide a fallback parser
                Box::new(CustomParser::new("ExaWebFallback".to_string()))
            }
            (SearchEngineType::Travily, SearchMode::WebQuery) => {
                // Travily is API-only, but we'll provide a fallback parser
                Box::new(CustomParser::new("TravilyWebFallback".to_string()))
            }
            (SearchEngineType::GoogleSerper, SearchMode::WebQuery) => {
                // GoogleSerper is API-only, but we'll provide a fallback parser
                Box::new(CustomParser::new("GoogleSerperWebFallback".to_string()))
            }

            // Custom engine parsers
            (SearchEngineType::Custom(name), _) => {
                if let Some(_parser) = self.custom_parsers.get(name) {
                    Box::new(CustomParser::new(name.clone()))
                } else {
                    Box::new(CustomParser::new(name.clone()))
                }
            }
        }
    }

    /// Get a parser for the given search engine type (legacy method for backward compatibility)
    pub fn get_parser_legacy(&self, engine_type: &SearchEngineType) -> Box<dyn SearchResultParser> {
        // Default to web query mode for backward compatibility
        self.get_parser(engine_type, SearchMode::WebQuery)
    }
}

impl Default for ParserFactory {
    fn default() -> Self {
        Self::new()
    }
}
