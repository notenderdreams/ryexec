use ryexec::parse_json;
use serde_json::json;

#[test]
fn test_parse_json_valid() {
    let json = json!({
        "function": "def test(): return 42",
        "input": {"x": 10, "y": 20}
    });

    let result = parse_json(&json);
    assert!(result.is_ok());

    let parsed = result.unwrap();
    assert_eq!(parsed.function_code, "def test(): return 42");
    assert_eq!(parsed.input, json!({ "x": 10, "y": 20 }));
}

#[test]
fn test_parse_json_missing_function() {
    let json = json!({
        "input": {"x": 10}
    });

    let result = parse_json(&json);
    assert!(result.is_err());
}

#[test]
fn test_parse_json_empty_input() {
    let json = json!({
        "function": "def test(): pass",
        "input": {}
    });

    let result = parse_json(&json);
    assert!(result.is_ok());

    let parsed = result.unwrap();
    assert_eq!(parsed.input, json!({}));
}
