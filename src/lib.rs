use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::PyTuple;
use pyo3::create_exception;

create_exception!(bigtree, SearchError, pyo3::exceptions::PyException);

#[pyfunction]
fn find_children(
    py: Python,
    tree: Vec<PyObject>,
    condition: PyObject,
    min_count: Option<usize>,
    max_count: Option<usize>,
) -> PyResult<&PyTuple> {
    let result_tuple;
    let result: Vec<PyObject> = tree
        .into_iter()
        .filter(|node| if !node.is_none(py) {condition.call1(py, (node,)).unwrap().is_true(py).unwrap()} else {false})
        .collect();

    let len = result.len();
    if let Some(min) = min_count {
        if min > 0 && len < min {
        let err_result = result.into_iter().map(|item| {
            item.getattr(py, "node_name").expect("node_name attribute should exist").extract::<String>(py).unwrap()
        }).collect::<Vec<String>>();
        let err_message = format!(
            "Expected more than {} element(s), found {} elements\n{:?}",
            min,
            len,
            err_result
        );
        return Err(SearchError::new_err(err_message))
        }
    }

    if let Some(max) = max_count {
        if max > 0 && len > max {
        let err_result = result.into_iter().map(|item| {
            item.getattr(py, "node_name").expect("node_name attribute should exist").extract::<String>(py).unwrap()
        }).collect::<Vec<String>>();
        let err_message = format!(
            "Expected less than {} element(s), found {} elements\n{:?}",
            max,
            len,
            err_result
        );
        return Err(SearchError::new_err(err_message))
        }
    }

    result_tuple = PyTuple::new(py, result);
    Ok(result_tuple)
}


#[pymodule]
fn bigtree_core(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("SearchError", py.get_type::<SearchError>())?;
    m.add_function(wrap_pyfunction!(find_children, m)?)?;
    Ok(())
}
