//! Python bindings for the `chromiumoxide` crate (PyO3 + Tokio).
//!
//! # Quick start
//!
//! ```python
//! from chromiumoxide_py import BrowserConfig, Browser, AddScriptToEvaluateOnNewDocumentParams
//!
//! cfg  = BrowserConfig.builder().chrome_executable("/usr/bin/chromium").build()
//! b    = Browser.launch(cfg)
//! page = b.new_page("https://example.com")
//! html = page.wait_for_navigation().content()
//! b.close()
//! ```

use std::sync::Arc;

use futures::StreamExt;
use once_cell::sync::OnceCell;
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3_stub_gen::{define_stub_info_gatherer, derive::*};
use tokio::runtime::Runtime;

// ─── Shared Tokio runtime ────────────────────────────────────────────────────

fn runtime() -> &'static Runtime {
    static RT: OnceCell<Runtime> = OnceCell::new();
    RT.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("failed to create Tokio runtime")
    })
}

fn to_py_err<E: std::fmt::Display>(e: E) -> PyErr {
    PyRuntimeError::new_err(e.to_string())
}

#[gen_stub_pyclass(module = "chromiumoxide_py.bindings")]
#[pyclass(name = "_BrowserConfigBuilder")]
pub struct PyBrowserConfigBuilder {
    executable: Option<String>,
    headless: bool,
    sandbox: bool,
    args: Vec<String>,
    window_width: Option<u32>,
    window_height: Option<u32>,
    user_data_dir: Option<String>,
}

#[gen_stub_pymethods(module = "chromiumoxide_py.bindings")]
#[pymethods]
impl PyBrowserConfigBuilder {
    #[new]
    fn new() -> Self {
        Self {
            executable: None,
            headless: true,
            sandbox: true,
            args: Vec::new(),
            window_width: None,
            window_height: None,
            user_data_dir: None,
        }
    }

    /// Set the path to the Chrome/Chromium executable.
    fn chrome_executable(&mut self, path: String) {
        self.executable = Some(path);
    }

    /// Show a browser window instead of running headlessly.
    fn with_head(&mut self) {
        self.headless = false;
    }

    /// Disable the Chrome sandbox (needed in many Docker/container setups).
    fn no_sandbox(&mut self) {
        self.sandbox = false;
    }

    /// Append an extra CLI argument passed directly to the Chrome process.
    fn arg(&mut self, arg: String) {
        self.args.push(arg);
    }

    /// Set the browser window dimensions (pixels).
    fn window_size(&mut self, width: u32, height: u32) {
        self.window_width = Some(width);
        self.window_height = Some(height);
    }

    /// Set the Chrome user-data directory (for profile persistence).
    fn user_data_dir(&mut self, path: String) {
        self.user_data_dir = Some(path);
    }

    /// Build a ready-to-use `BrowserConfig`.
    ///
    /// Raises `ValueError` if the configuration is invalid.
    fn build(&self) -> PyResult<PyBrowserConfig> {
        use chromiumoxide::browser::BrowserConfig;

        let mut b = BrowserConfig::builder();

        if let Some(ref path) = self.executable {
            b = b.chrome_executable(path);
        }
        if !self.headless {
            b = b.with_head();
        }
        if !self.sandbox {
            b = b.no_sandbox();
        }
        for arg in &self.args {
            b = b.arg(arg.clone());
        }
        if let (Some(w), Some(h)) = (self.window_width, self.window_height) {
            b = b.window_size(w, h);
        }
        if let Some(ref dir) = self.user_data_dir {
            b = b.user_data_dir(dir);
        }

        let config = b.build().map_err(PyValueError::new_err)?;
        Ok(PyBrowserConfig { inner: config })
    }
}

#[gen_stub_pyclass(module = "chromiumoxide_py.bindings")]
#[pyclass(name = "_BrowserConfig")]
#[derive(Clone)]
pub struct PyBrowserConfig {
    inner: chromiumoxide::browser::BrowserConfig,
}

