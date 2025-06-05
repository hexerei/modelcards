use std::include_str;

pub fn get_default() -> &'static str {
    include_str!("default.toml")
}
