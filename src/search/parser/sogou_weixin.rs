use super::base::{BaseParser, BaseParserImpl};
use crate::Result;
use crate::search::types::{SearchEngineType, SearchResult};

pub struct SogouWeixinParser {
    base: BaseParserImpl,
}

impl SogouWeixinParser {
    pub fn new() -> Self {
        Self {
            base: BaseParserImpl::new(
                "SogouWeixinParser".to_string(),
                SearchEngineType::SougouWeixin,
            ),
        }
    }
}

impl BaseParser for SogouWeixinParser {
    fn name(&self) -> &str {
        self.base.name()
    }
    fn engine_type(&self) -> SearchEngineType {
        self.base.engine_type()
    }

    fn parse(&self, _html: &str, _limit: usize) -> Result<Vec<SearchResult>> {
        let results = Vec::new();
        Ok(results)
    }
}

impl Default for SogouWeixinParser {
    fn default() -> Self {
        Self::new()
    }
}
