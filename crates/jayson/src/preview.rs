//! Preview rendering — pluggable trait for rendering merged JSON data.

use serde_json::Value;
use std::fmt;

/// Error type for preview rendering.
#[derive(Debug)]
pub struct PreviewError {
    pub message: String,
}

impl fmt::Display for PreviewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Preview error: {}", self.message)
    }
}

impl std::error::Error for PreviewError {}

/// Trait for rendering merged JSON data into a preview.
pub trait PreviewRenderer: Send + Sync {
    /// Render the merged data into a string (typically HTML).
    fn render(&self, data: &Value) -> Result<String, PreviewError>;

    /// Content type of the rendered output.
    fn content_type(&self) -> &str {
        "text/html; charset=utf-8"
    }
}

/// Default preview renderer — field names as headings, values as text.
pub struct PlainPreview;

impl PreviewRenderer for PlainPreview {
    fn render(&self, data: &Value) -> Result<String, PreviewError> {
        let mut html = String::from("<html><body>\n");
        render_value_html(&mut html, data, 1);
        html.push_str("</body></html>");
        Ok(html)
    }
}

fn render_value_html(out: &mut String, value: &Value, depth: usize) {
    match value {
        Value::Object(map) => {
            for (key, val) in map {
                let tag = format!("h{}", depth.min(6));
                out.push_str(&format!("<{}>{}</{}>\n", tag, escape_html(key), tag));
                render_value_html(out, val, depth + 1);
            }
        }
        Value::Array(arr) => {
            out.push_str("<ul>\n");
            for item in arr {
                out.push_str("<li>");
                render_value_html(out, item, depth);
                out.push_str("</li>\n");
            }
            out.push_str("</ul>\n");
        }
        Value::String(s) => {
            out.push_str(&format!("<p>{}</p>\n", escape_html(s)));
        }
        Value::Number(n) => {
            out.push_str(&format!("<p>{}</p>\n", n));
        }
        Value::Bool(b) => {
            out.push_str(&format!("<p>{}</p>\n", b));
        }
        Value::Null => {
            out.push_str("<p><em>null</em></p>\n");
        }
    }
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn plain_preview_renders_object() {
        let preview = PlainPreview;
        let data = json!({"title": "Test Model", "version": 1});
        let result = preview.render(&data).unwrap();
        assert!(result.contains("<h1>title</h1>"));
        assert!(result.contains("<p>Test Model</p>"));
        assert!(result.contains("<h1>version</h1>"));
    }

    #[test]
    fn plain_preview_escapes_html() {
        let preview = PlainPreview;
        let data = json!({"name": "<script>alert('xss')</script>"});
        let result = preview.render(&data).unwrap();
        assert!(result.contains("&lt;script&gt;"));
        assert!(!result.contains("<script>"));
    }

    #[test]
    fn plain_preview_renders_array() {
        let preview = PlainPreview;
        let data = json!({"tags": ["ml", "nlp"]});
        let result = preview.render(&data).unwrap();
        assert!(result.contains("<ul>"));
        assert!(result.contains("<li>"));
    }
}
