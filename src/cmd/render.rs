use std::path::Path;
use modelcards::utils::load_json_file;
use anyhow::{bail, Result};

pub fn render_modelcard(sources: Vec<String>, template_file: Option<String>) -> Result<bool> {
    if let Ok(modelcard) = modelcards::merge::from_paths(sources) {
        let template = if template_file.is_some() {
            load_json_file(Path::new(&template_file.unwrap()))?
        } else {
            serde_json::from_str(&modelcards::assets::templates::get_md())?
        };

        println!("Modelcard: {:?}", modelcard);
        println!("Template: {:?}", template);
        return Ok(true);
    }
    bail!("Could not construct modelcard source data.");


}