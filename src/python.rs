use crate::{
    Result,
    converter::{Converter, Format},
    fetcher::WebFetcher,
    search::{SearchEngine, SearchMode},
};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pymodule]
fn tarzi(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyConverter>()?;
    m.add_class::<PyWebFetcher>()?;
    m.add_class::<PySearchEngine>()?;
    m.add_class::<PySearchResult>()?;
    m.add_function(wrap_pyfunction!(convert_html, m)?)?;
    m.add_function(wrap_pyfunction!(fetch_url, m)?)?;
    m.add_function(wrap_pyfunction!(search_web, m)?)?;
    Ok(())
}

#[pyclass]
struct PyConverter {
    inner: Converter,
}

#[pymethods]
impl PyConverter {
    #[new]
    fn new() -> Self {
        Self {
            inner: Converter::new(),
        }
    }

    #[pyo3(name = "convert")]
    fn py_convert(&self, input: &str, format: &str) -> PyResult<String> {
        let format = Format::from_str(format)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        rt.block_on(async { self.inner.convert(input, format).await })
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
}

#[pyclass]
struct PyWebFetcher {
    inner: WebFetcher,
}

#[pymethods]
impl PyWebFetcher {
    #[new]
    fn new() -> Self {
        Self {
            inner: WebFetcher::new(),
        }
    }

    #[pyo3(name = "fetch")]
    fn py_fetch(&self, url: &str) -> PyResult<String> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        rt.block_on(async { self.inner.fetch(url).await })
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    #[pyo3(name = "fetch_with_js")]
    fn py_fetch_with_js(&mut self, url: &str) -> PyResult<String> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        rt.block_on(async { self.inner.fetch_with_js(url).await })
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
}

#[pyclass]
struct PySearchEngine {
    inner: SearchEngine,
}

#[pymethods]
impl PySearchEngine {
    #[new]
    fn new() -> Self {
        Self {
            inner: SearchEngine::new(),
        }
    }

    #[pyo3(name = "with_api_key")]
    fn py_with_api_key(
        mut self_: PyRefMut<Self>,
        _py: Python,
        api_key: String,
    ) -> PyResult<PyRefMut<Self>> {
        self_.inner = self_.inner.with_api_key(api_key);
        Ok(self_)
    }

    #[pyo3(name = "search")]
    fn py_search(
        &mut self,
        query: &str,
        mode: &str,
        limit: usize,
    ) -> PyResult<Vec<PySearchResult>> {
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
}

#[pyclass]
#[derive(Clone)]
struct PySearchResult {
    #[pyo3(get)]
    title: String,
    #[pyo3(get)]
    url: String,
    #[pyo3(get)]
    snippet: String,
    #[pyo3(get)]
    rank: usize,
}

#[pyfunction]
fn convert_html(input: &str, format: &str) -> PyResult<String> {
    let converter = Converter::new();
    let format = Format::from_str(format)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

    rt.block_on(async { converter.convert(input, format).await })
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pyfunction]
fn fetch_url(url: &str, js: bool) -> PyResult<String> {
    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

    rt.block_on(async {
        if js {
            let mut fetcher = WebFetcher::new();
            fetcher.fetch_with_js(url).await
        } else {
            let fetcher = WebFetcher::new();
            fetcher.fetch(url).await
        }
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pyfunction]
fn search_web(query: &str, mode: &str, limit: usize) -> PyResult<Vec<PySearchResult>> {
    let mode = SearchMode::from_str(mode)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

    rt.block_on(async {
        let mut search_engine = SearchEngine::new();
        search_engine.search(query, mode, limit).await
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
