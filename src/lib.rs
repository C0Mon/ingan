pub mod format;
pub mod math;
pub mod render;

use pyo3::prelude::*;

use crate::format::Obj;


#[pyfunction]
fn test_obj_read(path: &str) -> PyResult<bool> {
    let mut object = Obj::new();
    object.read_obj_file(path).expect("Reading Object Failed");
    object.print_points();
    Ok(true)
}

/// Define the Python module
#[pymodule]
fn ingan(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(test_obj_read, m)?)?;
    Ok(())
}