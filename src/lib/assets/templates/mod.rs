use std::include_str;

pub fn get_md() -> String {
    let template = include_str!("google.modelcard.md.jinja");
    String::from(template)
}

pub fn get_html() -> String {
    let template = include_str!("google.modelcard.html.jinja");
    String::from(template)
}
