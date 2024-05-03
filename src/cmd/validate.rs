//use std::path::Path;


pub fn validate_modelcard(data: String, schema_file: Option<String>, defaults: Option<String>) -> bool {
    let schema = if schema_file.is_some() {
        schema_file.unwrap()
    } else {
        "modelcard.schema.json".to_string()
    };
    true
}