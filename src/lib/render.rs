//! # Render
//! 
//! The `render` module provides functions to render Jinja templates with JSON data.
//! 
//! ## Functions
//! 
//! The module provides the following functions:
//! 
//! - `render_template(template: &Path, data: &Path) -> Result<String>` - Render a template with a data file to String.
//! - `render_template_valid(template: &Path, data: &Path, schema: &Path) -> Result<String>` - Render a template with a data file to String and validate against a schema.
//! - `render_value_to_template(data: Value, template: Option<&Path>) -> Result<String>` - Render a template with a JSON object.
//! 
//! ## Errors
//! 
//! The functions will return an error if the template, data, or schema file could not be found or if the JSON object does not validate against the schema.
//! The anyhow crate is used for error handling.
//! 

use minijinja::{Environment, path_loader};
use serde_json::Value;

use std::{fs::read_to_string, ffi::OsStr, path::Path};
use crate::{
    utils::console,
    validate::check_against_schema
};
use anyhow::{bail, Result};

/// Render a template with a data file to String
/// 
/// The function takes a JSON file path and a template file and renders the template with the JSON object.
/// 
/// ## Arguments
/// 
/// - `template` - A path to a Jinja template file.
/// - `data` - A path to a JSON file to be rendered.
/// 
/// ## Returns
/// 
/// The function returns a `Result` with the rendered template as a `String` or an error if the template could not be rendered.
/// 
/// ## Errors
/// 
/// The function will return an error if the template or data file could not be found.
/// 
/// ## Example
/// 
/// ```rust
/// use std::path::Path;
/// use crate::render::render_template;
/// 
/// let template = Path::new("template.md.jinja"); // content: "Hello, {{ name }}!"
/// let data = Path::new("data.json"); // content: {"name": "World"}
/// let result = render_template(&template, &data).unwrap();
/// assert_eq!(result, "Hello, World!");
/// ```
/// 
pub fn render_template(template: &Path, data: &Path) -> Result<String> {

    // // check if template exists
    // if !template.is_file() {
    //     bail!("Template file does not exist at '{}'", template.to_string_lossy().to_string());
    // }

    // check if data exists
    if !data.is_file() {
        bail!("Modelcard file '{}' does not exist.", data.to_string_lossy().to_string());
    }

    // console::debug("Rendering template...");
    // console::debug(&format!("Template: {}", template.to_string_lossy().to_string()));
    // console::debug(&format!("Data: {}", data.to_string_lossy().to_string()));

    // let template_name = template.file_name().unwrap_or(OsStr::new("modelcard.md.jinja")).to_str().unwrap();
    // let template_content = read_to_string(template).unwrap();

    // //let env = create_env();
    // let mut env = Environment::new();
    // env.add_template(template_name, template_content.as_str())?;

    // let template = env.get_template(template_name).unwrap();
    let data = crate::utils::load_json_file(data)?;
    // let tmp = template.render(&data);

    render_value_to_template(data, Some(template))

    // if let Err(e) = tmp {
    //     bail!("Could not render template: {:?}", e);
    // }

    // console::debug("Done!");

    // Ok(tmp.unwrap())
}

/// Render a template with a data file to String and validate against a schema
/// 
/// The function takes a JSON file path, a template file, and a schema file and renders the template with the JSON object if the JSON object validates against the given schema.
/// 
/// ## Arguments
/// 
/// - `template` - A path to a Jinja template file.
/// - `data` - A path to a JSON file to be rendered.
/// - `schema` - A path to a JSON schema file.
/// 
/// ## Returns
/// 
/// The function returns a `Result` with the rendered template as a `String` or an error if the template could not be rendered or the JSON object does not validate against the schema.
/// 
/// ## Errors
/// 
/// The function will return an error if the template, data, or schema file could not be found or if the JSON object does not validate against the schema.
/// 
/// ## Example
/// 
/// ```rust
/// use std::path::Path;
/// use crate::render::render_template_valid;
/// 
/// let template = Path::new("template.md.jinja"); // content: "Hello, {{ name }}!"
/// let data = Path::new("data.json"); // content: {"name": "World"}
/// let schema = Path::new("schema.json"); // content: {"type": "object", "properties": {"name": {"type": "string"}}}
/// let result = render_template_valid(&template, &data, &schema).unwrap();
/// assert_eq!(result, "Hello, World!");
/// ```
/// 
pub fn render_template_valid(template: &Path, data: &Path, schema: &Path) -> Result<String> {

    // // check if template exists
    // if !template.is_file() {
    //     bail!("Template file does not exist at '{}'", template.to_string_lossy().to_string());
    // }

    // check if data exists
    if !data.is_file() {
        bail!("Modelcard file '{}' does not exist.", data.to_string_lossy().to_string());
    }

    // check if schema exists
    if !schema.is_file() {
        bail!("Modelcard file '{}' does not exist.", schema.to_string_lossy().to_string());
    }

    // check if data validates agains schema
    if let Err(e) = check_against_schema(schema, data) {
        bail!("Project could not be validated!\n{:?}", e);
    }

    // console::debug("Rendering template...");
    // console::debug(&format!("Template: {}", template.to_string_lossy().to_string()));
    // console::debug(&format!("Data: {}", data.to_string_lossy().to_string()));

    // let template_name = template.file_name().unwrap_or(OsStr::new("modelcard.md.jinja")).to_str().unwrap();
    // let template_content = read_to_string(template).unwrap();

    // //let env = create_env();
    // let mut env = Environment::new();
    // env.add_template(template_name, template_content.as_str())?;

    // let template = env.get_template(template_name).unwrap();
    let data = crate::utils::load_json_file(data)?;
    // let tmp = template.render(&data);

    render_value_to_template(data, Some(template))

    // if let Err(e) = tmp {
    //     bail!("Could not render template: {:?}", e);
    // }

    // console::debug("Done!");

    // Ok(tmp.unwrap())
}

