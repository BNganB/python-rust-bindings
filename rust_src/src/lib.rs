use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::PyList;
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

#[pyfunction]
fn str_len(a: &str) -> PyResult<usize> {
    Ok(a.len())
}

#[pyfunction]
fn round(a: f64) -> PyResult<usize> {
    Ok(a.round() as usize)
}

#[pyfunction]
fn cbrt(a: f64) -> PyResult<f64> {
    Ok(a.cbrt())
}

#[pyfunction]
fn array(array_input: Vec<i64>) -> PyResult<Vec<i64>> {
    let x: usize = array_input.len();
    let mut vec = Vec::with_capacity(x);
    for i in 0..x {
        vec.push(array_input[i]);
    }
    Ok(vec)
}

#[pyfunction]
fn randint(low: isize, high: isize) -> PyResult<isize> {
    let num = rand::thread_rng().gen_range(low..high);
    Ok(num)
}

#[pyfunction]
fn linspace(start: f64, stop: f64, num_steps: usize) -> PyResult<Vec<f64>> {
    let step_size: f64 = (stop - start) / ((num_steps - 1) as f64);
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

#[pyfunction]
fn read_file(file_name: &str) -> PyResult<String> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}


// buffered reader implementation to test speed difference
#[pyfunction]
//&str -> &PyUnicode for less conversion
fn read_file_v2(file_name: &str) -> PyResult<String> {
    let file = File::open(file_name)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

#[pyfunction]
fn abs(num: f64) -> PyResult<f64> {
    Ok(f64::abs(num))
}


#[pyfunction]
fn timer_wrapper(_py: Python, func: &PyAny) -> PyResult<String> {
    let start_time = Instant::now();

    func.call0()?;

    let end_time = Instant::now();
    let duration = end_time - start_time;
    let duration_ms = duration.as_secs_f64() * 1000.0;

    let result = format!("Execution time: {:.2} ms", duration_ms);

    Ok(result)
}

#[pyfunction]
fn triangular_number(input_num: isize) -> PyResult<isize> {
    if input_num <= 0 {
        // positive only check
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Input number must be a positive integer.",
        ));
    }

    let mut i = input_num;
    let mut result = 0;

    while i != 0 {
        result += i;
        i -= 1;
    }

    Ok(result)
}

#[pyfunction]
fn list_sort(mut input_list: Vec<isize>) -> PyResult<Vec<isize>> {
    //performance still stucks, PyList method from docs doesnt work. Revisit?
    input_list.sort();

    Ok(input_list)
}

#[pyfunction]
fn how_many_x_in_y(slice: &str, full: &str) -> PyResult<String> {
    let matches = full.matches(slice).count();
    if matches == 0 {
        Ok("No matches found".to_string())
    } else {
        Ok(matches.to_string())
    }
}

fn add_functions(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(str_len, m)?)?;
    m.add_function(wrap_pyfunction!(round, m)?)?;
    m.add_function(wrap_pyfunction!(cbrt, m)?)?;
    m.add_function(wrap_pyfunction!(array, m)?)?;
    m.add_function(wrap_pyfunction!(randint, m)?)?;
    m.add_function(wrap_pyfunction!(linspace, m)?)?;
    m.add_function(wrap_pyfunction!(equal, m)?)?;
    m.add_function(wrap_pyfunction!(read_file, m)?)?;
    m.add_function(wrap_pyfunction!(read_file_v2, m)?)?;
    m.add_function(wrap_pyfunction!(abs, m)?)?;
    m.add_function(wrap_pyfunction!(timer_wrapper, m)?)?;
    m.add_function(wrap_pyfunction!(triangular_number, m)?)?;
    m.add_function(wrap_pyfunction!(list_sort, m)?)?;
    m.add_function(wrap_pyfunction!(how_many_x_in_y, m)?)?;
    Ok(())
}

#[pymodule]
#[pyo3(name = "rusted")]
fn rusted(_py: Python, m: &PyModule) -> PyResult<()> {
    add_functions(m)?;
    Ok(())
}