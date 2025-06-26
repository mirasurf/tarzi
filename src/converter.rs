use crate::{error::TarsierError, Result};
use pulldown_cmark::{Event, Parser as MarkdownParser, Tag, HeadingLevel};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Format {
    Html,
    Markdown,
    Json,
    Yaml,
}

impl FromStr for Format {
    type Err = TarsierError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "html" => Ok(Format::Html),
            "markdown" | "md" => Ok(Format::Markdown),
            "json" => Ok(Format::Json),
            "yaml" | "yml" => Ok(Format::Yaml),
            _ => Err(TarsierError::InvalidFormat(s.to_string())),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub title: Option<String>,
    pub content: String,
    pub links: Vec<String>,
    pub images: Vec<String>,
}

pub struct Converter;

impl Converter {
    pub fn new() -> Self {
        Self
    }

    pub async fn convert(&self, input: &str, format: Format) -> Result<String> {
        match format {
            Format::Html => Ok(input.to_string()),
            Format::Markdown => self.html_to_markdown(input),
            Format::Json => self.html_to_json(input).await,
            Format::Yaml => self.html_to_yaml(input).await,
        }
    }

    fn html_to_markdown(&self, html: &str) -> Result<String> {
        let markdown = html2md::parse_html(html);
        Ok(markdown)
    }

    async fn html_to_json(&self, html: &str) -> Result<String> {
        let document = self.parse_html_document(html).await?;
        let json = serde_json::to_string_pretty(&document)?;
        Ok(json)
    }

    async fn html_to_yaml(&self, html: &str) -> Result<String> {
        let document = self.parse_html_document(html).await?;
        let yaml = serde_yaml::to_string(&document)?;
        Ok(yaml)
    }

    async fn parse_html_document(&self, html: &str) -> Result<Document> {
        // First convert to markdown
        let markdown = self.html_to_markdown(html)?;
        
        // Parse markdown to extract structured data
        let mut title = None;
        let mut content = String::new();
        let mut links = Vec::new();
        let mut images = Vec::new();

        let parser = MarkdownParser::new(&markdown);
        let mut in_title = false;

        for event in parser {
            match event {
                Event::Start(Tag::Heading(HeadingLevel::H1, ..)) => {
                    in_title = true;
                }
                Event::Text(text) => {
                    if in_title {
                        title = Some(text.to_string());
                    } else {
                        content.push_str(&text);
                        content.push(' ');
                    }
                }
                Event::End(Tag::Heading(HeadingLevel::H1, ..)) => {
                    in_title = false;
                }
                Event::Start(Tag::Link(_, url, _)) => {
                    links.push(url.to_string());
                }
                Event::Start(Tag::Image(_, url, _)) => {
                    images.push(url.to_string());
                }
                Event::End(Tag::Paragraph) => {
                    content.push('\n');
                }
                _ => {}
            }
        }

        Ok(Document {
            title,
            content: content.trim().to_string(),
            links,
            images,
        })
    }
}

impl Default for Converter {
    fn default() -> Self {
        Self::new()
    }
}

// Helper function for converting search results
pub fn convert_search_results(results: &[crate::search::SearchResult], format: Format) -> Result<String> {
    match format {
        Format::Json => {
            let json_results = serde_json::to_string_pretty(results)?;
            Ok(json_results)
        }
        Format::Yaml => {
            let yaml_results = serde_yaml::to_string(results)?;
            Ok(yaml_results)
        }
        _ => Err(TarsierError::InvalidFormat("Only JSON and YAML formats supported for search results".to_string())),
    }
} 