#[gen_stub_pymethods(module = "chromiumoxide_py.bindings")]
#[pymethods]
impl PyBrowserConfig {
    #[staticmethod]
    fn builder() -> PyBrowserConfigBuilder {
        PyBrowserConfigBuilder::new()
    }

    #[staticmethod]
    fn default() -> PyBrowserConfig {
        PyBrowserConfigBuilder::new().build().unwrap()
    }
}

#[gen_stub_pyclass(module = "chromiumoxide_py.bindings")]
#[pyclass(name = "_Browser")]
pub struct PyBrowser {
    inner: Arc<tokio::sync::Mutex<chromiumoxide::Browser>>,
    /// Keeps the background WebSocket handler task alive for the browser
    /// lifetime.
    _handler: Arc<tokio::task::JoinHandle<()>>,
}

#[gen_stub_pymethods(module = "chromiumoxide_py.bindings")]
#[pymethods]
impl PyBrowser {
    #[staticmethod]
    fn launch(config: PyBrowserConfig) -> PyResult<Self> {
        let owned = config.inner.to_owned();

        let rt = runtime();
        let (browser, mut handler) = rt
            .block_on(chromiumoxide::Browser::launch(owned))
            .map_err(to_py_err)?;

        // Drive the internal WebSocket/CDP handler in a background task.
        let handle = rt.spawn(async move { while let Some(_event) = handler.next().await {} });

        Ok(PyBrowser {
            inner: Arc::new(tokio::sync::Mutex::new(browser)),
            _handler: Arc::new(handle),
        })
    }

    fn new_page(&self, url: &str) -> PyResult<PyPage> {
        let browser = self.inner.clone();
        let url = url.to_owned();
        let page = runtime()
            .block_on(async move {
                let b = browser.lock().await;
                b.new_page(url).await
            })
            .map_err(to_py_err)?;
        Ok(PyPage {
            inner: Arc::new(page),
        })
    }

    fn close(&self) -> PyResult<()> {
        let browser = self.inner.clone();
        runtime()
            .block_on(async move {
                let mut b = browser.lock().await;
                b.close().await
            })
            .map_err(to_py_err)?;
        Ok(())
    }

    fn websocket_address(&self) -> String {
        let browser = self.inner.clone();
        runtime().block_on(async move {
            let b = browser.lock().await;
            b.websocket_address().to_owned()
        })
    }

    fn __repr__(&self) -> String {
        "Browser(<chromiumoxide browser>)".to_owned()
    }
}

#[gen_stub_pyclass(module = "chromiumoxide_py.bindings")]
#[pyclass(name = "_AddScriptToEvaluateOnNewDocumentParams", from_py_object)]
#[derive(Clone)]
pub struct PyAddScriptParams {
    source: String,
    include_command_line_api: Option<bool>,
    world_name: Option<String>,
    run_immediately: Option<bool>,
}

#[gen_stub_pymethods(module = "chromiumoxide_py.bindings")]
#[pymethods]
impl PyAddScriptParams {
    #[new]
    #[pyo3(signature = (source, include_command_line_api=None, world_name=None, run_immediately=None))]
    fn new(
        source: String,
        include_command_line_api: Option<bool>,
        world_name: Option<String>,
        run_immediately: Option<bool>,
    ) -> Self {
        Self {
            source,
            include_command_line_api,
            world_name,
            run_immediately,
        }
    }

    #[getter]
    fn source(&self) -> &str {
        &self.source
    }

    #[getter]
    fn include_command_line_api(&self) -> Option<bool> {
        self.include_command_line_api
    }

    #[getter]
    fn world_name(&self) -> Option<&str> {
        self.world_name.as_deref()
    }

    #[getter]
    fn run_immediately(&self) -> Option<bool> {
        self.run_immediately
    }
}

