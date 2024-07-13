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
    use modelcards::utils::create_file;
    use std::{
        env::temp_dir,
        fs::{create_dir, remove_dir_all},
        path::PathBuf
    };

    fn get_temp_dir(path: &str, create: bool) -> PathBuf {
        let mut dir = temp_dir();
        dir.push(path);
        if dir.exists() {
            remove_dir_all(&dir).expect("Could not free test directory");
        }
        if create {
            create_dir(&dir).expect("Could not create test directory");
        }
        dir
    }

    #[test]
    fn test_validate_modelcard_with_valid_data_and_no_schema() -> Result<()> {
        let temp_dir = get_temp_dir("mc_valid_data_no_schema", true);
        let modelcard_path = temp_dir.join("modelcard.json");
        let content = modelcards::assets::schema::get_sample();
        create_file(&modelcard_path, &content)?;

        let sources = vec![modelcard_path.to_str().unwrap().to_string()];
        let result = validate_modelcard(sources, None)?;

        assert!(result);
        Ok(())
    }

    #[test]
    fn test_validate_modelcard_with_invalid_data() -> Result<()> {
        let temp_dir = get_temp_dir("mc_invalid_data", true);
        let modelcard_path = temp_dir.join("invalid_modelcard.json");
        create_file(&modelcard_path, r#"{{"invalid": "data"}}"#)?;

        let sources = vec![modelcard_path.to_str().unwrap().to_string()];
        let result = validate_modelcard(sources, None);

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_validate_modelcard_with_valid_data_and_custom_schema() -> Result<()> {
        let temp_dir = get_temp_dir("mc_valid_data_custom_schema", true);
        let modelcard_path = temp_dir.join("modelcard.json");
        let schema_path = temp_dir.join("schema.json");
        create_file(&modelcard_path, r#"{"name": "Test Model", "description": "A test model for validation."}"#).expect("Could not create modelcard file.");
        create_file(&schema_path, r#"{"type": "object", "properties": {"name": {"type": "string"}, "description": {"type": "string"}}, "required": ["name", "description"]}"#).expect("Could not create schema file.");

        let sources = vec![modelcard_path.to_str().unwrap().to_string()];
        let result = validate_modelcard(sources, Some(schema_path.to_str().unwrap().to_string()))?;

        assert!(result);
        Ok(())
    }

    #[test]
    fn test_validate_modelcard_fails_with_nonexistent_source() {
        let sources = vec!["nonexistent_modelcard.json".to_string()];
        let result = validate_modelcard(sources, None);

        assert!(result.is_err());
    }

    #[test]
    fn test_validate_modelcard_fails_with_nonexistent_schema() -> Result<()> {
        let temp_dir = get_temp_dir("mc_valid_data_nofile_schema", true);
        let modelcard_path = temp_dir.join("modelcard.json");
        create_file(&modelcard_path, r#"{"name": "Test Model", "description": "A test model for validation."}"#)?;

        let sources = vec![modelcard_path.to_str().unwrap().to_string()];
        let result = validate_modelcard(sources, Some("nonexistent_schema.json".to_string()));

        assert!(result.is_err());
        Ok(())
    }
}