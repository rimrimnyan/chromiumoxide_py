use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use once_cell::sync::OnceCell;
use std::future::Future;
use tokio::runtime::Runtime;

pub fn runtime() -> &'static Runtime {
    static RT: OnceCell<Runtime> = OnceCell::new();
    RT.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("failed to create Tokio runtime")
    })
}

pub fn call_fut<Fut, T>(fut: Fut) -> T
where
    Fut: Future<Output = T>,
{
    runtime().block_on(fut)
}

pub fn to_py_err<E: std::fmt::Display>(e: E) -> PyErr {
    PyRuntimeError::new_err(e.to_string())
}

pub fn getattr<T>(ob: &Borrowed<'_, '_, PyAny>, field: &str) -> Result<T, PyErr>
where
    T: for<'py> FromPyObject<'py, 'py>,
    for<'py> <T as FromPyObject<'py, 'py>>::Error: std::fmt::Display,
{
    ob.getattr(field)?.extract().map_err(to_py_err)
}
