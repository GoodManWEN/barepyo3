use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
// use hashbrown::HashMap;

#[pyfunction]
fn test(

) -> usize {
    0
}
/// A Python module implemented in Rust.
#[pymodule]
fn testlib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(test, m)?)?;
    Ok(())
}