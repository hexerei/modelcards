use minijinja::{Environment, path_loader};
use serde_json::Value;

use std::{fs::read_to_string, ffi::OsStr, path::Path};
use crate::{
    utils::console,
    validate::check_against_schema
};
use anyhow::{bail, Result};

/// Render a template with a data file to String
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