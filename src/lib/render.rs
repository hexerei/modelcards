use minijinja::{Environment, path_loader};

use std::{fs::read_to_string, ffi::OsStr, path::Path};
use crate::{
    utils::console,
    validate::check_against_schema
};
use anyhow::{bail, Result};

/// Render a template with a data file to String
pub fn render_template(template: &Path, data: &Path) -> Result<String> {

    // check if template exists
    if !template.is_file() {
        bail!("Template file does not exist at '{}'", template.to_string_lossy().to_string());
    }

    // check if data exists
    if !data.is_file() {
        bail!("Modelcard file '{}' does not exist.", data.to_string_lossy().to_string());
    }

    console::debug("Rendering template...");
    console::debug(&format!("Template: {}", template.to_string_lossy().to_string()));
    console::debug(&format!("Data: {}", data.to_string_lossy().to_string()));

    let template_name = template.file_name().unwrap_or(OsStr::new("modelcard.md.jinja")).to_str().unwrap();
    let template_content = read_to_string(template).unwrap();

    //let env = create_env();
    let mut env = Environment::new();
    env.add_template(template_name, template_content.as_str())?;

    let template = env.get_template(template_name).unwrap();
    let data = crate::utils::load_json_file(data)?;
    let tmp = template.render(&data);

    if let Err(e) = tmp {
        bail!("Could not render template: {:?}", e);
    }

    console::debug("Done!");

    Ok(tmp.unwrap())
}

/// Render a template with a data file to String and validate against a schema
pub fn render_template_valid(template: &Path, data: &Path, schema: &Path) -> Result<String> {

    // check if template exists
    if !template.is_file() {
        bail!("Template file does not exist at '{}'", template.to_string_lossy().to_string());
    }

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

    console::debug("Rendering template...");
    console::debug(&format!("Template: {}", template.to_string_lossy().to_string()));
    console::debug(&format!("Data: {}", data.to_string_lossy().to_string()));

    let template_name = template.file_name().unwrap_or(OsStr::new("modelcard.md.jinja")).to_str().unwrap();
    let template_content = read_to_string(template).unwrap();

    //let env = create_env();
    let mut env = Environment::new();
    env.add_template(template_name, template_content.as_str())?;

    let template = env.get_template(template_name).unwrap();
    let data = crate::utils::load_json_file(data)?;
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