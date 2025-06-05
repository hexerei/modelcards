use std::path::Path;

use anyhow::{bail, Result};
use modelcards::utils::console;

pub fn render_modelcard(sources: Vec<String>, template_file: Option<String>) -> Result<bool> {
    let file_name = sources.last().ok_or_else(|| anyhow::anyhow!("No sources provided"))?;
    let file_name = Path::new(file_name);
    let target_file = Path::new(file_name.file_name().ok_or_else(|| anyhow::anyhow!("Invalid file path"))?).with_extension("md");
    console::info(format!("Rendering modelcard to {}.", target_file.to_string_lossy()).as_str());

    if let Ok(modelcard) = modelcards::merge::from_paths(sources) {
        let result: Result<String> = match template_file {
            None => modelcards::render::render_value_to_template(modelcard, None),
            Some(file) => {
                let template_file = Path::new(&file);
                modelcards::render::render_value_to_template(modelcard, Some(template_file))
            }
        };
        if let Ok(rendered) = result {
            modelcards::utils::create_file(target_file.as_path(), &rendered)?;
            return Ok(true);
        }
        bail!("Could not render template: {:?}", result.err());
    }
    bail!("Could not construct modelcard source data.");
}