impl PyAddScriptParams {
    fn into_cdp(
        self,
    ) -> chromiumoxide_cdp::cdp::browser_protocol::page::AddScriptToEvaluateOnNewDocumentParams
    {
        use chromiumoxide_cdp::cdp::browser_protocol::page::AddScriptToEvaluateOnNewDocumentParams;
        let mut p = AddScriptToEvaluateOnNewDocumentParams::new(self.source);
        p.include_command_line_api = self.include_command_line_api;
        p.world_name = self.world_name;
        p.run_immediately = self.run_immediately;
        p
    }
}

// ─── Page ────────────────────────────────────────────────────────────────────

#[gen_stub_pyclass(module = "chromiumoxide_py.bindings")]
#[pyclass(name = "_Page")]
pub struct PyPage {
    inner: Arc<chromiumoxide::Page>,
}

#[gen_stub_pymethods(module = "chromiumoxide_py.bindings")]
#[pymethods]
impl PyPage {
    fn content(&self) -> PyResult<String> {
        let page = self.inner.clone();
        runtime()
            .block_on(async move { page.content().await })
            .map_err(to_py_err)
    }

    fn set_content(&self, html: &str) -> PyResult<()> {
        let page = self.inner.clone();
        let html = html.to_owned();
        // FIX: set_content returns Result<&Page, _> — borrow ends within the
        // async block by discarding the reference with `map(|_| ())`.
        runtime()
            .block_on(async move { page.set_content(html).await.map(|_| ()) })
            .map_err(to_py_err)
    }

    fn title(&self) -> PyResult<Option<String>> {
        let page = self.inner.clone();
        runtime()
            .block_on(async move { page.get_title().await })
            .map_err(to_py_err)
    }

    fn url(&self) -> PyResult<Option<String>> {
        let page = self.inner.clone();
        runtime()
            .block_on(async move { page.url().await })
            .map_err(to_py_err)
    }

    fn evaluate(&self, expression: &str) -> PyResult<String> {
        let page = self.inner.clone();
        let expr = expression.to_owned();
        let js_val = runtime()
            .block_on(async move { page.evaluate(expr).await })
            .map_err(to_py_err)?;
        // FIX: added serde_json to Cargo.toml (see note below).
        Ok(serde_json::to_string(
            js_val
                .value()
                .as_ref()
                .map_or(&serde_json::Value::Null, |v| v),
        )
        .unwrap_or_default())
    }

    fn add_script_to_evaluate_on_new_document(
        &self,
        params: &PyAddScriptParams,
    ) -> PyResult<String> {
        let page = self.inner.clone();
        let cdp = params.clone().into_cdp();
        let ret = runtime()
            .block_on(async move { page.execute(cdp).await })
            .map_err(to_py_err)?;
        // FIX: ScriptIdentifier's constructor is private; clone the inner String
        Ok(ret.identifier.clone().into())
    }

    fn remove_script_to_evaluate_on_new_document(&self, identifier: &str) -> PyResult<()> {
        use chromiumoxide_cdp::cdp::browser_protocol::page::{
            RemoveScriptToEvaluateOnNewDocumentParams, ScriptIdentifier,
        };
        let page = self.inner.clone();
        // ScriptIdentifier's constructor is private; use From<String> instead.
        let id: ScriptIdentifier = identifier.to_owned().into();
        runtime()
            .block_on(async move {
                page.execute(RemoveScriptToEvaluateOnNewDocumentParams::new(id))
                    .await
            })
            .map_err(to_py_err)?;
        Ok(())
    }

    fn enable_stealth_mode(&self) -> PyResult<()> {
        let page = self.inner.clone();
        runtime()
            .block_on(async move { page.enable_stealth_mode().await })
            .map_err(to_py_err)
    }

    fn screenshot(&self) -> PyResult<Vec<u8>> {
        use chromiumoxide_cdp::cdp::browser_protocol::page::{
            CaptureScreenshotFormat, CaptureScreenshotParams,
        };
        let page = self.inner.clone();
        // FIX: CaptureScreenshotParams::builder().build() returns the struct
        // directly (not a Result), so there is no .map_err() to call.
        let params = CaptureScreenshotParams::builder()
            .format(CaptureScreenshotFormat::Png)
            .build();
        runtime()
            .block_on(async move { page.screenshot(params).await })
            .map_err(to_py_err)
    }