/// Render a template with a data file to String
/// 
/// The function takes a JSON object and a template file and renders the template with the JSON object.
/// If no template is provided, the default template will be used (currently Google model card toolkit markdown format).
/// 
/// ## Arguments
/// 
/// - `data` - A JSON object to be rendered.
/// - `template` - An optional path to a template file. If not provided, the default template will be used.
/// 
/// ## Returns
/// 
/// The function returns a `Result` with the rendered template as a `String` or an error if the template could not be rendered.
/// 
/// ## Errors
/// 
/// The function will return an error if the template could not be rendered.
/// 
/// ## Example
/// 
/// ```rust
/// use std::path::Path;
/// use serde_json::json;
/// use crate::render::render_value_to_template;
/// 
/// let data = json!({
///    "name": "World"
/// });
/// let jinja_file = Path::new("template.md.jinja"); // content: "Hello, {{ name }}!"
/// let result = render_value_to_template(data, None).unwrap();
/// assert_eq!(result, "Hello, World!");
/// ```
/// 
pub fn render_value_to_template(data: Value, template: Option<&Path>) -> Result<String> {

    let template_name: &str;
    let template_content: String;

    if template.is_none() {
        template_name = "default_md";
        template_content = crate::assets::templates::get_md();
    } else {
        let template = template.unwrap();
        // check if template exists
        if !template.is_file() {
            bail!("Template file does not exist at '{}'", template.to_string_lossy().to_string());
        }

        template_name = template.file_name().unwrap_or(OsStr::new("modelcard.md.jinja")).to_str().unwrap();
        template_content = read_to_string(template).unwrap();
    }

    console::debug("Rendering template...");
    console::debug(&format!("Template: {}", template_name));

    //let env = create_env();
    let mut env = Environment::new();
    env.add_template(template_name, template_content.as_str())?;

    let template = env.get_template(template_name).unwrap();
    let tmp = template.render(&data);

    if let Err(e) = tmp {
        bail!("Could not render template: {:?}", e);
    }

    console::debug("Done!");

    Ok(tmp.unwrap())
}


#[allow(dead_code)]
#[doc(hidden)]
fn create_env(path: &Path) -> Environment<'static> {
    //TODO loader needs to get path from config or cli
    let mut env = Environment::new();
    env.set_loader(path_loader(path));
    env
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::create_file;

    use std::{
        env::temp_dir,
        fs::{create_dir, remove_dir_all},
        path::PathBuf
    };

    fn get_temp_dir(path: &str, create: bool) -> PathBuf {
        let mut dir = temp_dir();
        dir.push(path);
        if dir.try_exists().expect("Could not check test directory") {
            remove_dir_all(&dir).expect("Could not free test directory");
        }
        if create {
            create_dir(&dir).expect("Could not create test directory");
        }
        dir
    }


    fn setup_test_environment(name: &str, template_content: &str, data_content: &str, schema_content: &str) -> (PathBuf, PathBuf, PathBuf) {
        let dir = get_temp_dir(name, true);
        let template_path = dir.join("template.md.jinja");
        let data_path = dir.join("data.json");
        let schema_path = dir.join("schema.json");

        // Write template content
        create_file(template_path.as_path(), template_content).expect("Could not create template file");

        // Write data content
        create_file(data_path.as_path(), data_content).expect("Could not create data file");

        // Write schema content
        create_file(schema_path.as_path(), schema_content).expect("Could not create data file");

        (template_path, data_path, schema_path)
    }

    #[test]
    fn test_render_template_valid() {
        let (template_path, data_path, schema_path) = setup_test_environment(
            "render_template_valid",
            "Hello, {{ name }}!",
            r#"{"name": "World"}"#,
            r#"{"type": "object", "properties": {"name": {"type": "string"}}}"#,
        );

        let result = render_template_valid(&template_path, &data_path, &schema_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, World!");
    }

    #[test]
    fn test_render_template_with_missing_data() {
        let (template_path, _, schema_path) = setup_test_environment(
            "render_template_with_missing_data",
            "Hello, {{ name }}!",
            r#"{"name": "World"}"#,
            r#"{"type": "object", "properties": {"name": {"type": "string"}}}"#,
        );

        let missing_data_path = Path::new("nonexistent_data.json");
        let result = render_template_valid(&template_path, &missing_data_path, &schema_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_render_template_with_invalid_schema() {
        let (template_path, data_path, _) = setup_test_environment(
            "render_template_with_invalid_schema",
            "Hello, {{ name }}!",
            r#"{"name": "World"}"#,
            r#"{"type": "object", "properties": {"name": {"type": "number"}}}"#,
        );

        let schema_path = Path::new("invalid_schema.json");
        let result = render_template_valid(&template_path, &data_path, &schema_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_render_value_to_template_with_default_template() {
        let content = crate::assets::schema::get_sample();
        let data = serde_json::from_str(&content).unwrap();
        let result = render_value_to_template(data, None);
        assert!(result.is_ok());
        // Assuming the default template content is "Hello, {{ name }}!"
        //assert_eq!(result.unwrap(), "Hello, World!");
    }

    #[test]
    fn test_render_value_to_template_with_custom_template() {
        let (template_path, data_path, _) = setup_test_environment(
            "render_value_to_template_with_custom_template",
            "Goodbye, {{ name }}!",
            r#"{"name": "World"}"#,
            "",
        );

        let data = crate::utils::load_json_file(&data_path).unwrap();
        let result = render_value_to_template(data, Some(&template_path));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Goodbye, World!");
    }
}