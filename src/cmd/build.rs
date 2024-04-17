use std::{fs::create_dir_all, path::{Path, PathBuf}};
use modelcards::{
    render::render_template,
    validate::check_against_schema
};
use anyhow::{bail, Result};

pub fn build_project(path: &Path, modelcard: Option<String>, target: Option<String>, force: bool) -> Result<bool> {

    // check if project directory exists
    if !path.is_dir() {
        bail!("Project directory does not exist at '{}'", path.to_string_lossy().to_string());
    }

    let modelcard = opt_get_path(modelcard, "sample.json", path)?;
    let file_name = Path::new(modelcard.file_name().unwrap()).with_extension("md");
    //let target_file = opt_get_path(target, modelcard.file_name().unwrap().to_str().unwrap(), path.join("cards").as_path())?;

    // check if output directory exists and create it if not
    let target = target.unwrap_or_else(|| path.join("cards").join(&file_name).to_string_lossy().to_string());
    let out_dir = Path::new(&target).parent().unwrap();
    if !out_dir.exists() {
        create_dir_all(out_dir)?;
    }

    // check if output file exists and if force is not set
    let target_file = out_dir.join(file_name);
    if target_file.exists() && !force {
        bail!("Modelcard file '{}' already exists. Use --force to overwrite.", target_file.to_string_lossy().to_string());
    }

    // check if data validates agains schema
    if let Err(e) = check_against_schema(path, modelcard.as_path()) {
        bail!("Project could not be validated!\n{:?}", e);
    }

    log::info!("Building project...");

    log::info!("Project: {}", path.to_string_lossy().to_string());
    log::info!("Modelcard: {}", modelcard.to_string_lossy().to_string());
    log::info!("Template: {}", path.join("templates/modelcard.md.jinja").to_string_lossy().to_string());
    log::info!("Output: {}", target_file.to_string_lossy().to_string());

    // render the template
    match render_template(&path.join("templates/modelcard.md.jinja"), modelcard.as_path()) {
        Ok(result) => modelcards::utils::create_file(&target_file, &result)?,
        Err(e) => bail!("Could not render template: {:?}", e),
    }

    log::info!("Done!");

    Ok(true)
}

/// Check if the file contains a valid path, or prepend project directory
fn get_path(file_name: &str, path: &Path) -> Result<PathBuf> {

    let mut file_path = Path::new(file_name).to_path_buf();
    // check if modelcard file exists
    if !file_path.is_file() {
        // if the file does not exist in the current directory, check the project directory
        file_path = path.join(file_name).to_path_buf();
        if !file_path.is_file() {
            bail!("File '{}' does not exist in expected path.", file_name);
        }
    }

    Ok(file_path)
}

fn opt_get_path(file_name: Option<String>, default: &str, path: &Path) -> Result<PathBuf> {
    let file_name = file_name.unwrap_or_else(|| default.to_string());
    get_path(&file_name, path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        env::temp_dir,
        fs::{create_dir, remove_dir_all},
        path::PathBuf
    };
    use modelcards::utils::create_file;
    #[allow(unused_imports)]
    use modelcards::assets::{templates, schema};
    use crate::cmd::create_new_project;

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
    fn build_project_with_defaults() {
        let path = get_temp_dir("test_build_project_with_defaults", true);
        create_new_project(path.to_str().unwrap(), false).expect("Could not populate test directory");
        build_project(&path, None, None, false).expect("Could not build project");
        assert!(path.join("cards/sample.md").exists());
    }

    #[test]
    fn build_project_with_custom_data() {
        let path = get_temp_dir("test_build_project_with_custom_data", true);
        create_new_project(path.to_str().unwrap(), false).expect("Could not populate test directory");
        create_file(path.join("modelcard.json").as_path(), &schema::get_sample()).expect("Could not create modelcard data file");
        build_project(&path, Some("modelcard.json".to_string()), None, false).expect("Could not build project");
        assert!(path.join("cards/modelcard.md").exists());
    }
}
