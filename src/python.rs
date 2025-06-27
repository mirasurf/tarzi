#![allow(unsafe_op_in_unsafe_fn)]
#![allow(non_local_definitions)]
use crate::config::Config;
use crate::{Converter, FetchMode, Format, SearchEngine, SearchMode, WebFetcher};
use pyo3::prelude::*;
use pyo3::types::PyType;
use pyo3::wrap_pyfunction;
use std::str::FromStr;
use toml;

#[pymodule]
fn tarzi(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyConverter>()?;
    m.add_class::<PyWebFetcher>()?;
    m.add_class::<PySearchEngine>()?;
    m.add_class::<PySearchResult>()?;
    m.add_class::<PyConfig>()?;
    m.add_function(wrap_pyfunction!(convert_html, m)?)?;
    m.add_function(wrap_pyfunction!(fetch_url, m)?)?;
    m.add_function(wrap_pyfunction!(search_web, m)?)?;
    m.add_function(wrap_pyfunction!(search_and_fetch, m)?)?;
    Ok(())
}

#[pyclass]
pub struct PyConverter {
    inner: Converter,
}

#[allow(non_local_definitions)]
#[pymethods]
impl PyConverter {
    #[new]
    fn new() -> Self {
        Self {
            inner: Converter::new(),
        }
    }

    #[classmethod]
    fn from_config(_cls: &PyType, _config: &PyConfig) -> PyResult<Self> {
        Ok(Self {
            inner: Converter::new(),
        })
    }

    fn convert(&self, input: &str, format: &str) -> PyResult<String> {
        let format = Format::from_str(format)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        rt.block_on(async { self.inner.convert(input, format).await })
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    fn convert_with_config(&self, input: &str, config: &PyConfig) -> PyResult<String> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        rt.block_on(async { self.inner.convert_with_config(input, &config.inner).await })
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
}

#[pyclass]
pub struct PyWebFetcher {
    inner: WebFetcher,
}

#[allow(non_local_definitions)]
#[pymethods]
impl PyWebFetcher {
    #[new]
    fn new() -> Self {
        Self {
            inner: WebFetcher::new(),
        }
    }

    #[classmethod]
    fn from_config(_cls: &PyType, config: &PyConfig) -> PyResult<Self> {
        Ok(Self {
            inner: WebFetcher::from_config(&config.inner),
        })
    }

