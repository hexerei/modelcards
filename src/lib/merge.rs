//#[macro_use]
//extern crate serde_json;
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