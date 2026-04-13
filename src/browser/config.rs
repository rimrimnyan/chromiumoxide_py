use crate::helper::{getattr, to_py_err};

use std::collections::HashMap;
use std::time::Duration;

use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

use chromiumoxide::BrowserConfig;
use chromiumoxide::browser::BrowserConfigBuilder;
use chromiumoxide::handler::viewport::Viewport;

#[gen_stub_pyclass(module = "chromiumoxide_py.bindings.browser")]
#[pyclass(name = "_BrowserConfig")]
pub struct PyBrowserConfig {
    pub inner: BrowserConfig,
}

#[gen_stub_pymethods(module = "chromiumoxide_py.bindings.browser")]
#[pymethods]
impl PyBrowserConfig {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: PyBrowserConfigBuilder::new().build_this().inner,
        }
    }

    #[staticmethod]
    pub fn builder() -> PyBrowserConfigBuilder {
        PyBrowserConfigBuilder::new()
    }
}

impl FromPyObject<'_, '_> for PyBrowserConfig {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, '_, PyAny>) -> PyResult<Self> {
        let builder = PyBrowserConfigBuilder::extract(ob)?;
        builder
            .inner
            .build()
            .map(|cfg| PyBrowserConfig { inner: cfg })
            .map_err(to_py_err)
    }
}

#[gen_stub_pyclass(module = "chromiumoxide_py.bindings.browser")]
#[pyclass(name = "BrowserConfigBuilder")]
pub struct PyBrowserConfigBuilder {
    pub inner: BrowserConfigBuilder,
}

impl FromPyObject<'_, '_> for PyBrowserConfigBuilder {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, '_, PyAny>) -> PyResult<Self> {
        let headless_mode: u8 = ob.getattr("headless_mode")?.getattr("value")?.extract()?;
        let process_envs: Option<Vec<(String, String)>> = ob
            .getattr("process_envs")?
            .extract::<Option<HashMap<String, String>>>()?
            .map(|m| m.into_iter().collect());

        let sandbox: bool = getattr(&ob, "sandbox")?;
        let window_size: Option<(u32, u32)> = getattr(&ob, "window_size")?;
        let port: u16 = getattr(&ob, "port")?;
        let chrome_executable: Option<String> = getattr(&ob, "chrome_executable")?;
        let extensions: Vec<String> = getattr(&ob, "extensions")?;
        let user_data_dir: Option<String> = getattr(&ob, "user_data_dir")?;
        let incognito: bool = getattr(&ob, "incognito")?;
        let launch_timeout: u64 = getattr(&ob, "launch_timeout")?;
        let ignore_https_errors: bool = getattr(&ob, "ignore_https_errors")?;
        let ignore_invalid_messages: bool = getattr(&ob, "ignore_invalid_messages")?;
        let disable_https_first: bool = getattr(&ob, "disable_https_first")?;
        let request_timeout: u64 = getattr(&ob, "request_timeout")?;
        let args: Vec<String> = getattr(&ob, "args")?;
        let disable_default_args: bool = getattr(&ob, "disable_default_args")?;
        let request_intercept: bool = getattr(&ob, "request_intercept")?;
        let cache_enabled: bool = getattr(&ob, "cache_enabled")?;
        let hidden: bool = getattr(&ob, "hidden")?;

        let mut builder = BrowserConfigBuilder::default();

        match headless_mode {
            0 => builder = builder.with_head(),
            2 => builder = builder.new_headless_mode(),
            _ => {}
        }
        if !sandbox {
            builder = builder.no_sandbox();
        }
        if let Some((w, h)) = window_size {
            builder = builder.window_size(w, h);
        }
        if port != 0 {
            builder = builder.port(port);
        }
        if let Some(path) = chrome_executable {
            builder = builder.chrome_executable(path);
        }
        if !extensions.is_empty() {
            builder = builder.extensions(extensions);
        }
        if let Some(envs) = process_envs {
            builder = builder.envs(envs);
        }
        if let Some(dir) = user_data_dir {
            builder = builder.user_data_dir(dir);
        }
        if incognito {
            builder = builder.incognito();
        }
        builder = builder.launch_timeout(Duration::from_millis(launch_timeout));
        if !ignore_https_errors {
            builder = builder.respect_https_errors();
        }
        if !ignore_invalid_messages {
            builder = builder.surface_invalid_messages();
        }
        if disable_https_first {
            builder = builder.disable_https_first();
        }
        builder = builder.request_timeout(Duration::from_millis(request_timeout));
        if !args.is_empty() {
            builder = builder.args(args);
        }
        if disable_default_args {
            builder = builder.disable_default_args();
        }
        if request_intercept {
            builder = builder.enable_request_intercept();
        }
        if !cache_enabled {
            builder = builder.disable_cache();
        }
        if hidden {
            builder = builder.hide();
        }

        Ok(PyBrowserConfigBuilder::from_config_builder(builder))
    }
}

impl PyBrowserConfigBuilder {
    pub fn from_config_builder(builder: BrowserConfigBuilder) -> Self {
        Self { inner: builder }
    }

    pub fn build_this(&self) -> PyBrowserConfig {
        PyBrowserConfig {
            inner: self.inner.clone().build().unwrap(),
        }
    }
}

