// use pyo3::prelude::*;
// use std::process::Command;
// /// Formats the sum of two numbers as string.
// #[pyfunction]
// fn spawn(path:String,args:Vec<String>) -> PyResult<Command>{
//     // Replace "your_executable.exe" with the actual name of your executable
//     println!("started");
//     // Execute the external executable
//     let mut op = Command::new(path)
//         // You can add arguments if your executable accepts them
//         .args(args);
//     // let output_str = String::from_utf8_lossy(&op.stdout).to_string();
//     Ok(op)
// }

// #[pymodule]
// fn Process(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(spawn, m)?)?;
//     Ok(())
// }
use pyo3::prelude::*;
use std::process::Command;

#[pyfunction]
fn spawn(path: String, args: Vec<String>) -> PyResult<Command> {
    // Create and configure the Command
    let mut command = Command::new(path).args(args);
    // let output = command.output();

    Ok(command)
}

#[pymodule]
fn process(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(spawn, m)?)?;
    Ok(())
}

// use pyo3::prelude::*;
// use std::process::{Command, Stdio};

// #[pyfunction]
// fn spawn(path: String, args: Vec<String>) -> PyResult<String> {
//     let output = Command::new(&path)
//         .args(args)
//         .stdout(Stdio::piped())
//         .output()
//         // .map_err(|e| PyErr::new::<exceptions::OSError, _>(e.to_string()))?;

//     let output_str = String::from_utf8_lossy(&output.stdout).to_string();
//     Ok(output_str)
// }

// #[pymodule]
// fn mymodule(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(spawn, m)?)?;
//     Ok(())
// }
