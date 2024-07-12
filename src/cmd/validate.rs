use std::path::Path;
use modelcards::utils::load_json_file;
use anyhow::{bail, Result};


pub fn validate_modelcard(sources: Vec<String>, schema_file: Option<String>) -> Result<bool> {
    let result = modelcards::merge::from_paths(sources);
    if let Ok(modelcard) = result {
        let schema = if schema_file.is_some() {
            load_json_file(Path::new(&schema_file.unwrap()))?
        } else {
            serde_json::from_str(&modelcards::assets::schema::get_schema())?
        };
        return modelcards::validate::validate_against_schema(modelcard, Some(schema));
    }
    bail!("Could not construct modelcard source data: {:?}", result.err());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::env::temp_dir;
    use modelcards::utils::create_file;

    #[test]
    fn test_validate_modelcard_with_valid_data_and_no_schema() -> Result<()> {
        let temp_dir = temp_dir();
        let modelcard_path = temp_dir.join("modelcard.json");
        //let mut file = File::create(&modelcard_path)?;
        let content = modelcards::assets::schema::get_sample();
        //writeln!(file, "{content}")?;
        create_file(&modelcard_path, &content)?;

        let sources = vec![modelcard_path.to_str().unwrap().to_string()];
        let result = validate_modelcard(sources, None)?;

        assert!(result);
        Ok(())
    }

    #[test]
    fn test_validate_modelcard_with_invalid_data() -> Result<()> {
        let temp_dir = temp_dir();
        let modelcard_path = temp_dir.join("invalid_modelcard.json");
        //let mut file = File::create(&modelcard_path)?;
        //writeln!(file, r#"{{"invalid": "data"}}"#)?;
        create_file(&modelcard_path, r#"{{"invalid": "data"}}"#)?;

        let sources = vec![modelcard_path.to_str().unwrap().to_string()];
        let result = validate_modelcard(sources, None);

        assert!(result.is_err());
        Ok(())
    }

    // #[test]
    // fn test_validate_modelcard_with_valid_data_and_custom_schema() -> Result<()> {
    //     let temp_dir = temp_dir();
    //     let modelcard_path = temp_dir.join("modelcard.json");
    //     let schema_path = temp_dir.join("schema.json");
    //     // let mut modelcard_file = File::create(&modelcard_path)?;
    //     // let mut schema_file = File::create(&schema_path)?;
    //     // writeln!(modelcard_file, r#"{{"name": "Test Model", "description": "A test model for validation."}}"#)?;
    //     // writeln!(schema_file, r#"{{"type": "object", "properties": {{"name": {{"type": "string"}}, "description": {{"type": "string"}}}}, "required": ["name", "description"]}}"#)?;
    //     create_file(&modelcard_path, r#"{"name": "Test Model", "description": "A test model for validation."}"#).expect("Could not create modelcard file.");
    //     create_file(&schema_path, r#"{"type": "object", "properties": {{"name": {{"type": "string"}}, "description": {{"type": "string"}}}}, "required": ["name", "description"]}"#).expect("Could not create schema file.");

    //     let sources = vec![modelcard_path.to_str().unwrap().to_string()];
    //     let result = validate_modelcard(sources, Some(schema_path.to_str().unwrap().to_string()))?;

    //     assert!(result);
    //     Ok(())
    // }

    #[test]
    fn test_validate_modelcard_fails_with_nonexistent_source() {
        let sources = vec!["nonexistent_modelcard.json".to_string()];
        let result = validate_modelcard(sources, None);

        assert!(result.is_err());
    }

    #[test]
    fn test_validate_modelcard_fails_with_nonexistent_schema() -> Result<()> {
        let temp_dir = temp_dir();
        let modelcard_path = temp_dir.join("modelcard.json");
        // let mut file = File::create(&modelcard_path).unwrap();
        // writeln!(file, r#"{{"name": "Test Model", "description": "A test model for validation."}}"#).unwrap();
        create_file(&modelcard_path, r#"{"name": "Test Model", "description": "A test model for validation."}"#)?;

        let sources = vec![modelcard_path.to_str().unwrap().to_string()];
        let result = validate_modelcard(sources, Some("nonexistent_schema.json".to_string()));

        assert!(result.is_err());
        Ok(())
    }
}