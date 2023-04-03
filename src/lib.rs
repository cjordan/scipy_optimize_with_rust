use numpy::PyReadonlyArray1;
use pyo3::{prelude::*, types::PyList};
use rayon::prelude::*;

#[pyclass]
struct RustCostFunction {
    data: Vec<f64>,
}

#[pymethods]
impl RustCostFunction {
    #[new]
    fn new(data: PyReadonlyArray1<f64>) -> Self {
        Self {
            data: data.to_vec().expect("data passed in is contiguous"),
        }
    }

    /// Calculates the difference between the data and the model function
    /// (`func_coeffs[0] * sin(i) + func_coeffs[1] * cos(i)`, where `i` is the
    /// index into the data).
    fn cost_function(&self, x: PyReadonlyArray1<f64>, args: &PyList) -> f64 {
        let (c1, c2) = {
            let func_coeffs = x.as_array();
            (func_coeffs[0], func_coeffs[1])
        };
        let parallel: bool = args
            .get_item(0)
            .and_then(|item| item.extract())
            .unwrap_or_default();

        if parallel {
            self.data
                .par_iter()
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
        } else {
            self.data
                .iter()
                .enumerate()
                .fold(0.0, |accumulator, (i, data)| {
                    let index = i as f64;
                    let diff = *data - (c1 * index.sin() + c2 * index.cos());
                    accumulator + diff.powi(2)
                })
        }
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn rustlib(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<RustCostFunction>()?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
