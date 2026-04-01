//! Schema loading and inference — generate Draft-07 JSON Schema from data.

use std::path::Path;
use anyhow::{Context, Result};
use serde_json::{json, Value};

/// Load a JSON file and parse it as a serde_json::Value.
pub fn load_json(path: &Path) -> Result<Value> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;
    serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse JSON: {}", path.display()))
}

/// Infer a Draft-07 JSON Schema from a JSON value.
///
/// Walks the JSON tree and produces a schema that describes its structure:
/// - Objects → `{"type": "object", "properties": {...}}`
/// - Strings → `{"type": "string"}`
/// - Numbers (integer) → `{"type": "integer"}`
/// - Numbers (float) → `{"type": "number"}`
/// - Booleans → `{"type": "boolean"}`
/// - Arrays → `{"type": "array", "items": {...}}` (inferred from first element)
/// - Null → `{}` (any type)
pub fn infer_schema(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut properties = serde_json::Map::new();
            let mut required = Vec::new();
            for (key, val) in map {
                properties.insert(key.clone(), infer_schema(val));
                required.push(Value::String(key.clone()));
            }
            let mut schema = serde_json::Map::new();
            schema.insert("type".to_string(), json!("object"));
            schema.insert("properties".to_string(), Value::Object(properties));
            if !required.is_empty() {
                schema.insert("required".to_string(), Value::Array(required));
            }
            Value::Object(schema)
        }
        Value::Array(arr) => {
            let mut schema = serde_json::Map::new();
            schema.insert("type".to_string(), json!("array"));
            if let Some(first) = arr.first() {
                schema.insert("items".to_string(), infer_schema(first));
            }
            Value::Object(schema)
        }
        Value::String(_) => json!({"type": "string"}),
        Value::Number(n) => {
            if n.is_i64() || n.is_u64() {
                json!({"type": "integer"})
            } else {
                json!({"type": "number"})
            }
        }
        Value::Bool(_) => json!({"type": "boolean"}),
        Value::Null => json!({}),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn infer_string() {
        assert_eq!(infer_schema(&json!("hello")), json!({"type": "string"}));
    }

    #[test]
    fn infer_integer() {
        assert_eq!(infer_schema(&json!(42)), json!({"type": "integer"}));
    }

    #[test]
    fn infer_float() {
        assert_eq!(infer_schema(&json!(3.14)), json!({"type": "number"}));
    }

    #[test]
    fn infer_boolean() {
        assert_eq!(infer_schema(&json!(true)), json!({"type": "boolean"}));
    }

    #[test]
    fn infer_null() {
        assert_eq!(infer_schema(&json!(null)), json!({}));
    }

    #[test]
    fn infer_empty_array() {
        assert_eq!(infer_schema(&json!([])), json!({"type": "array"}));
    }

    #[test]
    fn infer_array_of_strings() {
        assert_eq!(
            infer_schema(&json!(["a", "b"])),
            json!({"type": "array", "items": {"type": "string"}})
        );
    }

    #[test]
    fn infer_simple_object() {
        let schema = infer_schema(&json!({"name": "test", "count": 5}));
        assert_eq!(schema["type"], "object");
        assert_eq!(schema["properties"]["name"], json!({"type": "string"}));
        assert_eq!(schema["properties"]["count"], json!({"type": "integer"}));
    }

    #[test]
    fn infer_nested_object() {
        let data = json!({
            "outer": {
                "inner": "value"
            }
        });
        let schema = infer_schema(&data);
        assert_eq!(schema["type"], "object");
        assert_eq!(schema["properties"]["outer"]["type"], "object");
        assert_eq!(
            schema["properties"]["outer"]["properties"]["inner"],
            json!({"type": "string"})
        );
    }

    #[test]
    fn infer_array_of_objects() {
        let data = json!([{"a": 1}, {"b": 2}]);
        let schema = infer_schema(&data);
        assert_eq!(schema["type"], "array");
        assert_eq!(schema["items"]["type"], "object");
    }

    #[test]
    fn infer_mixed_structure() {
        let data = json!({
            "name": "model",
            "version": 1,
            "tags": ["ml", "nlp"],
            "config": {
                "enabled": true,
                "threshold": 0.95
            }
        });
        let schema = infer_schema(&data);
        assert_eq!(schema["properties"]["name"], json!({"type": "string"}));
        assert_eq!(schema["properties"]["version"], json!({"type": "integer"}));
        assert_eq!(schema["properties"]["tags"]["type"], "array");
        assert_eq!(schema["properties"]["tags"]["items"], json!({"type": "string"}));
        assert_eq!(schema["properties"]["config"]["properties"]["enabled"], json!({"type": "boolean"}));
        assert_eq!(schema["properties"]["config"]["properties"]["threshold"], json!({"type": "number"}));
    }
}