    /// Render the page to a PDF and return the raw bytes.
    ///
    /// Args:
    ///     landscape (bool): Use landscape orientation.  Default ``False``.
    #[pyo3(signature = (landscape = false))]
    fn pdf(&self, landscape: bool) -> PyResult<Vec<u8>> {
        use chromiumoxide_cdp::cdp::browser_protocol::page::PrintToPdfParams;
        let page = self.inner.clone();
        // FIX: same as above — PrintToPdfParams::builder().build() is infallible.
        let params = PrintToPdfParams::builder().landscape(landscape).build();
        runtime()
            .block_on(async move { page.pdf(params).await })
            .map_err(to_py_err)
    }

    // ── DOM ───────────────────────────────────────────────────────────────────

    /// Return the first DOM element matching the CSS `selector`.
    ///
    /// Raises `RuntimeError` if no element matches.
    fn find_element(&self, selector: &str) -> PyResult<PyElement> {
        let page = self.inner.clone();
        let sel = selector.to_owned();
        let elem = runtime()
            .block_on(async move { page.find_element(sel).await })
            .map_err(to_py_err)?;
        Ok(PyElement {
            inner: Arc::new(elem),
        })
    }

    /// Return all DOM elements matching the CSS `selector`.
    fn find_elements(&self, selector: &str) -> PyResult<Vec<PyElement>> {
        let page = self.inner.clone();
        let sel = selector.to_owned();
        let elems = runtime()
            .block_on(async move { page.find_elements(sel).await })
            .map_err(to_py_err)?;
        Ok(elems
            .into_iter()
            .map(|e| PyElement { inner: Arc::new(e) })
            .collect())
    }

    // ── Misc ──────────────────────────────────────────────────────────────────

    /// Override the User-Agent header sent with every request.
    fn set_user_agent(&self, user_agent: &str) -> PyResult<()> {
        let page = self.inner.clone();
        let ua = user_agent.to_owned();
        // FIX: set_user_agent returns Result<&Page, _> — discard the reference.
        runtime()
            .block_on(async move { page.set_user_agent(ua).await.map(|_| ()) })
            .map_err(to_py_err)
    }

    /// Return the cookies visible to the current URL as a JSON string.
    fn cookies(&self) -> PyResult<String> {
        let page = self.inner.clone();
        let cookies = runtime()
            .block_on(async move { page.get_cookies().await })
            .map_err(to_py_err)?;
        // FIX: added serde_json to Cargo.toml; explicit closure type silences E0282.
        serde_json::to_string(&cookies)
            .map_err(|e: serde_json::Error| PyRuntimeError::new_err(e.to_string()))
    }

    /// Close this tab.
    fn close(&self) -> PyResult<()> {
        // FIX: Page::close(self) consumes the Page by value, but we only hold
        // an Arc<Page>. Dereference and clone the inner Page before calling close().
        let page: chromiumoxide::Page = (*self.inner).clone();
        runtime()
            .block_on(async move { page.close().await })
            .map_err(to_py_err)
    }

    fn __repr__(&self) -> String {
        "Page(<chromiumoxide tab>)".to_owned()
    }
}

#[gen_stub_pyclass(module = "chromiumoxide_py.bindings")]
#[pyclass(name = "_Element")]
pub struct PyElement {
    inner: Arc<chromiumoxide::Element>,
}

#[gen_stub_pymethods(module = "chromiumoxide_py.bindings")]
#[pymethods]
impl PyElement {
    /// Click the element.  Returns self.
    fn click(&self) -> PyResult<Py<PyElement>> {
        let elem = self.inner.clone();
        runtime()
            .block_on(async move {
                elem.click().await?;
                Ok(())
            })
            .map_err(to_py_err::<Box<dyn std::error::Error + Send + Sync>>)?;
        pyo3::Python::attach(|py| {
            Py::new(
                py,
                PyElement {
                    inner: self.inner.clone(),
                },
            )
        })
    }

