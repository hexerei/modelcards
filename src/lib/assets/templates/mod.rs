use std::include_str;

pub fn get_md() -> &'static str {
    include_str!("google.modelcard.md.jinja")
}

pub fn get_html() -> &'static str {
    include_str!("google.modelcard.html.jinja")
}
