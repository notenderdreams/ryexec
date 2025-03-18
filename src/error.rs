use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("JSON parsing error: {0}")]
    JsonParse(String),

    #[error("Function code not found in input")]
    MissingFunctionCode,

    #[error("No callable function found in the provided code")]
    NoFunctionFound,

    #[error("Python execution error: {0}")]
    PythonExecution(String),

    #[error("Python type conversion error: {0}")]
    TypeConversion(String),

    #[error("Function not found in locals")]
    FunctionNotFound,

    #[error("Failed to set kwargs item")]
    KwargsError,

    #[error("Failed to create Python module: {0}")]
    ModuleCreationError(String),
}

impl From<pyo3::PyErr> for Error {
    fn from(err: pyo3::PyErr) -> Self {
        Error::PythonExecution(err.to_string())
    }
}
