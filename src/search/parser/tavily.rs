use crate::Result;
use super::SearchResultParser;
use crate::search::types::{SearchEngineType, SearchResult};
use tracing::{info, warn};

pub struct TavilyParser;

impl TavilyParser {
    pub fn new() -> Self {
        Self
    }
}

impl SearchResultParser for TavilyParser {
    fn parse(&self, html: &str, limit: usize) -> Result<Vec<SearchResult>> {
        info!("Parsing Tavily search results from HTML");
        
        let mut results = Vec::new();
        
        // Mock implementation - in reality, you would parse actual Tavily HTML structure
        let mock_results_count = std::cmp::min(limit, 10);
        
        for i in 0..mock_results_count {
            let rank = i + 1;
            
            // Mock Tavily-style results
            let result = SearchResult {
                title: format!("Tavily Search Result #{} - Mock Title", rank),
                url: format!("https://example-tavily-result-{}.com", rank),
                snippet: format!("This is a mock Tavily search result snippet for result {}. In a real implementation, this would be extracted from the HTML using appropriate CSS selectors.", rank),
                rank,
            };
            
            results.push(result);
        }
        
        info!("Successfully parsed {} Tavily search results", results.len());
        
        Ok(results)
    }
    
    fn name(&self) -> &str {
        "TavilyParser"
    }
    
    fn supports(&self, engine_type: &SearchEngineType) -> bool {
        matches!(engine_type, SearchEngineType::Tavily)
    }
}

impl Default for TavilyParser {
    fn default() -> Self {
        Self::new()
    }
} 