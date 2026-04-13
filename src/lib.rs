mod browser;
mod element;
mod helper;
mod page;
// mod cdp;
// mod handler;

use crate::browser::mod_browser;
use crate::element::mod_element;
use crate::page::mod_page;

use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;

#[pymodule]
fn bindings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    mod_browser(m)?;
    mod_element(m)?;
    mod_page(m)?;

    Ok(())
}

define_stub_info_gatherer!(stub_info);
