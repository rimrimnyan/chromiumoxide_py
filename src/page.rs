use crate::helper::getattr;
use crate::helper::{call_fut, to_py_err};

use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

use chromiumoxide::Page;
use chromiumoxide::cdp::browser_protocol::page::{
    AddScriptToEvaluateOnNewDocumentParams, NavigateParams,
};

#[gen_stub_pyclass(module = "chromiumoxide_py.bindings")]
#[pyclass(name = "Page")]
#[derive(Debug, Clone)]
pub struct PyPage {
    pub inner: Page,
}

#[gen_stub_pymethods(module = "chromiumoxide_py.bindings")]
#[pymethods]
impl PyPage {
    fn enable_stealth_mode(&self) -> PyResult<()> {
        call_fut(self.inner.enable_stealth_mode()).map_err(to_py_err)?;
        Ok(())
    }

    fn enable_stealth_mode_with_agent(&self, ua: String) -> PyResult<()> {
        call_fut(self.inner.enable_stealth_mode_with_agent(ua.as_str())).map_err(to_py_err)?;
        Ok(())
    }

    fn goto(slf: PyRefMut<Self>, url: String) -> PyResult<PyRefMut<Self>> {
        let nav_params: NavigateParams = url.into();
        call_fut(slf.inner.execute(nav_params)).map_err(to_py_err)?;
        Ok(slf)
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

    fn url(&self) -> PyResult<String> {
        Ok(call_fut(self.inner.url()).map_err(to_py_err)?.unwrap())
    }
}
