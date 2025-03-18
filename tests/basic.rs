use ryexec::{exec, parse_json};
use serde_json::json;

#[test]
fn test_parse_json() {
    let json_data = json!({
        "function": "def add(a, b):\n    return a + b",
        "input": { "a": 3, "b": 5 }
    });

    let parsed = parse_json(&json_data).unwrap();
    assert_eq!(parsed.function_code, "def add(a, b):\n    return a + b");
    assert_eq!(parsed.input["a"], 3);
    assert_eq!(parsed.input["b"], 5);
}

#[test]
fn test_exec() {
    let input = json!({ "a": 2, "b": 4 });
    let function_code = "def add(a, b):\n    return a + b";
    let result = exec(&input, function_code).unwrap();
    assert_eq!(result, "6");
}

#[test]
fn test_exec_with_missing_function() {
    let input = json!({});
    let function_code = "x = 5"; // No function defined
    let result = exec(&input, function_code);
    assert!(result.is_err());
}
