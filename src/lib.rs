mod browser;
mod element;
mod helper;
mod page;
// mod cdp;
// mod handler;

use crate::browser::mod_browser;
use crate::page::PyPage;

use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;

#[pymodule]
fn bindings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyPage>()?;

    mod_browser(m)?;

    Ok(())
}

define_stub_info_gatherer!(stub_info);
