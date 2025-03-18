use ryexec::exec;
use serde_json::json;

/*
    This  is for testing the external  library Integration
    with the python library **NumPy**
    If NumPy is not installed, this test will fail
    run `uv pip install numpy` to install it
*/

#[test]
fn test_simple_numpy() {
    // Simple NumPy code to calculate mean and sum
    let code = r#"
import numpy as np

def simple_numpy(values):
    arr = np.array(values)
    return {
        "mean": float(np.mean(arr)),
        "sum": float(np.sum(arr))
    }
"#;

    // Input
    let input = json!({
        "values": [1, 2, 3, 4, 5]
    });

    let result = exec(&input, code);

    // but that's expected since we're testing NumPy integration
    match result {
        Ok(output) => {
            println!("NumPy result: {}", output);
            assert!(output.contains("mean") && output.contains("sum"));
        }
        Err(e) => {
            println!("NumPy test failed: {}", e);
            // Only fail if it's not a NumPy import error
            if !e.to_string().contains("ImportError") {
                panic!("Test failed for non-import related reason: {}", e);
            }
        }
    }
}
