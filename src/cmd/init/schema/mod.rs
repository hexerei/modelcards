use std::include_str;

pub fn get_schema() -> String {
    let template = include_str!("google.modelcard.schema.json");
    String::from(template)
}

pub fn get_sample() -> String {
    let template = include_str!("google.sample.json");
    String::from(template)
}