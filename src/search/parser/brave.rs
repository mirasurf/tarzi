use super::SearchResultParser;
use crate::Result;
use crate::search::types::{SearchEngineType, SearchResult};
use tracing::info;

pub struct BraveParser;

impl BraveParser {
    pub fn new() -> Self {
        Self
    }
}

impl SearchResultParser for BraveParser {
    fn parse(&self, _html: &str, limit: usize) -> Result<Vec<SearchResult>> {
        info!("Parsing Brave search results from HTML");

        let mut results = Vec::new();

        // Mock implementation - in reality, you would parse actual Brave HTML structure
        let mock_results_count = std::cmp::min(limit, 10);

        for i in 0..mock_results_count {
            let rank = i + 1;

            // Mock Brave-style results
            let result = SearchResult {
                title: format!("Brave Search Result #{} - Mock Title", rank),
                url: format!("https://example-brave-result-{}.com", rank),
                snippet: format!(
                    "This is a mock Brave search result snippet for result {}. In a real implementation, this would be extracted from the HTML using appropriate CSS selectors.",
                    rank
                ),
                rank,
            };

            results.push(result);
        }

        info!("Successfully parsed {} Brave search results", results.len());

        Ok(results)
    }

    fn name(&self) -> &str {
        "BraveParser"
    }

    fn supports(&self, engine_type: &SearchEngineType) -> bool {
        matches!(engine_type, SearchEngineType::BraveSearch)
    }
}

impl Default for BraveParser {
    fn default() -> Self {
        Self::new()
    }
}
