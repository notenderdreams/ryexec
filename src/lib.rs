//! A library for executing Python code from Rust using PyO3

mod error;
mod executor;
mod types;

pub use error::Error;
pub use executor::exec;
pub use types::parse_json;
pub use types::ParsedFunction;

pub type Result<T> = std::result::Result<T, Error>;
