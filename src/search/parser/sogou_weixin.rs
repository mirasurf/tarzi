use super::base::{BaseParser, BaseParserImpl};
use crate::Result;
use crate::search::types::{SearchEngineType, SearchResult};
use select::document::Document;
use select::predicate::Name;
use std::collections::HashSet;
use url::Url;

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

    fn parse(&self, html: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let mut results: Vec<SearchResult> = Vec::new();
        if html.trim().is_empty() || limit == 0 {
            return Ok(results);
        }

        // Check for anti-bot CAPTCHA or verification page
        if html.contains("此验证码用于确认")
            || html.contains("验证码：")
            || html.contains("VerifyCode")
        {
            return Err(crate::error::TarziError::Search(
                "Sogou WeChat search detected automated request and is showing CAPTCHA page. \
                This is a common anti-bot measure. Try accessing the site manually first or use a different search engine."
                    .to_string(),
            ));
        }

        let document = Document::from(html);
        let mut seen_urls: HashSet<String> = HashSet::new();

        // Strategy: Sogou Weixin pages often contain direct links to mp.weixin.qq.com articles,
        // or redirect links like weixin.sogou.com/link?url=<encoded mp.weixin.qq.com URL>.
        // We scan all anchors, resolve to a final URL that points to mp.weixin.qq.com when possible,
        // and collect unique results up to the requested limit.
        for anchor in document.find(Name("a")) {
            if results.len() >= limit {
                break;
            }

            let Some(href) = anchor.attr("href") else {
                // Try common data attributes used by Sogou Weixin when href is javascript:void(0)
                // or missing
                let mut candidate: Option<String> = None;
                for key in [
                    "data-share",
                    "data-url",
                    "data-href",
                    "data-shareurl",
                    "data-link",
                ] {
                    if let Some(val) = anchor.attr(key).filter(|v| !v.is_empty()) {
                        candidate = Some(val.to_string());
                        break;
                    }
                }
                if candidate.is_none() {
                    continue;
                }

                let mut resolved_url = resolve_weixin_url(&candidate.unwrap());
                resolved_url = normalize_url(&resolved_url);
                if !is_mp_weixin_url(&resolved_url) {
                    continue;
                }

                if resolved_url.is_empty() || seen_urls.contains(&resolved_url) {
                    continue;
                }

                let mut title = anchor.text().trim().to_string();
                if title.is_empty() {
                    title = anchor.text().trim().to_string();
                }

                let snippet = String::new();

                seen_urls.insert(resolved_url.clone());
                results.push(SearchResult {
                    title,
                    url: resolved_url,
                    snippet,
                    rank: results.len() + 1,
                });
                continue;
            };
            if href.is_empty() {
                continue;
            }

            // Resolve potential sogou redirect to the underlying mp.weixin.qq.com URL
            let mut resolved_url = resolve_weixin_url(href);

            // Normalize common URL forms
            resolved_url = normalize_url(&resolved_url);

            // Accept either direct mp.weixin links or sogou redirect links
            let is_mp = is_mp_weixin_url(&resolved_url);
            let is_redirect = is_sogou_weixin_redirect_url(&resolved_url);
            if !is_mp && !is_redirect {
                continue;
            }

            if resolved_url.is_empty() || seen_urls.contains(&resolved_url) {
                continue;
            }

            let mut title = anchor.text().trim().to_string();
            if title.is_empty() {
                // Fallback: sometimes the anchor has nested elements; use their combined text
                title = anchor.text().trim().to_string();
            }

            let snippet = String::new(); // Snippet is optional; structure varies widely

            seen_urls.insert(resolved_url.clone());
            results.push(SearchResult {
                title,
                url: resolved_url,
                snippet,
                rank: results.len() + 1,
            });
        }

        Ok(results)
    }
}

impl Default for SogouWeixinParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Normalize a URL into an absolute https URL when possible
fn normalize_url(href: &str) -> String {
    if href.starts_with("http://") || href.starts_with("https://") {
        href.to_string()
    } else if href.starts_with("//") {
        format!("https:{href}")
    } else if href.starts_with("/link?") {
        // Relative sogou redirect
        format!("https://weixin.sogou.com{href}")
    } else if href.starts_with('/') {
        // If it's a relative link to mp.weixin.qq.com
        if href.contains("mp.weixin.qq.com") {
            format!("https://mp.weixin.qq.com{href}")
        } else if href.contains("weixin.sogou.com") {
            format!("https://weixin.sogou.com{href}")
        } else {
            href.to_string()
        }
    } else {
        href.to_string()
    }
}

