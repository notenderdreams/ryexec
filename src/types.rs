use crate::error::Error;
use serde_json::Value;

#[derive(Debug)]
pub struct ParsedFunction {
    pub input: Value,
    pub function_code: String,
}

/// Parses a JSON value into a ParsedFunction struct
///
/// # Arguments
///
/// * `json_data` - A reference to a serde_json Value containing the function code and input data
///
/// # Returns
///
/// * `Result<ParsedFunction>` - A Result containing either:
///   * Ok(ParsedFunction) - Successfully parsed function with input values and code
///   * Err - If function code is missing or invalid JSON structure
///
/// # Example
///
/// ```
/// use serde_json::json;
/// use ryexec::parse_json;
///
/// let json = serde_json::json!({
///     "function": "def example(): pass",
///     "input": {"arg": 1}
/// });
/// let parsed = parse_json(&json).unwrap();
/// ```
pub fn parse_json(json_data: &Value) -> Result<ParsedFunction, crate::Error> {
    let function_code = json_data["function"]
        .as_str()
        .ok_or(Error::MissingFunctionCode)?
        .to_string();

    let input_values = json_data["input"].clone();

    Ok(ParsedFunction {
        input: input_values,
        function_code,
    })
}
