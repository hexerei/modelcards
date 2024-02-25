use std::include_str;

pub fn get_default() -> String {
    let template = include_str!("default.toml");
    String::from(template)
}
