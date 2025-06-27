use super::SearchResultParser;
use crate::Result;
use crate::search::types::{SearchEngineType, SearchResult};
use tracing::info;

pub struct SearchApiParser;

impl SearchApiParser {
    pub fn new() -> Self {
        Self
    }
}

impl SearchResultParser for SearchApiParser {
    fn parse(&self, _html: &str, limit: usize) -> Result<Vec<SearchResult>> {
        info!("Parsing SearchApi search results from HTML");

        let mut results = Vec::new();

        // Mock implementation - in reality, you would parse actual SearchApi HTML structure
        let mock_results_count = std::cmp::min(limit, 10);

        for i in 0..mock_results_count {
            let rank = i + 1;

            // Mock SearchApi-style results
            let result = SearchResult {
                title: format!("SearchApi Search Result #{} - Mock Title", rank),
                url: format!("https://example-searchapi-result-{}.com", rank),
                snippet: format!(
                    "This is a mock SearchApi search result snippet for result {}. In a real implementation, this would be extracted from the HTML using appropriate CSS selectors.",
                    rank
                ),
                rank,
            };

            results.push(result);
        }

        info!(
            "Successfully parsed {} SearchApi search results",
            results.len()
        );

        Ok(results)
    }

    fn name(&self) -> &str {
        "SearchApiParser"
    }

    fn supports(&self, engine_type: &SearchEngineType) -> bool {
        matches!(engine_type, SearchEngineType::SearchApi)
    }
}

impl Default for SearchApiParser {
    fn default() -> Self {
        Self::new()
    }
}
