use std::path::Path;

use anyhow::{bail, Result};

pub fn render_modelcard(sources: Vec<String>, template_file: Option<String>) -> Result<bool> {
    let file_name = sources.last().unwrap();
    let file_name = Path::new(file_name);
    let target_file = Path::new(file_name.file_name().unwrap()).with_extension("md");

    if let Ok(modelcard) = modelcards::merge::from_paths(sources) {
        let result: Result<String>;
        if template_file.is_none() {
            result = modelcards::render::render_value_to_template(modelcard, None);
        } else {
            let template_file = template_file.unwrap();
            let template_file = Path::new(&template_file);
            result = modelcards::render::render_value_to_template(modelcard, Some(template_file));
        }
        if let Ok(rendered) = result {
            modelcards::utils::create_file(target_file.as_path(), &rendered)?;
            return Ok(true);
        }
        bail!("Could not render template: {:?}", result.err());
    }
    bail!("Could not construct modelcard source data.");
}