/// Resolve sogou redirect links to the underlying mp.weixin.qq.com URL when present
fn resolve_weixin_url(href: &str) -> String {
    // If it's already an mp.weixin.qq.com link, return as is (strict host check)
    if is_mp_weixin_url(href) {
        return href.to_string();
    }

    // Try to parse as URL and extract the "url" query parameter used by sogou redirector
    // Handle both absolute and relative redirectors like:
    // - https://weixin.sogou.com/link?url=<encoded>
    // - /link?url=<encoded>
    if href.contains("weixin.sogou.com/link") || href.starts_with("/link?") {
        let absolute_href = if href.starts_with("/link?") {
            format!("https://weixin.sogou.com{href}")
        } else {
            href.to_string()
        };

        if let Ok(parsed) = Url::parse(&absolute_href) {
            for (k, v) in parsed.query_pairs() {
                if k == "url" {
                    let inner = v.into_owned();
                    let candidate = match urlencoding::decode(&inner) {
                        Ok(decoded) => decoded.into_owned(),
                        Err(_) => inner,
                    };
                    if is_mp_weixin_url(&candidate) {
                        return candidate;
                    }
                }
            }
        }
        // Manual fallback extraction if standard parsing fails or encoding is unexpected
        if let Some(decoded) = extract_url_param(&absolute_href).filter(|u| is_mp_weixin_url(u)) {
            return decoded;
        }
    }

    href.to_string()
}

fn extract_url_param(raw: &str) -> Option<String> {
    let key = "url=";
    if let Some(pos) = raw.find(key) {
        let rest = &raw[pos + key.len()..];
        let value = match rest.find('&') {
            Some(end) => &rest[..end],
            None => rest,
        };
        if let Ok(decoded) = urlencoding::decode(value) {
            return Some(decoded.into_owned());
        }
        return Some(value.to_string());
    }
    None
}

fn is_mp_weixin_url(url_str: &str) -> bool {
    if url_str.starts_with("//mp.weixin.qq.com") {
        return true;
    }
    if let Ok(u) = Url::parse(url_str) {
        return u.host_str() == Some("mp.weixin.qq.com");
    }
    false
}

