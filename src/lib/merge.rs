//! Merge JSON files or strings into a single JSON object.
//! 
//! The module provides functions to merge multiple JSON files or strings into a single JSON object.
//! 
//! ## Functions
//! 
//! The module provides the following functions:
//! 
//! - `from_paths(sources: Vec<String>) -> Result<Value>` - Merge multiple JSON files into a single JSON object.
//! - `from_strings(strings: Vec<String>) -> Result<Value>` - Merge multiple JSON strings into a single JSON object.
//! - `merge(a: &mut Value, b: Value)` - Merge two JSON values recursively.
//! 
//! ## Errors
//! 
//! The functions will return an error if any of the files are not found or don't include valid JSON strings.
//! The anyhow crate is used for error handling.
//! 
//! ## Example
//! 
//! ```rust
//! use serde_json::json;
//! use crate::merge::from_paths;
//! 
//! let sources = vec![
//!     "tests/data/a.json".to_string(), // contains {"a": 1}
//!     "tests/data/b.json".to_string()   // contains {"b": 2}
//! ];
//! let result = from_paths(sources).unwrap();
//! let expected = json!({
//!     "a": 1,
//!     "b": 2
//! });
//! assert_eq!(result, expected);
//! ```
//! 

use std::fs;
use serde_json::Value;
use anyhow::{bail, Result};


/* fn merge(a: &Value, b: &Value) -> Value {
    match (a, b) {
        (Value::Object(a), Value::Object(b)) => {
            let mut res = a.clone();
            for (k, v) in b {
                res.insert(k.clone(), v.clone());
            }
            Value::Object(res)
        }
        (Value::Array(a), Value::Array(b)) => {
            let mut res = a.clone();
            res.extend(b.clone());
            Value::Array(res)
        }
        (_, b) => b.clone(),
    }
}
*/


/// Merge multiple JSON files into a single JSON object.
/// 
/// The function takes a vector of file paths and merges the JSON objects in the files into a single JSON object.
/// 
/// ## Arguments
/// 
/// - `sources` - A vector of file paths to JSON files to be merged.
/// 
/// ## Returns
/// 
/// The function returns a `Result` with the merged JSON object or an error if the files are not found or don't include valid JSON strings.
/// 
/// ## Errors
/// 
/// The function will return an error if any of the files are not found or don't include valid JSON strings.
/// 
/// ## Example
/// 
/// ```rust
/// use serde_json::json;
/// use crate::merge::from_paths;
/// 
/// let sources = vec![
///     "tests/data/a.json".to_string(), // contains {"a": 1}
///     "tests/data/b.json".to_string()   // contains {"b": 2}
/// ];
/// let result = from_paths(sources).unwrap();
/// let expected = json!({
///     "a": 1,
///     "b": 2
/// });
/// assert_eq!(result, expected);
/// ```
/// 
 pub fn from_paths(sources: Vec<String>) -> Result<Value> {
    let mut modelcards = Vec::new();
    for source in sources {
        let result = fs::read_to_string(&source);
        if result.is_err() {
            bail!("Could not read source {}. Error: {:?}", source, result.err().unwrap());
        }
        modelcards.push(result.unwrap());
    }
    from_strings(modelcards)
}
 
/// Merge multiple JSON strings into a single JSON object.
/// 
/// The function takes a vector of JSON strings and merges them into a single JSON object.
/// 
/// ## Arguments
/// 
/// - `strings` - A vector of valid JSON strings to be merged.
/// 
/// ## Returns
/// 
/// The function returns a `Result` with the merged JSON object or an error if the JSON strings are invalid.
/// 
/// ## Errors
/// 
/// The function will return an error if any of the JSON strings are invalid.
/// 
/// ## Example
/// 
/// ```rust
/// use serde_json::json;
/// use crate::merge::from_strings;
/// 
/// let strings = vec![
///     "{\"a\": 1}".to_string(),
///     "{\"b\": 2}".to_string()
/// ];
/// let result = from_strings(strings).unwrap();
/// let expected = json!({
///     "a": 1,
///     "b": 2
/// });
/// assert_eq!(result, expected);
/// ```
/// 
pub fn from_strings(strings: Vec<String>) -> Result<Value> {
    let mut result = Value::Object(serde_json::Map::new());
    for string in strings {
        match serde_json::from_str(&string) {
            Ok(json) => merge(&mut result, json),
            Err(e) => bail!("Invalid json:\n{string}\n\nError: {:?}", e),
        }
    }
    Ok(result)
}

