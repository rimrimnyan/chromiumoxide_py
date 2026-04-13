use crate::element::PyElement;
use crate::helper::getattr;
use crate::helper::{call_fut, to_py_err};

use std::time::Duration;
use tokio::time::sleep;

use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

use chromiumoxide::Page;
use chromiumoxide::cdp::browser_protocol::page::{
    AddScriptToEvaluateOnNewDocumentParams, NavigateParams,
};
use chromiumoxide::error::CdpError;
use chromiumoxide::layout::Point;

#[gen_stub_pyclass(module = "chromiumoxide_py.bindings")]
#[pyclass(name = "Page")]
#[derive(Debug, Clone)]
pub struct PyPage {
    pub inner: Page,
}

#[gen_stub_pymethods(module = "chromiumoxide_py.bindings")]
#[pymethods]
impl PyPage {
    #[getter]
    fn url(&self) -> PyResult<String> {
        Ok(call_fut(self.inner.url()).map_err(to_py_err)?.unwrap())
    }

    #[getter]
    fn content(&self) -> PyResult<String> {
        call_fut(self.inner.content()).map_err(to_py_err)
    }

    #[getter]
    fn user_agent(&self) -> PyResult<String> {
        call_fut(self.inner.user_agent()).map_err(to_py_err)
    }

    #[setter]
    fn set_user_agent(&self, ua: String) -> PyResult<()> {
        call_fut(self.inner.set_user_agent(ua)).map_err(to_py_err)?;
        Ok(())
    }

    fn enable_stealth_mode(&self) -> PyResult<()> {
        call_fut(self.inner.enable_stealth_mode()).map_err(to_py_err)?;
        Ok(())
    }

    fn enable_stealth_mode_with_agent(&self, ua: String) -> PyResult<()> {
        call_fut(self.inner.enable_stealth_mode_with_agent(ua.as_str())).map_err(to_py_err)?;
        Ok(())
    }

    fn enable_stealth_mode_2(&self) -> PyResult<()> {
        self.apply_evasion_scripts().map_err(to_py_err)?;
        call_fut(self.inner.set_user_agent("Mozilla/5.0 (Windows NT 11.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.5296.0 Safari/537.36")).map_err(to_py_err)?;
        Ok(())
    }

    fn execute(&self, command: &Bound<'_, PyAny>) -> PyResult<()> {
        let class_name = command.get_type().name()?;
        let name = class_name.to_str()?;

        let obj = &command.as_borrowed();

        let res = match name {
            "AddScriptToEvaluateOnNewDocument" => AddScriptToEvaluateOnNewDocumentParams {
                source: getattr(obj, "source")?,
                world_name: getattr(obj, "world_name")?,
                include_command_line_api: getattr(obj, "include_command_line_api")?,
                run_immediately: getattr(obj, "run_immediately")?,
            },
            _ => return Err(PyTypeError::new_err("Invalid command!")),
        };

        call_fut(self.inner.execute(res)).map_err(to_py_err)?;

        Ok(())
    }

    fn evaluate(&self, source: String) -> PyResult<String> {
        Ok(call_fut(self.inner.evaluate(source))
            .map_err(to_py_err)?
            .into_value()
            .unwrap())
    }

    fn find_first_element(&self, selector: String) -> PyResult<PyElement> {
        Ok(PyElement {
            inner: call_fut(self.inner.find_element(selector)).map_err(to_py_err)?,
        })
    }

    fn find_elements(&self, selector: String) -> PyResult<Vec<PyElement>> {
        Ok(call_fut(self.inner.find_elements(selector))
            .map_err(to_py_err)?
            .into_iter()
            .map(|inner| PyElement { inner })
            .collect())
    }

    fn goto(slf: PyRefMut<Self>, url: String) -> PyResult<PyRefMut<Self>> {
        let nav_params: NavigateParams = url.into();
        call_fut(slf.inner.execute(nav_params)).map_err(to_py_err)?;
        Ok(slf)
    }

    fn click(slf: PyRefMut<Self>, x: f64, y: f64) -> PyResult<PyRefMut<Self>> {
        call_fut(slf.inner.click(Point { x, y })).map_err(to_py_err)?;
        Ok(slf)
    }

    fn wait_for_navigation(slf: PyRefMut<Self>) -> PyResult<PyRefMut<Self>> {
        call_fut(slf.inner.wait_for_navigation()).map_err(to_py_err)?;
        Ok(slf)
    }

    fn scroll_to_bottom(&self) -> PyResult<()> {
        let mut previous_height = 0;

        loop {
            // Get current scroll height
            let height: i64 = call_fut(self.inner.evaluate("document.body.scrollHeight"))
                .map_err(to_py_err)?
                .into_value()
                .unwrap();

            // Break if no new content loaded
            if height == previous_height {
                break;
            }

            previous_height = height;

            // Scroll to bottom
            call_fut(
                self.inner
                    .evaluate("window.scrollTo(0, document.body.scrollHeight);"),
            )
            .map_err(to_py_err)?;

            // Wait for new content to load
            println!("Scrolling... (height={})", height);
            call_fut(sleep(Duration::from_secs(2)));
        }

        Ok(())
    }
}

impl PyPage {
    // some evasion scripts
    fn apply_script<T: Into<String>>(&self, source: T) -> Result<(), CdpError> {
        let script = AddScriptToEvaluateOnNewDocumentParams {
            source: source.into(),
            world_name: None,
            include_command_line_api: None,
            run_immediately: None,
        };

        call_fut(self.inner.execute(script))?;
        Ok(())
    }

    // applies all scripts found in evasions folder
    fn apply_evasion_scripts(&self) -> Result<(), CdpError> {
        for src in EVASIONS {
            self.apply_script(src)?
        }
        Ok(())
    }
}

// evasion scripts
static EVASIONS: [&str; 13] = [
    include_str!("evasions/chrome_app.js"),
    include_str!("evasions/chrome_runtime.js"),
    include_str!("evasions/hairline_fix.js"),
    include_str!("evasions/iframe_content_window.js"),
    include_str!("evasions/media_codecs.js"),
    include_str!("evasions/navigator_language.js"),
    include_str!("evasions/navigator_permissions.js"),
    include_str!("evasions/navigator_plugins.js"),
    include_str!("evasions/navigator_vendor.js"),
    include_str!("evasions/navigator_webdriver.js"),
    include_str!("evasions/utils.js"),
    include_str!("evasions/webgl_vendor_override.js"),
    include_str!("evasions/window_outerdimensions.js"),
];