fn is_sogou_weixin_redirect_url(url_str: &str) -> bool {
    if url_str.starts_with("/link?") {
        return true;
    }
    if let Ok(u) = Url::parse(url_str) {
        return u.host_str() == Some("weixin.sogou.com") && u.path().starts_with("/link");
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_html_and_zero_limit() {
        let parser = SogouWeixinParser::new();
        let results = parser.parse("", 5).unwrap();
        assert!(results.is_empty());

        let html = "<html><body><a href=\"https://mp.weixin.qq.com/s/abc\">t</a></body></html>";
        let results_zero = parser.parse(html, 0).unwrap();
        assert!(results_zero.is_empty());
    }

    #[test]
    fn test_parse_direct_mp_weixin_links() {
        let parser = SogouWeixinParser::new();
        let html = r#"
            <html><body>
                <div class="result">
                    <a href="https://mp.weixin.qq.com/s/ABCDEFG">Article Title 1</a>
                </div>
                <div class="result">
                    <a href="//mp.weixin.qq.com/s/HIJKLMN"><span>Article</span> Title 2</a>
                </div>
            </body></html>
        "#;

        let results = parser.parse(html, 10).unwrap();
        assert_eq!(results.len(), 2);
        assert!(results[0].url.contains("mp.weixin.qq.com"));
        assert!(results[1].url.contains("mp.weixin.qq.com"));
        assert_eq!(results[0].rank, 1);
        assert_eq!(results[1].rank, 2);
    }

    #[test]
    fn test_parse_sogou_redirect_links() {
        let parser = SogouWeixinParser::new();
        // weixin.sogou.com/link?url=<encoded mp url>
        let encoded = urlencoding::encode("https://mp.weixin.qq.com/s/REDIRECTED");
        let html = format!(
            "<html><body><a href=\"https://weixin.sogou.com/link?url={encoded}\">Redirect</a></body></html>"
        );

        let results = parser.parse(&html, 5).unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].url.contains("mp.weixin.qq.com/s/REDIRECTED"));
    }

    #[test]
    fn test_dedup_and_limit() {
        let parser = SogouWeixinParser::new();
        let html = r#"
            <html><body>
                <a href="https://mp.weixin.qq.com/s/SAME">One</a>
                <a href="https://mp.weixin.qq.com/s/SAME">Duplicate</a>
                <a href="https://mp.weixin.qq.com/s/OTHER">Two</a>
                <a href="https://mp.weixin.qq.com/s/THREE">Three</a>
            </body></html>
        "#;

        let results = parser.parse(html, 2).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].url, "https://mp.weixin.qq.com/s/SAME");
        assert_eq!(results[1].url, "https://mp.weixin.qq.com/s/OTHER");
        assert_eq!(results[0].rank, 1);
        assert_eq!(results[1].rank, 2);
    }

    #[test]
    fn test_ignore_non_weixin_and_relative_links() {
        let parser = SogouWeixinParser::new();
        let html = r#"
            <html><body>
                <a href="/s/RELATIVE">Relative</a>
                <a href="https://weixin.qq.com/s/notmp">Weixin but not mp</a>
                <a href="javascript:void(0)">JS Link</a>
                <a href="mailto:test@example.com">Mail</a>
            </body></html>
        "#;

        let results = parser.parse(html, 10).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_dedup_between_redirect_and_direct() {
        let parser = SogouWeixinParser::new();
        let encoded = urlencoding::encode("https://mp.weixin.qq.com/s/SAME");
        let html = format!(
            r#"
            <html><body>
                <a href="https://weixin.sogou.com/link?url={encoded}">Via Redirect</a>
                <a href="https://mp.weixin.qq.com/s/SAME">Direct</a>
                <a href="//mp.weixin.qq.com/s/OTHER">Other</a>
            </body></html>
            "#
        );

        let results = parser.parse(&html, 10).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].url, "https://mp.weixin.qq.com/s/SAME");
        assert_eq!(results[1].url, "https://mp.weixin.qq.com/s/OTHER");
        assert_eq!(results[0].rank, 1);
        assert_eq!(results[1].rank, 2);
    }

    #[test]
    fn test_protocol_relative_and_title_trim() {
        let parser = SogouWeixinParser::new();
        let html = r#"
            <html><body>
                <a href="//mp.weixin.qq.com/s/PROTO">   Title With Spaces   </a>
            </body></html>
        "#;

        let results = parser.parse(html, 5).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].url, "https://mp.weixin.qq.com/s/PROTO");
        assert_eq!(results[0].title, "Title With Spaces");
        assert_eq!(results[0].rank, 1);
    }

    #[test]
    fn test_limit_applied_with_skipped_items() {
        let parser = SogouWeixinParser::new();
        let html = r#"
            <html><body>
                <a href="https://example.com/irrelevant">Ignore</a>
                <a href="https://mp.weixin.qq.com/s/A1">A1</a>
                <a href="https://not-weixin.example/a2">Ignore</a>
                <a href="https://mp.weixin.qq.com/s/A2">A2</a>
                <a href="https://mp.weixin.qq.com/s/A3">A3</a>
            </body></html>
        "#;

        let results = parser.parse(html, 2).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].url, "https://mp.weixin.qq.com/s/A1");
        assert_eq!(results[1].url, "https://mp.weixin.qq.com/s/A2");
        assert_eq!(results[0].rank, 1);
        assert_eq!(results[1].rank, 2);
    }

    #[test]
    fn test_captcha_detection() {
        let parser = SogouWeixinParser::new();

        // Test Chinese CAPTCHA page
        let captcha_html = r#"
            <html><body>
                <p class="p2">此验证码用于确认这些请求是您的正常行为而不是自动程序发出的，需要您协助验证。</p>
                <p class="p3"><label for="seccodeInput">验证码：</label></p>
            </body></html>
        "#;

        let result = parser.parse(captcha_html, 10);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("CAPTCHA page"));
        assert!(error_msg.contains("anti-bot measure"));

        // Test VerifyCode detection
        let verify_html = r#"
            <html><body>
                <p>VerifyCode：2eb1fd17014e</p>
                <form name="authform">
                    <input type="hidden" name="vc" id="vc" value="">
                </form>
            </body></html>
        "#;

        let result2 = parser.parse(verify_html, 10);
        assert!(result2.is_err());
        let error_msg2 = result2.unwrap_err().to_string();
        assert!(error_msg2.contains("CAPTCHA page"));
    }

    #[test]
    fn test_helpers_normalize_and_resolve() {
        // normalize_url
        assert_eq!(
            normalize_url("https://mp.weixin.qq.com/s/A"),
            "https://mp.weixin.qq.com/s/A"
        );
        assert_eq!(
            normalize_url("//mp.weixin.qq.com/s/A"),
            "https://mp.weixin.qq.com/s/A"
        );
        assert_eq!(normalize_url("/s/B"), "/s/B"); // no host info, keep as-is

        // resolve_weixin_url for direct and redirect
        let direct = resolve_weixin_url("https://mp.weixin.qq.com/s/ABC");
        assert_eq!(direct, "https://mp.weixin.qq.com/s/ABC");

        let encoded = urlencoding::encode("https://mp.weixin.qq.com/s/XYZ");
        let redirected =
            resolve_weixin_url(&format!("https://weixin.sogou.com/link?url={encoded}"));
        assert_eq!(redirected, "https://mp.weixin.qq.com/s/XYZ");
    }
}
