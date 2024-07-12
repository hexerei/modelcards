
pub fn validate_modelcard(sources: Vec<String>, schema_file: Option<String>) -> bool {
    if let Ok(modelcard) = modelcards::merge::from_paths(sources) {
        let schema = if schema_file.is_some() {
            schema_file.unwrap()
        } else {
            "modelcard.schema.json".to_string()
        };
        println!("Modelcard: {:?}", modelcard);
        println!("Schema: {:?}", schema);
        return true;
    }
    false
}