use super::SearchResultParser;
use crate::Result;
use crate::search::types::{SearchEngineType, SearchResult};
use tracing::info;

pub struct DuckDuckGoParser;

impl DuckDuckGoParser {
    pub fn new() -> Self {
        Self
    }
}

impl SearchResultParser for DuckDuckGoParser {
    fn parse(&self, _html: &str, limit: usize) -> Result<Vec<SearchResult>> {
        info!("Parsing DuckDuckGo search results from HTML");

        let mut results = Vec::new();

        // Mock implementation - in reality, you would parse actual DuckDuckGo HTML structure
        // DuckDuckGo typically uses CSS classes like 'result' for search results

        let mock_results_count = std::cmp::min(limit, 10);

        for i in 0..mock_results_count {
            let rank = i + 1;

            // Mock DuckDuckGo-style results
            let result = SearchResult {
                title: format!("DuckDuckGo Search Result #{} - Mock Title", rank),
                url: format!("https://example-duckduckgo-result-{}.com", rank),
                snippet: format!(
                    "This is a mock DuckDuckGo search result snippet for result {}. In a real implementation, this would be extracted from the HTML using CSS selectors like '.result__snippet' or similar.",
                    rank
                ),
                rank,
            };

            results.push(result);
        }

        info!(
            "Successfully parsed {} DuckDuckGo search results",
            results.len()
        );

        // In a real implementation for DuckDuckGo:
        // - Results container: .result
        // - Title: .result__title a
        // - URL: .result__url
        // - Snippet: .result__snippet

        Ok(results)
    }

    fn name(&self) -> &str {
        "DuckDuckGoParser"
    }

    fn supports(&self, engine_type: &SearchEngineType) -> bool {
        matches!(engine_type, SearchEngineType::DuckDuckGo)
    }
}

impl Default for DuckDuckGoParser {
    fn default() -> Self {
        Self::new()
    }
}
