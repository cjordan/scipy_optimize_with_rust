use ndarray::parallel::prelude::*;
use numpy::PyReadonlyArray1;
use pyo3::prelude::*;
// use rayon::prelude::*;

/// Calculates the difference between the data and the model function
/// (`func_coeffs[0] * sin(i) + func_coeffs[1] * cos(i)`, where `i` is the index
/// into the data).
#[pyfunction]
fn cost_function(data: PyReadonlyArray1<f64>, func_coeffs: PyReadonlyArray1<f64>) -> f64 {
    let (c1, c2) = {
        let func_coeffs = func_coeffs.as_array();
        (func_coeffs[0], func_coeffs[1])
    };
    // Why do I need to allocate??
    let a = data.as_array().to_vec();

    a.par_iter()
        .enumerate()
        .fold(
            || 0.0,
            |accumulator, (i, data)| {
                let index = i as f64;
                let diff = *data - (c1 * index.sin() + c2 * index.cos());
                accumulator + diff.powi(2)
            },
        )
        .sum()
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn rustlib(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(cost_function, m)?)?;
    Ok(())
}