/// Merge two JSON values recursively.
/// 
/// The function takes two deserialized JSON objects and merges them on value level recursively.
/// 
/// ## Arguments
/// - `a` - A mutable reference to the first JSON value.
/// - `b` - The second JSON value which will overwrite existing entries in `a` or add them to `a`.
/// 
/// ## Example
/// 
/// ```rust
/// use serde_json::json;
/// use crate::merge::merge;
/// 
/// let mut a = json!({
///     "a": 1,
///     "b": 2,
///     "c": [1, 2, 3],
///     "d": {
///         "e": 3,
///         "f": 4
///     }
/// });
/// let b = json!({
///     "b": 3,
///     "c": [4, 5, 6],
///     "d": {
///         "f": 5,
///         "g": 6
///     }
/// });
/// merge(&mut a, b);
/// let expected = json!({
///     "a": 1,
///     "b": 3,
///     "c": [4, 5, 6],
///     "d": {
///         "e": 3,
///         "f": 5,
///         "g": 6
///     }
/// });
/// assert_eq!(a, expected);
/// ```
/// 
fn merge(a: &mut Value, b: Value) {
    match (a, b) {
        (a @ &mut Value::Object(_), Value::Object(b)) => {
            let a = a.as_object_mut().unwrap();
            for (k, v) in b {
                merge(a.entry(k).or_insert(Value::Null), v);
            }
        }
        (a, b) => *a = b,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_merge() {
        let mut a = json!({
            "a": 1,
            "b": 2,
            "c": [1, 2, 3],
            "d": {
                "e": 3,
                "f": 4
            }
        });
        let b = json!({
            "b": 3,
            "c": [4, 5, 6],
            "d": {
                "f": 5,
                "g": 6
            }
        });
        merge(&mut a, b);
        let expected = json!({
            "a": 1,
            "b": 3,
            "c": [4, 5, 6],
            "d": {
                "e": 3,
                "f": 5,
                "g": 6
            }
        });
        assert_eq!(a, expected);
    }

    #[test]
    fn test_merge_overwrite_primitive() {
        let mut a = json!({
            "a": 1,
            "b": 2
        });
        let b = json!({
            "b": 3,
            "c": 4
        });
        merge(&mut a, b);
        let expected = json!({
            "a": 1,
            "b": 3,
            "c": 4
        });
        assert_eq!(a, expected);
    }

    #[test]
    fn test_merge_nested_object() {
        let mut a = json!({
            "a": {
                "b": 1,
                "c": 2
            }
        });
        let b = json!({
            "a": {
                "c": 3,
                "d": 4
            }
        });
        merge(&mut a, b);
        let expected = json!({
            "a": {
                "b": 1,
                "c": 3,
                "d": 4
            }
        });
        assert_eq!(a, expected);
    }

    #[test]
    fn test_merge_with_array_replacement() {
        let mut a = json!({
            "a": [1, 2, 3]
        });
        let b = json!({
            "a": [4, 5, 6]
        });
        merge(&mut a, b);
        let expected = json!({
            "a": [4, 5, 6]
        });
        assert_eq!(a, expected);
    }

    #[test]
    fn test_merge_adds_new_elements() {
        let mut a = json!({
            "a": 1
        });
        let b = json!({
            "b": 2
        });
        merge(&mut a, b);
        let expected = json!({
            "a": 1,
            "b": 2
        });
        assert_eq!(a, expected);
    }

    #[test]
    fn test_merge_from_strings() {
        let strings = vec![
            "{\"a\": 1}".to_string(),
            "{\"b\": 2}".to_string()
        ];
        let result = from_strings(strings).unwrap();
        let expected = json!({
            "a": 1,
            "b": 2
        });
        assert_eq!(result, expected);
    }

    #[test]
    fn test_merge_from_strings_with_error() {
        let strings = vec![
            "{\"a\": 1}".to_string(),
            "invalid json".to_string()
        ];
        let result = from_strings(strings);
        assert!(result.is_err());
    }
}