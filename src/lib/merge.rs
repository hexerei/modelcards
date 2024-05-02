//#[macro_use]
//extern crate serde_json;

use serde_json::Value;

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
}