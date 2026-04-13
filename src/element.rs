use crate::helper::getattr;
use crate::helper::{call_fut, to_py_err};

use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;

use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

use chromiumoxide::Element;
use chromiumoxide::Page;
use chromiumoxide::cdp::browser_protocol::page::{
    AddScriptToEvaluateOnNewDocumentParams, NavigateParams,
};
use chromiumoxide::error::CdpError;
use chromiumoxide::layout::Point;

use chromiumoxide::layout::BoundingBox;

#[gen_stub_pyclass(module = "chromiumoxide_py.bindings")]
#[pyclass]
pub struct PyElement {
    pub inner: Element,
}

#[gen_stub_pymethods(module = "chromiumoxide_py.bindings")]
#[pymethods]
impl PyElement {
    #[getter]
    fn bounding_box(&self) -> PyResult<(f64, f64, f64, f64)> {
        let bb = call_fut(self.inner.bounding_box()).map_err(to_py_err)?;
        Ok((bb.x, bb.y, bb.width, bb.height))
    }

    #[getter]
    fn inner_text(&self) -> PyResult<Option<String>> {
        call_fut(self.inner.inner_text()).map_err(to_py_err)
    }

    #[getter]
    fn inner_html(&self) -> PyResult<Option<String>> {
        call_fut(self.inner.inner_html()).map_err(to_py_err)
    }

    #[getter]
    fn outer_html(&self) -> PyResult<Option<String>> {
        call_fut(self.inner.outer_html()).map_err(to_py_err)
    }

    #[getter]
    fn attributes(&self) -> PyResult<HashMap<String, String>> {
        let mut attrs = HashMap::new();

        let mut iter = call_fut(self.inner.attributes())
            .map_err(to_py_err)?
            .into_iter();

        while let (Some(key), Some(val)) = (iter.next(), iter.next()) {
            attrs.insert(key, val);
        }

        Ok(attrs)
    }

    fn find_first_element(&self, selector: String) -> PyResult<Self> {
        Ok(PyElement {
            inner: call_fut(self.inner.find_element(selector)).map_err(to_py_err)?,
        })
    }

    fn find_elements(&self, selector: String) -> PyResult<Vec<Self>> {
        Ok(call_fut(self.inner.find_elements(selector))
            .map_err(to_py_err)?
            .into_iter()
            .map(|inner| PyElement { inner })
            .collect())
    }

    fn click(slf: PyRefMut<Self>) -> PyResult<PyRefMut<Self>> {
        call_fut(slf.inner.click()).map_err(to_py_err)?;
        Ok(slf)
    }

    fn type_str(slf: PyRefMut<Self>, input: String) -> PyResult<PyRefMut<Self>> {
        call_fut(slf.inner.type_str(input)).map_err(to_py_err)?;
        Ok(slf)
    }

    fn press_key(slf: PyRefMut<Self>, input: String) -> PyResult<PyRefMut<Self>> {
        call_fut(slf.inner.press_key(input)).map_err(to_py_err)?;
        Ok(slf)
    }

    fn focus(&self) -> PyResult<()> {
        call_fut(self.inner.focus()).map_err(to_py_err)?;
        Ok(())
    }

    fn scroll_into_view(&self) -> PyResult<()> {
        call_fut(self.inner.scroll_into_view()).map_err(to_py_err)?;
        Ok(())
    }
}