    fn fetch(&mut self, url: &str, mode: &str, format: &str) -> PyResult<String> {
        let mode = FetchMode::from_str(mode)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let format = Format::from_str(format)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        rt.block_on(async { self.inner.fetch(url, mode, format).await })
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    fn fetch_raw(&mut self, url: &str, mode: &str) -> PyResult<String> {
        let mode = FetchMode::from_str(mode)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        rt.block_on(async { self.inner.fetch_raw(url, mode).await })
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    fn fetch_with_proxy(
        &mut self,
        url: &str,
        proxy: &str,
        mode: &str,
        format: &str,
    ) -> PyResult<String> {
        let mode = FetchMode::from_str(mode)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let format = Format::from_str(format)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        rt.block_on(async { self.inner.fetch_with_proxy(url, proxy, mode, format).await })
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    fn connect_to_external_browser(&mut self, ws_endpoint: &str) -> PyResult<()> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        rt.block_on(async { self.inner.connect_to_external_browser(ws_endpoint).await })
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
}

#[pyclass]
pub struct PySearchEngine {
    inner: SearchEngine,
}

#[allow(non_local_definitions)]
#[pymethods]
impl PySearchEngine {
    #[new]
    fn new() -> Self {
        Self {
            inner: SearchEngine::new(),
        }
    }

    #[classmethod]
    fn from_config(_cls: &PyType, config: &PyConfig) -> PyResult<Self> {
        Ok(Self {
            inner: SearchEngine::from_config(&config.inner),
        })
    }

    fn with_api_key(mut self_: PyRefMut<Self>, api_key: String) -> PyRefMut<Self> {
        let inner = std::mem::take(&mut self_.inner);
        self_.inner = inner.with_api_key(api_key);
        self_
    }

    fn search(&mut self, query: &str, mode: &str, limit: usize) -> PyResult<Vec<PySearchResult>> {
        let mode = SearchMode::from_str(mode)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        rt.block_on(async { self.inner.search(query, mode, limit).await })
            .map(|results| {
                results
                    .into_iter()
                    .map(|r| PySearchResult {
                        title: r.title,
                        url: r.url,
                        snippet: r.snippet,
                        rank: r.rank,
                    })
                    .collect()
            })
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    fn search_and_fetch(
        &mut self,
        query: &str,
        mode: &str,
        limit: usize,
        fetch_mode: &str,
        format: &str,
    ) -> PyResult<Vec<(PySearchResult, String)>> {
        let mode = SearchMode::from_str(mode)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let fetch_mode = FetchMode::from_str(fetch_mode)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let format = Format::from_str(format)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        rt.block_on(async {
            self.inner
                .search_and_fetch(query, mode, limit, fetch_mode, format)
                .await
        })
        .map(|results| {
            results
                .into_iter()
                .map(|(r, content)| {
                    (
                        PySearchResult {
                            title: r.title,
                            url: r.url,
                            snippet: r.snippet,
                            rank: r.rank,
                        },
                        content,
                    )
                })
                .collect()
        })
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    fn search_with_proxy(
        &mut self,
        query: &str,
        mode: &str,
        limit: usize,
        proxy: &str,
    ) -> PyResult<Vec<PySearchResult>> {
        let mode = SearchMode::from_str(mode)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        rt.block_on(async {
            self.inner
                .search_with_proxy(query, mode, limit, proxy)
                .await
        })
        .map(|results| {
            results
                .into_iter()
                .map(|r| PySearchResult {
                    title: r.title,
                    url: r.url,
                    snippet: r.snippet,
                    rank: r.rank,
                })
                .collect()
        })
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    fn cleanup(&mut self) -> PyResult<()> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        rt.block_on(async { self.inner.cleanup().await })
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PySearchResult {
    #[pyo3(get)]
    pub title: String,
    #[pyo3(get)]
    pub url: String,
    #[pyo3(get)]
    pub snippet: String,
    #[pyo3(get)]
    pub rank: usize,
}

#[pyclass]
#[derive(Clone)]
pub struct PyConfig {
    inner: Config,
}

#[allow(non_local_definitions)]
#[pymethods]
impl PyConfig {
    #[new]
    fn new() -> Self {
        Self {
            inner: Config::new(),
        }
    }

    #[classmethod]
    fn from_file(_cls: &PyType, _path: &str) -> PyResult<Self> {
        let config = Config::load()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(Self { inner: config })
    }

    #[classmethod]
    fn from_str(_cls: &PyType, content: &str) -> PyResult<Self> {
        let config: Config = toml::from_str(content)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(Self { inner: config })
    }

    fn save(&self) -> PyResult<()> {
        self.inner
            .save()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    fn save_dev(&self) -> PyResult<()> {
        self.inner
            .save_dev()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }
}

// Helper functions for direct Python usage
#[pyfunction]
fn convert_html(html: &str, format: &str) -> PyResult<String> {
    let converter = PyConverter::new();
    converter.convert(html, format)
}

#[pyfunction]
fn fetch_url(url: &str, mode: &str, format: &str) -> PyResult<String> {
    let mut fetcher = PyWebFetcher::new();
    fetcher.fetch(url, mode, format)
}

#[pyfunction]
fn search_web(query: &str, mode: &str, limit: usize) -> PyResult<Vec<PySearchResult>> {
    let mut engine = PySearchEngine::new();
    engine.search(query, mode, limit)
}

#[pyfunction]
fn search_and_fetch(
    query: &str,
    mode: &str,
    limit: usize,
    fetch_mode: &str,
    format: &str,
) -> PyResult<Vec<(PySearchResult, String)>> {
    let mut engine = PySearchEngine::new();
    engine.search_and_fetch(query, mode, limit, fetch_mode, format)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_py_converter_new() {
        let converter = PyConverter::new();
        assert_eq!(converter.inner, Converter::new());
    }

    #[test]
    fn test_py_converter_convert_html() {
        let converter = PyConverter::new();
        let html = "<h1>Test</h1>";
        let result = converter.convert(html, "html").unwrap();
        assert_eq!(result, html);
    }

    #[test]
    fn test_py_converter_convert_markdown() {
        let converter = PyConverter::new();
        let html = "<h1>Test</h1>";
        let result = converter.convert(html, "markdown").unwrap();
        assert!(result.contains("# Test"));
    }

    #[test]
    fn test_py_converter_convert_json() {
        let converter = PyConverter::new();
        let html = "<h1>Test</h1><p>Content</p>";
        let result = converter.convert(html, "json").unwrap();
        assert!(result.contains("Test"));
        assert!(result.contains("Content"));
    }

    #[test]
    fn test_py_converter_convert_yaml() {
        let converter = PyConverter::new();
        let html = "<h1>Test</h1><p>Content</p>";
        let result = converter.convert(html, "yaml").unwrap();
        assert!(result.contains("Test"));
        assert!(result.contains("Content"));
    }

    #[test]
    fn test_py_converter_invalid_format() {
        let converter = PyConverter::new();
        let html = "<h1>Test</h1>";
        let result = converter.convert(html, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_py_webfetcher_new() {
        let _fetcher = PyWebFetcher::new();
        // Just test that it can be created without panicking
    }

    #[test]
    fn test_py_webfetcher_from_config() {
        let config = PyConfig::new();
        // Just test that it can be created without panicking
        let _fetcher = PyWebFetcher {
            inner: WebFetcher::from_config(&config.inner),
        };
    }

    #[test]
    fn test_py_searchengine_new() {
        let _engine = PySearchEngine::new();
        // Just test that it can be created without panicking
    }

    #[test]
    fn test_py_searchengine_from_config() {
        let config = PyConfig::new();
        // Just test that it can be created without panicking
        let _engine = PySearchEngine {
            inner: SearchEngine::from_config(&config.inner),
        };
    }

    #[test]
    fn test_py_search_result() {
        let result = PySearchResult {
            title: "Test Title".to_string(),
            url: "https://example.com".to_string(),
            snippet: "Test snippet".to_string(),
            rank: 1,
        };
        assert_eq!(result.title, "Test Title");
        assert_eq!(result.url, "https://example.com");
        assert_eq!(result.snippet, "Test snippet");
        assert_eq!(result.rank, 1);
    }

    #[test]
    fn test_py_config_new() {
        let _config = PyConfig::new();
        // Just test that it can be created without panicking
    }

    #[test]
    fn test_py_config_from_str() {
        let config_str = r#"
[fetcher]
timeout = 30
user_agent = "Test Agent"
format = "html"
proxy = ""

[search]
engine = "bing"
api_key = ""
query_pattern = "https://www.bing.com/search?q={query}"
"#;
        let config: Config = toml::from_str(config_str).unwrap();
        assert_eq!(config.fetcher.user_agent, "Test Agent");
        assert_eq!(config.fetcher.timeout, 30);
    }

    #[test]
    fn test_py_config_from_str_invalid() {
        let config_str = "invalid toml content";
        let result = toml::from_str::<Config>(config_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_html_function() {
        let html = "<h1>Test</h1>";
        let result = convert_html(html, "html").unwrap();
        assert_eq!(result, html);
    }

    #[test]
    fn test_convert_html_function_invalid_format() {
        let html = "<h1>Test</h1>";
        let result = convert_html(html, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_fetch_url_function_invalid_mode() {
        let result = fetch_url("https://example.com", "invalid", "html");
        assert!(result.is_err());
    }

    #[test]
    fn test_fetch_url_function_invalid_format() {
        let result = fetch_url("https://example.com", "plain_request", "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_search_web_function_invalid_mode() {
        let result = search_web("test query", "invalid", 5);
        assert!(result.is_err());
    }

    #[test]
    fn test_search_and_fetch_function_invalid_mode() {
        let result = search_and_fetch("test query", "invalid", 5, "plain_request", "html");
        assert!(result.is_err());
    }

    #[test]
    fn test_search_and_fetch_function_invalid_fetch_mode() {
        let result = search_and_fetch("test query", "webquery", 5, "invalid", "html");
        assert!(result.is_err());
    }

    #[test]
    fn test_search_and_fetch_function_invalid_format() {
        let result = search_and_fetch("test query", "webquery", 5, "plain_request", "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_py_search_result_clone() {
        let result = PySearchResult {
            title: "Test Title".to_string(),
            url: "https://example.com".to_string(),
            snippet: "Test snippet".to_string(),
            rank: 1,
        };
        let cloned = result.clone();
        assert_eq!(result.title, cloned.title);
        assert_eq!(result.url, cloned.url);
        assert_eq!(result.snippet, cloned.snippet);
        assert_eq!(result.rank, cloned.rank);
    }
}
