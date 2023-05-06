use pyo3::prelude::*;

#[pyfunction]
fn str_len(a: &str) -> PyResult<usize> {
    Ok(a.len())
}

#[pyfunction]
fn round(a: f64) -> PyResult<usize>{
    Ok(a.round() as usize)
}

#[pyfunction]
fn cbrt(a: f64) -> PyResult<f64>{
    Ok(a.cbrt())
}

#[pyfunction]
fn array (array_input: Vec<i64>) -> PyResult<Vec<i64>>{
    let x: usize = array_input.len();
    let mut vec = Vec::with_capacity(x);
    for i in 0..x{
        vec.push(array_input[i]);
    }
    Ok(vec)

}

#[pymodule]
#[pyo3(name = "numpyrust")]
fn rust_len_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(str_len, m)?)?;
    m.add_function(wrap_pyfunction!(round, m)?)?;
    m.add_function(wrap_pyfunction!(cbrt, m)?)?;
    m.add_function(wrap_pyfunction!(array, m)?)?;
    Ok(())
}