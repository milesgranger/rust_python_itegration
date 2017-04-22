#[macro_use]
extern crate cpython;

use std::vec::Vec;
use cpython::{PyObject, PyResult, Python, PyTuple, PyDict};

py_module_initializer!(_rustbridge, init_rustbridge, PyInit__rustbridge, |py, m| {
    try!(m.add(py, "__doc__", "Module documentation string"));
    try!(m.add(py, "run", py_fn!(py, run(*args, **kwargs))));
    try!(m.add(py, "val", py_fn!(py, val())));
    try!(m.add(py, "multiply", py_fn!(py, multiply(a: Vec<f64>))));
    Ok(())
});

fn run(py: Python, args: &PyTuple, kwargs: Option<&PyDict>) -> PyResult<PyObject> {
    println!("Rust says: Hello Python!");
    for arg in args.iter(py) {
        println!("Rust got {}", arg);
    }
    if let Some(kwargs) = kwargs {
        for (key, val) in kwargs.items(py) {
            println!("{} = {}", key, val);
        }
    }
    Ok(py.None())
}

fn _multiply(a: &Vec<f64>) -> Vec<f64> {

    let mut tmp_vec: Vec<f64> = vec![];

    for i in a.iter() {
        let result: f64 = i * 2.;
        tmp_vec.push(result);
    }
    let tmp_vec = tmp_vec;
    tmp_vec
}

fn multiply(_: Python, a: Vec<f64>) -> PyResult<Vec<f64>> {
    let out = _multiply(&a);
    Ok(out)
}

fn val(_: Python) -> PyResult<i32> {
    Ok(42)
}