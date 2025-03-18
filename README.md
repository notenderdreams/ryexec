# ryexec ⚡️

## Overview

**ryexec** is a Rust library that allows you to execute Python functions directly from your Rust code.

This library is designed for scenarios where you need to dynamically execute Python code or leverage Python's extensive ecosystem within your Rust applications. It works fine with external Python Libraries as well.

---

## Features ✨

- **Dynamic Python Execution**: Execute arbitrary Python function from Rust.
- **JSON Input**: Pass structured input to Python functions using JSON.

---

## Installation ⚙️

Add ryexec to your `Cargo.toml` file:

```toml
[dependencies]
ryexec = "0.1.0"
```

---

## How to Use 📋

### Basic Usage

```rust
use ryexec::exec;
use serde_json::json;

fn main() {
    let input = json!({
        "x": 1,
        "y": 2
    });
    let code = "def add(x, y):\n    return x + y";
    let result = exec(&input, code).unwrap();
    assert_eq!(result, "3");
}
```

### Error Handling

The `exec` function returns a `Result<String, anyhow::Error>`, allowing you to handle potential errors gracefully. The `Error` enum provides detailed error types:

- `JsonParse(String)`: Errors in parsing JSON input.
- `MissingFunctionCode`: Function code is missing.
- `NoFunctionFound`: No callable function found in the provided code.
- `PythonExecution(String)`: Errors during Python execution.
- `TypeConversion(String)`: Errors in Python type conversion.

<br>

<div style="text-align: center; margin-top: 30px; padding: 10px;">

<img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT">
<br>
<br>
    <p>Created with ❤️ by <a href="https://github.com/notenderdreams" target="_blank">Sajid Al Nahian</a></p>
    <p>Feel free to fork, contribute, and open issues on GitHub.</p>
</div>