    /// Type `text` into the element (focuses it first).  Returns self.
    fn type_str(&self, text: &str) -> PyResult<Py<PyElement>> {
        let elem = self.inner.clone();
        let text = text.to_owned();
        runtime()
            .block_on(async move {
                elem.type_str(text).await?;
                Ok(())
            })
            .map_err(to_py_err::<Box<dyn std::error::Error + Send + Sync>>)?;
        pyo3::Python::attach(|py| {
            Py::new(
                py,
                PyElement {
                    inner: self.inner.clone(),
                },
            )
        })
    }

    /// Dispatch a key-press event (e.g. ``"Enter"``, ``"Tab"``, ``"Escape"``).
    ///
    /// Returns self.
    fn press_key(&self, key: &str) -> PyResult<Py<PyElement>> {
        let elem = self.inner.clone();
        let key = key.to_owned();
        runtime()
            .block_on(async move {
                elem.press_key(key).await?;
                Ok(())
            })
            .map_err(to_py_err::<Box<dyn std::error::Error + Send + Sync>>)?;
        pyo3::Python::attach(|py| {
            Py::new(
                py,
                PyElement {
                    inner: self.inner.clone(),
                },
            )
        })
    }

    /// Return the element's visible text (``innerText``), or ``None``.
    fn inner_text(&self) -> PyResult<Option<String>> {
        let elem = self.inner.clone();
        runtime()
            .block_on(async move { elem.inner_text().await })
            .map_err(to_py_err)
    }

    /// Return the element's raw HTML (``innerHTML``), or ``None``.
    fn inner_html(&self) -> PyResult<Option<String>> {
        let elem = self.inner.clone();
        runtime()
            .block_on(async move { elem.inner_html().await })
            .map_err(to_py_err)
    }

    /// Return the value of attribute `name`, or ``None`` if not present.
    fn attribute(&self, name: &str) -> PyResult<Option<String>> {
        let elem = self.inner.clone();
        let name = name.to_owned();
        runtime()
            .block_on(async move { elem.attribute(name).await })
            .map_err(to_py_err)
    }

    /// Scroll this element into the viewport.
    fn scroll_into_view(&self) -> PyResult<()> {
        let elem = self.inner.clone();
        // FIX: scroll_into_view returns Result<&Element, _> — discard the reference.
        runtime()
            .block_on(async move { elem.scroll_into_view().await.map(|_| ()) })
            .map_err(to_py_err)
    }

    /// Give keyboard focus to this element.
    fn focus(&self) -> PyResult<()> {
        let elem = self.inner.clone();
        // FIX: focus returns Result<&Element, _> — discard the reference.
        runtime()
            .block_on(async move { elem.focus().await.map(|_| ()) })
            .map_err(to_py_err)
    }

    /// Find a child element matching `selector` within this element's subtree.
    fn find_element(&self, selector: &str) -> PyResult<PyElement> {
        let elem = self.inner.clone();
        let sel = selector.to_owned();
        let child = runtime()
            .block_on(async move { elem.find_element(sel).await })
            .map_err(to_py_err)?;
        Ok(PyElement {
            inner: Arc::new(child),
        })
    }

    fn __repr__(&self) -> String {
        "Element(<chromiumoxide element>)".to_owned()
    }
}

#[pymodule]
fn bindings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyBrowserConfigBuilder>()?;
    m.add_class::<PyBrowserConfig>()?;
    m.add_class::<PyBrowser>()?;
    m.add_class::<PyPage>()?;
    m.add_class::<PyElement>()?;
    m.add_class::<PyAddScriptParams>()?;

    Ok(())
}

define_stub_info_gatherer!(stub_info);
