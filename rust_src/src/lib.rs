use pyo3::prelude::*;
use rand::Rng;

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

#[pyfunction]
fn randint (low: isize, high: isize) -> PyResult<isize>{
    let num = rand::thread_rng().gen_range(low..high);
    Ok(num)
}

#[pyfunction]
fn linspace(start: f64, stop: f64, num_steps: usize) -> PyResult<Vec<f64>> {
    let step_size = (stop - start) / ((num_steps - 1) as f64);
    let mut values = Vec::with_capacity(num_steps);
    for i in 0..num_steps {
        values.push(start + (i as f64) * step_size);
    }
    Ok(values)
}

#[pyfunction]
fn equal(arr1: Vec<f64>, arr2: Vec<f64>) -> PyResult<Vec<bool>> {
    assert_eq!(arr1.len(), arr2.len(), "Arrays must have the same length");
    Ok(arr1.iter().zip(arr2.iter()).map(|(&x, &y)| x == y).collect())
}


#[pymodule]
#[pyo3(name = "numpyrust")]
fn all_funcs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(str_len, m)?)?;
    m.add_function(wrap_pyfunction!(round, m)?)?;
    m.add_function(wrap_pyfunction!(cbrt, m)?)?;
    m.add_function(wrap_pyfunction!(array, m)?)?;
    m.add_function(wrap_pyfunction!(randint, m)?)?;
    m.add_function(wrap_pyfunction!(linspace, m)?)?;
    m.add_function(wrap_pyfunction!(equal, m)?)?;
    Ok(())
}