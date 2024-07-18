use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn rtime() -> PyResult<f64> {
    Ok(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64())
}

/// A Python module implemented in Rust.
#[pymodule]
fn rpy_helper(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rtime, m)?)?;
    Ok(())
}
