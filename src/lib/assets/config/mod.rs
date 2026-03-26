use std::include_str;

pub fn get_default() -> &'static str {
    include_str!("default.toml")
}

pub fn get_project_config() -> &'static str {
    include_str!("project.toml")
}
