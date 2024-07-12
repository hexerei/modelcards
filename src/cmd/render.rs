pub fn render_modelcard(sources: Vec<String>, template_file: Option<String>) -> bool {
    if let Ok(modelcard) = modelcards::merge::from_paths(sources) {
        let template = if template_file.is_some() {
            template_file.unwrap()
        } else {
            "modelcard.schema.json".to_string()
        };
        println!("Modelcard: {:?}", modelcard);
        println!("Template: {:?}", template);
        return true;
    }
    false
}