#[gen_stub_pymethods(module = "chromiumoxide_py.bindings.browser")]
#[pymethods]
impl PyBrowserConfigBuilder {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: BrowserConfigBuilder::default(),
        }
    }

    #[staticmethod]
    pub fn build_from_browser_config() -> Self {
        Self {
            inner: BrowserConfigBuilder::default(),
        }
    }

    pub fn window_size(mut slf: PyRefMut<Self>, width: u32, height: u32) -> PyRefMut<Self> {
        slf.inner = slf.inner.clone().window_size(width, height);
        slf
    }

    pub fn no_sandbox(mut slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf.inner = slf.inner.clone().no_sandbox();
        slf
    }

    pub fn with_head(mut slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf.inner = slf.inner.clone().with_head();
        slf
    }

    pub fn new_headless_mode(mut slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf.inner = slf.inner.clone().new_headless_mode();
        slf
    }

    pub fn incognito(mut slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf.inner = slf.inner.clone().incognito();
        slf
    }

    pub fn respect_https_errors(mut slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf.inner = slf.inner.clone().respect_https_errors();
        slf
    }

    pub fn surface_invalid_messages(mut slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf.inner = slf.inner.clone().surface_invalid_messages();
        slf
    }

    pub fn port(mut slf: PyRefMut<Self>, port: u16) -> PyRefMut<Self> {
        slf.inner = slf.inner.clone().port(port);
        slf
    }

    /// timeout in milliseconds
    pub fn launch_timeout(mut slf: PyRefMut<Self>, timeout_ms: u64) -> PyRefMut<Self> {
        slf.inner = slf
            .inner
            .clone()
            .launch_timeout(Duration::from_millis(timeout_ms));
        slf
    }

    /// timeout in milliseconds
    pub fn request_timeout(mut slf: PyRefMut<Self>, timeout_ms: u64) -> PyRefMut<Self> {
        slf.inner = slf
            .inner
            .clone()
            .request_timeout(Duration::from_millis(timeout_ms));
        slf
    }

    /// width and height in pixels; pass None to disable viewport emulation
    pub fn viewport(
        mut slf: PyRefMut<Self>,
        width: Option<u32>,
        height: Option<u32>,
    ) -> PyResult<PyRefMut<Self>> {
        let vp = match (width, height) {
            (Some(w), Some(h)) => Some(Viewport {
                width: w,
                height: h,
                ..Default::default()
            }),
            (None, None) => None,
            _ => {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    "provide both width and height, or neither to disable",
                ));
            }
        };
        slf.inner = slf.inner.clone().viewport(vp);
        Ok(slf)
    }

    pub fn user_data_dir(mut slf: PyRefMut<Self>, path: String) -> PyRefMut<Self> {
        slf.inner = slf.inner.clone().user_data_dir(path);
        slf
    }

    pub fn chrome_executable(mut slf: PyRefMut<Self>, path: String) -> PyRefMut<Self> {
        slf.inner = slf.inner.clone().chrome_executable(path);
        slf
    }

    pub fn extension(mut slf: PyRefMut<Self>, ext: String) -> PyRefMut<Self> {
        slf.inner = slf.inner.clone().extension(ext);
        slf
    }

    pub fn extensions(mut slf: PyRefMut<Self>, exts: Vec<String>) -> PyRefMut<Self> {
        slf.inner = slf.inner.clone().extensions(exts);
        slf
    }

    pub fn env(mut slf: PyRefMut<Self>, key: String, val: String) -> PyRefMut<Self> {
        slf.inner = slf.inner.clone().env(key, val);
        slf
    }

    pub fn envs(mut slf: PyRefMut<Self>, envs: Vec<(String, String)>) -> PyRefMut<Self> {
        slf.inner = slf.inner.clone().envs(envs);
        slf
    }

    pub fn arg(mut slf: PyRefMut<Self>, arg: String) -> PyRefMut<Self> {
        slf.inner = slf.inner.clone().arg(arg);
        slf
    }

    pub fn args(mut slf: PyRefMut<Self>, args: Vec<String>) -> PyRefMut<Self> {
        slf.inner = slf.inner.clone().args(args);
        slf
    }

    pub fn disable_default_args(mut slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf.inner = slf.inner.clone().disable_default_args();
        slf
    }

    pub fn disable_https_first(mut slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf.inner = slf.inner.clone().disable_https_first();
        slf
    }

    pub fn enable_request_intercept(mut slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf.inner = slf.inner.clone().enable_request_intercept();
        slf
    }

    pub fn disable_request_intercept(mut slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf.inner = slf.inner.clone().disable_request_intercept();
        slf
    }

    pub fn enable_cache(mut slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf.inner = slf.inner.clone().enable_cache();
        slf
    }

    pub fn disable_cache(mut slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf.inner = slf.inner.clone().disable_cache();
        slf
    }

    pub fn hide(mut slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf.inner = slf.inner.clone().hide();
        slf
    }

    pub fn build(slf: PyRef<Self>) -> PyResult<PyBrowserConfig> {
        slf.inner
            .clone()
            .build()
            .map(|cfg| PyBrowserConfig { inner: cfg })
            .map_err(to_py_err)
    }
}
