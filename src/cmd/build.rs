use std::{ffi::OsStr, path::Path};
use modelcards::validate::check_against_schema;
use anyhow::{bail, Result};
use minijinja::{Environment, path_loader};

pub fn build_project(path: &Path, modelcard: Option<String>, out_dir: Option<String>, force: bool) -> Result<bool> {

    if !path.is_dir() {
        bail!("Project directory does not exist at '{}'", path.to_string_lossy().to_string());
    }

    let modelcard = modelcard.unwrap_or_else(|| "sample.json".to_string());
    let modelcard = Path::new(&modelcard);

    if !modelcard.is_file() {
        bail!("Modelcard file '{}' does not exist.", modelcard.to_string_lossy().to_string());
    }

    let out_dir = out_dir.unwrap_or_else(|| path.join("cards").to_string_lossy().to_string());
    let out_dir = Path::new(&out_dir);

    let filename = Path::new(modelcard.file_name().unwrap_or(OsStr::new("modelcard.json"))).with_extension("md");
    let target_file = out_dir.join(filename);

    if target_file.exists() && !force {
        bail!("Modelcard file '{}' already exists. Use --force to overwrite.", target_file.to_string_lossy().to_string());
    }

    // validate schema and data
    if let Err(e) = check_against_schema(path, modelcard) {
        bail!("Project could not be validated!\n{:?}", e);
    }

    println!("Building project...");

    println!("Project: {}", path.to_string_lossy().to_string());
    println!("Modelcard: {}", modelcard.to_string_lossy().to_string());
    println!("Template: {}", path.join("templates/modelcard.md.jinja").to_string_lossy().to_string());
    println!("Output: {}", target_file.to_string_lossy().to_string());

    let env = create_env();
    let template = env.get_template("modelcard.md.jinja").unwrap();
    let data = modelcards::utils::load_json_file(modelcard);

    println!("{}", template.render(&data).unwrap());

    println!("Done!");

    Ok(true)
}

fn create_env() -> Environment<'static> {
    let mut env = Environment::new();
    env.set_loader(path_loader("templates"));
    env
}