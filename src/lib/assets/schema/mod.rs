use std::include_str;

pub fn get_schema() -> &'static str {
    include_str!("google.modelcard.schema.json")
}

pub fn get_sample() -> &'static str {
    include_str!("google.sample.json")
}