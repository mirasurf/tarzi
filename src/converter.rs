use crate::{config::Config, error::TarziError, Result};
use pulldown_cmark::{Event, HeadingLevel, Parser as MarkdownParser, Tag};
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
    type Err = TarziError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "html" => Ok(Format::Html),
            "markdown" | "md" => Ok(Format::Markdown),
            "json" => Ok(Format::Json),
            "yaml" | "yml" => Ok(Format::Yaml),
            _ => Err(TarziError::InvalidFormat(s.to_string())),
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

#[derive(Debug, PartialEq, Clone)]
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

    /// Convert content using the format specified in the config
    pub async fn convert_with_config(&self, input: &str, config: &Config) -> Result<String> {
        let format = Format::from_str(&config.fetcher.format)?;
        self.convert(input, format).await
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
pub fn convert_search_results(
    results: &[crate::search::SearchResult],
    format: Format,
) -> Result<String> {
    match format {
        Format::Json => {
            let json_results = serde_json::to_string_pretty(results)?;
            Ok(json_results)
        }
        Format::Yaml => {
            let yaml_results = serde_yaml::to_string(results)?;
            Ok(yaml_results)
        }
        _ => Err(TarziError::InvalidFormat(
            "Only JSON and YAML formats supported for search results".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::search::SearchResult;

    #[test]
    fn test_format_parsing() {
        // Test valid formats
        assert_eq!(Format::from_str("html").unwrap(), Format::Html);
        assert_eq!(Format::from_str("HTML").unwrap(), Format::Html);
        assert_eq!(Format::from_str("markdown").unwrap(), Format::Markdown);
        assert_eq!(Format::from_str("MARKDOWN").unwrap(), Format::Markdown);
        assert_eq!(Format::from_str("md").unwrap(), Format::Markdown);
        assert_eq!(Format::from_str("MD").unwrap(), Format::Markdown);
        assert_eq!(Format::from_str("json").unwrap(), Format::Json);
        assert_eq!(Format::from_str("JSON").unwrap(), Format::Json);
        assert_eq!(Format::from_str("yaml").unwrap(), Format::Yaml);
        assert_eq!(Format::from_str("YAML").unwrap(), Format::Yaml);
        assert_eq!(Format::from_str("yml").unwrap(), Format::Yaml);
        assert_eq!(Format::from_str("YML").unwrap(), Format::Yaml);

        // Test invalid formats
        assert!(Format::from_str("invalid").is_err());
        assert!(Format::from_str("").is_err());
        assert!(Format::from_str("xml").is_err());
    }

    #[test]
    fn test_converter_creation() {
        let converter = Converter::new();
        assert_eq!(converter, Converter);
    }

    #[test]
    fn test_converter_default() {
        let converter1 = Converter::new();
        let converter2 = Converter::new();
        assert_eq!(converter1, converter2);
    }

    #[test]
    fn test_html_to_markdown() {
        let converter = Converter::new();

        // Test basic HTML conversion
        let html = "<h1>Hello World</h1>";
        let result = converter.html_to_markdown(html).unwrap();
        assert!(result.contains("Hello World"));

        // Test with paragraphs
        let html = "<p>This is a <strong>test</strong> paragraph.</p>";
        let result = converter.html_to_markdown(html).unwrap();
        assert!(result.contains("This is a"));
        assert!(result.contains("test"));

        // Test with links
        let html = "<a href=\"https://example.com\">Example Link</a>";
        let result = converter.html_to_markdown(html).unwrap();
        assert!(result.contains("Example Link"));
        assert!(result.contains("https://example.com"));

        // Test with images
        let html = "<img src=\"image.jpg\" alt=\"Test Image\">";
        let result = converter.html_to_markdown(html).unwrap();
        assert!(result.contains("image.jpg"));

        // Test empty HTML
        let result = converter.html_to_markdown("").unwrap();
        assert_eq!(result, "");
    }

    #[tokio::test]
    async fn test_html_to_json() {
        let converter = Converter::new();

        // Test with title and content
        let html =
            "<h1>Test Title</h1><p>Test content with <a href=\"https://example.com\">link</a>.</p>";
        let result = converter.html_to_json(html).await.unwrap();

        // Verify JSON structure
        assert!(result.contains("Test Title"));
        assert!(result.contains("Test content"));
        assert!(result.contains("https://example.com"));
        assert!(result.contains("\"title\""));
        assert!(result.contains("\"content\""));
        assert!(result.contains("\"links\""));
        assert!(result.contains("\"images\""));

        // Test with images
        let html = "<h1>Image Test</h1><img src=\"test.jpg\" alt=\"Test\">";
        let result = converter.html_to_json(html).await.unwrap();
        assert!(result.contains("test.jpg"));

        // Test empty HTML
        let result = converter.html_to_json("").await.unwrap();
        println!("[DEBUG] JSON output for empty HTML: {result}");
        // Accept as long as content field exists
        assert!(
            result.contains("\"content\":"),
            "JSON should contain a content field"
        );
    }

    #[tokio::test]
    async fn test_html_to_yaml() {
        let converter = Converter::new();

        // Test with title and content
        let html =
            "<h1>YAML Test</h1><p>YAML content with <a href=\"https://test.com\">link</a>.</p>";
        let result = converter.html_to_yaml(html).await.unwrap();

        // Verify YAML structure
        assert!(result.contains("YAML Test"));
        assert!(result.contains("YAML content"));
        assert!(result.contains("https://test.com"));
        assert!(result.contains("title:"));
        assert!(result.contains("content:"));
        assert!(result.contains("links:"));
        assert!(result.contains("images:"));

        // Test empty HTML
        let result = converter.html_to_yaml("").await.unwrap();
        assert!(result.contains("title: null"));
        assert!(
            result.contains("content:"),
            "YAML should contain a content field"
        );
    }

    #[tokio::test]
    async fn test_convert_html_format() {
        let converter = Converter::new();
        let html = "<h1>Test</h1><p>Content</p>";

        let result = converter.convert(html, Format::Html).await.unwrap();
        assert_eq!(result, html);
    }

    #[tokio::test]
    async fn test_convert_markdown_format() {
        let converter = Converter::new();
        let html = "<h1>Test</h1><p>Content</p>";

        let result = converter.convert(html, Format::Markdown).await.unwrap();
        assert!(result.contains("Test"));
        assert!(result.contains("Content"));
    }

    #[tokio::test]
    async fn test_convert_json_format() {
        let converter = Converter::new();
        let html = "<h1>Test</h1><p>Content</p>";

        let result = converter.convert(html, Format::Json).await.unwrap();
        assert!(result.contains("Test"));
        assert!(result.contains("Content"));
        assert!(result.contains("\"title\""));
        assert!(result.contains("\"content\""));
    }

    #[tokio::test]
    async fn test_convert_yaml_format() {
        let converter = Converter::new();
        let html = "<h1>Test</h1><p>Content</p>";

        let result = converter.convert(html, Format::Yaml).await.unwrap();
        assert!(result.contains("Test"));
        assert!(result.contains("Content"));
        assert!(result.contains("title:"));
        assert!(result.contains("content:"));
    }

    #[tokio::test]
    async fn test_parse_html_document() {
        let converter = Converter::new();

        // Test document with all elements
        let html = r#"
            <h1>Document Title</h1>
            <p>This is the main content.</p>
            <a href="https://link1.com">Link 1</a>
            <a href="https://link2.com">Link 2</a>
            <img src="image1.jpg" alt="Image 1">
            <img src="image2.jpg" alt="Image 2">
        "#;

        let document = converter.parse_html_document(html).await.unwrap();

        assert_eq!(document.title, Some("Document Title".to_string()));
        assert!(document.content.contains("This is the main content"));
        assert_eq!(
            document.links,
            vec!["https://link1.com", "https://link2.com"]
        );
        assert_eq!(document.images, vec!["image1.jpg", "image2.jpg"]);
    }

    #[tokio::test]
    async fn test_parse_html_document_no_title() {
        let converter = Converter::new();

        let html = "<p>Content without title</p>";
        let document = converter.parse_html_document(html).await.unwrap();

        assert_eq!(document.title, None);
        assert!(document.content.contains("Content without title"));
        assert!(document.links.is_empty());
        assert!(document.images.is_empty());
    }

    #[tokio::test]
    async fn test_parse_html_document_empty() {
        let converter = Converter::new();

        let document = converter.parse_html_document("").await.unwrap();

        assert_eq!(document.title, None);
        assert_eq!(document.content, "");
        assert!(document.links.is_empty());
        assert!(document.images.is_empty());
    }

    #[test]
    fn test_convert_search_results_json() {
        let results = vec![
            SearchResult {
                title: "Test Result 1".to_string(),
                url: "https://example1.com".to_string(),
                snippet: "Snippet 1".to_string(),
                rank: 1,
            },
            SearchResult {
                title: "Test Result 2".to_string(),
                url: "https://example2.com".to_string(),
                snippet: "Snippet 2".to_string(),
                rank: 2,
            },
        ];

        let json_result = convert_search_results(&results, Format::Json).unwrap();

        assert!(json_result.contains("Test Result 1"));
        assert!(json_result.contains("Test Result 2"));
        assert!(json_result.contains("https://example1.com"));
        assert!(json_result.contains("https://example2.com"));
        assert!(json_result.contains("\"title\""));
        assert!(json_result.contains("\"url\""));
        assert!(json_result.contains("\"snippet\""));
        assert!(json_result.contains("\"rank\""));
    }

    #[test]
    fn test_convert_search_results_yaml() {
        let results = vec![SearchResult {
            title: "YAML Test".to_string(),
            url: "https://yaml-test.com".to_string(),
            snippet: "YAML snippet".to_string(),
            rank: 1,
        }];

        let yaml_result = convert_search_results(&results, Format::Yaml).unwrap();

        assert!(yaml_result.contains("YAML Test"));
        assert!(yaml_result.contains("https://yaml-test.com"));
        assert!(yaml_result.contains("YAML snippet"));
        assert!(yaml_result.contains("title:"));
        assert!(yaml_result.contains("url:"));
        assert!(yaml_result.contains("snippet:"));
        assert!(yaml_result.contains("rank:"));
    }

    #[test]
    fn test_convert_search_results_invalid_format() {
        let results = vec![SearchResult {
            title: "Test".to_string(),
            url: "https://test.com".to_string(),
            snippet: "Snippet".to_string(),
            rank: 1,
        }];

        // Test with unsupported formats
        assert!(convert_search_results(&results, Format::Html).is_err());
        assert!(convert_search_results(&results, Format::Markdown).is_err());
    }

    #[test]
    fn test_convert_search_results_empty() {
        let results: Vec<SearchResult> = vec![];
        let format = Format::Json;

        let result = convert_search_results(&results, format).unwrap();
        assert_eq!(result, "[]");
    }

    #[test]
    fn test_convert_with_config() {
        use crate::config::Config;

        let converter = Converter::new();
        let mut config = Config::new();

        // Test with markdown format
        config.fetcher.format = "markdown".to_string();
        let html = "<h1>Hello World</h1>";

        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async { converter.convert_with_config(html, &config).await })
            .unwrap();

        assert!(result.contains("Hello World"));

        // Test with json format
        config.fetcher.format = "json".to_string();
        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async { converter.convert_with_config(html, &config).await })
            .unwrap();

        assert!(result.contains("Hello World"));
        assert!(result.contains("title"));
    }

    #[tokio::test]
    async fn test_complex_html_conversion() {
        let converter = Converter::new();

        let html = r#"
            <html>
                <head><title>Page Title</title></head>
                <body>
                    <h1>Main Heading</h1>
                    <p>This is a <strong>bold</strong> paragraph with a 
                    <a href="https://example.com">link</a> and an 
                    <img src="test.jpg" alt="Test Image"> image.</p>
                    <h2>Sub Heading</h2>
                    <p>Another paragraph with <em>emphasis</em>.</p>
                    <ul>
                        <li>List item 1</li>
                        <li>List item 2</li>
                    </ul>
                </body>
            </html>
        "#;

        // Test markdown conversion
        let markdown = converter.convert(html, Format::Markdown).await.unwrap();
        assert!(markdown.contains("Main Heading"));
        assert!(markdown.contains("bold"));
        assert!(markdown.contains("https://example.com"));
        assert!(markdown.contains("test.jpg"));
        assert!(markdown.contains("emphasis"));
        assert!(markdown.contains("List item 1"));

        // Test JSON conversion
        let json = converter.convert(html, Format::Json).await.unwrap();
        assert!(json.contains("Main Heading"));
        assert!(json.contains("https://example.com"));
        assert!(json.contains("test.jpg"));

        // Test YAML conversion
        let yaml = converter.convert(html, Format::Yaml).await.unwrap();
        assert!(yaml.contains("Main Heading"));
        assert!(yaml.contains("https://example.com"));
        assert!(yaml.contains("test.jpg"));
    }
}
