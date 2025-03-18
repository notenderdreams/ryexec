use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde_json::Value;

use crate::{Error, Result};

fn json_to_py(py: Python, value: &Value) -> Result<PyObject> {
    match value {
        Value::String(s) => Ok(s.to_object(py)),
        Value::Number(n) => {
            if n.is_i64() {
                Ok(n.as_i64().unwrap().to_object(py))
            } else if n.is_u64() {
                Ok(n.as_u64().unwrap().to_object(py))
            } else {
                Ok(n.as_f64().unwrap().to_object(py))
            }
        }
        Value::Bool(b) => Ok(b.to_object(py)),
        Value::Null => Ok(py.None()),
        Value::Array(arr) => {
            let py_list = arr
                .iter()
                .map(|v| json_to_py(py, v))
                .collect::<Result<Vec<PyObject>>>()?;
            Ok(py_list.to_object(py))
        }
        Value::Object(obj) => {
            let py_dict = PyDict::new(py);
            for (k, v) in obj {
                py_dict.set_item(k, json_to_py(py, v)?)?;
            }
            Ok(py_dict.to_object(py))
        }
    }
}

/// Executes a Python function with the provided input values.
///
/// # Arguments
///
/// * `input_values` - A JSON Value containing the keyword arguments to pass to the Python function
/// * `function_code` - A string containing the Python function code to execute
///
/// # Returns
///
/// * `Result<String>` - The string representation of the Python function's return value,
///   or an error if execution fails
///
/// # Example
///
/// ```
/// use serde_json::json;
/// use ryexec::exec;
///
/// let input = json!({
///     "x": 1,
///     "y": 2
/// });
/// let code = "def add(x, y):\n    return x + y";
/// let result = exec(&input, code).unwrap();
/// assert_eq!(result, "3");
/// ```
pub fn exec(input_values: &Value, function_code: &str) -> Result<String> {
    pyo3::prepare_freethreaded_python();

    Python::with_gil(|py| {
        let module = PyModule::from_code(py, function_code, "dynamic_module.py", "dynamic_module")
            .map_err(|e| Error::ModuleCreationError(e.to_string()))?;

        let locals = module.dict();
        let function_names: Vec<String> = locals
            .keys()
            .iter()
            .filter_map(|key| {
                let key_str = key.to_string();
                py.eval(&format!("callable({})", key_str), None, Some(locals))
                    .ok()
                    .and_then(|result| result.extract::<bool>().ok())
                    .and_then(|is_callable| if is_callable { Some(key_str) } else { None })
            })
            .collect();

        if function_names.is_empty() {
            return Err(Error::NoFunctionFound);
        }

        let function_name = &function_names[0];
        let function = locals
            .get_item(function_name)
            .ok_or(Error::FunctionNotFound)?;

        let kwargs = PyDict::new(py);
        if let Some(obj) = input_values.as_object() {
            for (key, value) in obj {
                kwargs.set_item(key, json_to_py(py, value)?)?;
            }
        }

        let result = function.call((), Some(kwargs))?;
        Ok(result.to_string())
    })
}
