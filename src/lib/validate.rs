//! # Validation
//! 
//! The `validate` module provides functions to validate a model card against a schema.
//! 
//! ## Functions
//! 
//! The module provides the following functions:
//! 
//! - `check_against_schema` - Check a model card against a schema.
//! - `validate_against_schema` - Validate a model card against a schema.
//! 
//! ## Errors
//! 
//! The functions will return an error if the model card is not valid against the schema.
//! The [`anyhow`] crate is used for error handling.
//! 

use std::path::Path;
use crate::{assets, utils::load_json_file};
use anyhow::{bail, Result};
use serde_json::Value;

/// Check a model card against a schema.
/// 
/// The function takes a path to a model card and a path to a schema and checks the model card against the schema.
/// 
/// ## Arguments
/// 
/// - `path` - A path to a model card or a directory containing a model card.
/// - `modelcard` - A path to a schema file.
/// 
/// ## Returns
/// 
/// The function returns a `Result` with a boolean indicating whether the model card is valid against the schema.
/// 
/// ## Errors
/// 
/// The function will return an error if the model card is not valid against the schema.
/// 
/// ## Example
/// 
/// ```rust,no_run
/// use std::path::Path;
/// use modelcards::validate::check_against_schema;
/// 
/// let schema_path = Path::new("tests/schemas/my_schema.json");
/// let data_path = Path::new("tests/data/sample.json");
/// let result = check_against_schema(schema_path, data_path).unwrap();
/// assert_eq!(result, true);
/// ```
/// 
pub fn check_against_schema(path: &Path, modelcard: &Path) -> Result<bool> {

    if !path.exists() {
        bail!("Path does not exist: {:?}", path);
    }

    let schema_file = if path.is_dir() {
        //TODO: get schema from config
        path.join("schema/modelcard.schema.json")
    } else {
        path.to_path_buf()
    };
    let schema_v7 = load_json_file(&schema_file)?;
    let modelcard = load_json_file(modelcard)?;

    validate_against_schema(modelcard, Some(schema_v7))
}

/// Validate a model card against a schema.
/// 
/// The function takes a model card and a schema and validates the model card against the schema.
/// 
/// ## Arguments
/// 
/// - `modelcard` - A JSON object representing the model card.
/// - `schema` - A JSON object representing the schema.
/// 
/// ## Returns
/// 
/// The function returns a `Result` with a boolean indicating whether the model card is valid against the schema.
/// 
/// ## Errors
/// 
/// The function will return an error if the model card is not valid against the schema.
/// 
/// ## Example
/// 
/// ```rust
/// use serde_json::json;
/// use modelcards::validate::validate_against_schema;
/// 
/// let modelcard = json!({
///     "name": "Model Name",
///     "schema_version": "0.0.2"
/// });
/// let schema = json!({
///     "type": "object",
///     "properties": {
///         "name": {
///             "type": "string"
///         },
///         "schema_version": {
///             "type": "string"
///         }
///     },
///     "required": ["name", "schema_version"]
/// });
/// let result = validate_against_schema(modelcard, Some(schema)).unwrap();
/// assert_eq!(result, true);
/// ```
/// 
pub fn validate_against_schema(modelcard: Value, schema: Option<Value>) -> Result<bool> {

    let schema_v7 = match schema {
        Some(s) => s,
        None => serde_json::from_str(assets::schema::get_schema())?,
    };

    let validator = jsonschema::validator_for(&schema_v7)
        .map_err(|e| anyhow::anyhow!("Could not compile schema: {}", e))?;

    let errors: Vec<String> = validator
        .iter_errors(&modelcard)
        .map(|e| {
            let path = e.instance_path();
            if path.as_str().is_empty() {
                format!("{}", e)
            } else {
                format!("{}: {}", path, e)
            }
        })
        .collect();

    if !errors.is_empty() {
        bail!("Validation failed:\n{}", errors.join("\n"));
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::assets::schema;
    use crate::utils::create_file;

    use std::{
        env::temp_dir,
        fs::{create_dir, remove_dir_all},
        path::{PathBuf, Path}
    };
    use anyhow::Result;

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

    fn populate_modelcards_dir(path: &Path) -> Result<()> {
        create_file(&path.join("sample.json"), schema::get_sample())?;
        create_dir(path.join("schema"))?;
        create_file(&path.join("schema/modelcard.schema.json"), schema::get_schema())?;
        Ok(())
    }

    #[test]
    fn check_valid_against_schema() {
        let dir = get_temp_dir("test_check_against_schema", true);
        populate_modelcards_dir(&dir).expect("Could not populate modelcards directory");
        assert!(check_against_schema(&dir, &dir.join("sample.json")).is_ok());
    }

    #[test]
    fn check_valid_against_missing_schema() {
        let dir = get_temp_dir("test_check_missing_schema", true);
        populate_modelcards_dir(&dir).expect("Could not populate modelcards directory");
        //force error with missing schema
        assert!(!check_against_schema(&dir, &dir.join("sample_2.json")).is_ok());
    }
}