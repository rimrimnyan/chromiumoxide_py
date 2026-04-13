pub mod config;
pub use config::{PyBrowserConfig, PyBrowserConfigBuilder};

use crate::helper::{call_fut, runtime, to_py_err};
use crate::page::PyPage;

use futures::StreamExt;
use std::sync::Arc;

use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3_stub_gen::{define_stub_info_gatherer, derive::*};

use chromiumoxide::browser::BrowserConfigBuilder;
use chromiumoxide::handler::HandlerConfig;
use chromiumoxide::{Browser, BrowserConfig};

#[gen_stub_pyclass(module = "chromiumoxide_py.bindings.browser")]
#[pyclass(name = "Browser")]
pub struct PyBrowser {
    inner: Arc<tokio::sync::Mutex<Browser>>,
    _handler: Arc<tokio::task::JoinHandle<()>>,
}

#[gen_stub_pymethods(module = "chromiumoxide_py.bindings.browser")]
#[pymethods]
impl PyBrowser {
    #[staticmethod]
    fn launch(config: PyBrowserConfig) -> PyResult<Self> {
        let owned = config.inner.to_owned();

        let rt = runtime();
        let (browser, mut handler) =
            call_fut(chromiumoxide::Browser::launch(owned)).map_err(to_py_err)?;

        // Drive the internal WebSocket/CDP handler in a background task.
        let handle = rt.spawn(async move { while let Some(_event) = handler.next().await {} });

        Ok(PyBrowser {
            inner: Arc::new(tokio::sync::Mutex::new(browser)),
            _handler: Arc::new(handle),
        })
    }

    #[staticmethod]
    fn connect(url: String) -> PyResult<Self> {
        let rt = runtime();

        let (browser, mut handler) =
            call_fut(Browser::connect_with_config(url, HandlerConfig::default()))
                .map_err(to_py_err)?;

        let handle = rt.spawn(async move { while let Some(_event) = handler.next().await {} });

        Ok(PyBrowser {
            inner: Arc::new(tokio::sync::Mutex::new(browser)),
            _handler: Arc::new(handle),
        })
    }

    #[getter]
    fn websocket_address(&self) -> PyResult<String> {
        let b = call_fut(self.inner.lock());
        Ok(b.websocket_address().clone())
    }

    #[getter]
    fn pages(&self) -> PyResult<Vec<PyPage>> {
        let b = call_fut(self.inner.lock());
        let pages = call_fut(b.pages())
            .map_err(to_py_err)?
            .into_iter()
            .map(|inner| PyPage { inner })
            .collect();

        Ok(pages)
    }

    fn new_page(&self, url: &str) -> PyResult<PyPage> {
        let b = call_fut(self.inner.lock());
        let page = call_fut(b.new_page(url)).map_err(to_py_err)?;
        Ok(PyPage { inner: page })
    }

    fn close(&mut self) -> PyResult<()> {
        let mut b = call_fut(self.inner.lock());
        call_fut(b.close()).map_err(to_py_err).unwrap();
        Ok(())
    }

    fn __repr__(&self) -> String {
        "Browser()".to_owned()
    }
}

pub fn mod_browser(parent: &Bound<PyModule>) -> PyResult<()> {
    let py = parent.py();
    let sub = PyModule::new(py, "browser")?;

    sub.add_class::<PyBrowserConfig>()?;
    sub.add_class::<PyBrowserConfigBuilder>()?;
    sub.add_class::<PyBrowser>()?;

    parent.add_submodule(&sub)?;

    Ok(())
}
