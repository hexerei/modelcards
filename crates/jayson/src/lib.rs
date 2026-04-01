//! # Jayson
//!
//! A generic, schema-driven JSON editor library with layered composition and live preview.
//!
//! Jayson provides:
//! - **Layers**: Named JSON documents that merge bottom-up (RFC 7396)
//! - **Schema inference**: Auto-generate JSON Schema Draft-07 from data
//! - **Preview rendering**: Pluggable trait for rendering merged data
//! - **Web editor**: Embedded SPA served via axum

pub mod layer;
pub mod preview;
pub mod schema;
pub mod serve;
mod ui;

use std::path::Path;

use anyhow::{bail, Result};
use layer::{Layer, LayerStack};
use preview::{PlainPreview, PreviewError, PreviewRenderer};
use serde_json::Value;

/// Callback type for on-save events.
type SaveCallback = Box<dyn Fn(&Value) + Send + Sync>;

/// The main editor struct — holds layers, schema, config, and preview renderer.
pub struct Jayson {
    pub(crate) stack: LayerStack,
    pub(crate) schema: Value,
    pub(crate) preview: Box<dyn PreviewRenderer>,
    pub(crate) port: u16,
    pub(crate) host: String,
    pub(crate) title: String,
    pub(crate) on_save: Option<SaveCallback>,
}

impl Jayson {
    /// Create a new builder.
    #[must_use]
    pub fn builder() -> JaysonBuilder {
        JaysonBuilder::default()
    }

    /// Get the merged value from all layers.
    pub fn merged_value(&self) -> Value {
        self.stack.merged()
    }

    /// Validate the merged value against the schema.
    pub fn validate(&self) -> Result<Vec<String>> {
        let merged = self.stack.merged();

        let validator = jsonschema::validator_for(&self.schema)
            .map_err(|e| anyhow::anyhow!("Invalid schema: {}", e))?;

        let errors: Vec<String> = validator
            .iter_errors(&merged)
            .map(|e| {
                let path = e.instance_path();
                if path.as_str().is_empty() {
                    format!("{}", e)
                } else {
                    format!("{}: {}", path, e)
                }
            })
            .collect();

        Ok(errors)
    }

    /// Render a preview of the merged data.
    pub fn render_preview(&self) -> Result<String, PreviewError> {
        let merged = self.stack.merged();
        self.preview.render(&merged)
    }

    /// Start the web server and open the browser. Blocks until shutdown.
    pub async fn serve(self) -> Result<()> {
        serve::run(self).await
    }
}

/// Builder for configuring a Jayson editor instance.
pub struct JaysonBuilder {
    layers: Vec<Layer>,
    schema: Option<Value>,
    preview: Option<Box<dyn PreviewRenderer>>,
    port: u16,
    host: String,
    title: String,
    on_save: Option<SaveCallback>,
}

impl Default for JaysonBuilder {
    fn default() -> Self {
        Self {
            layers: Vec::new(),
            schema: None,
            preview: None,
            port: 0,
            host: "127.0.0.1".to_string(),
            title: "Jayson Editor".to_string(),
            on_save: None,
        }
    }
}

impl JaysonBuilder {
    /// Set the JSON Schema directly.
    pub fn schema(mut self, schema: Value) -> Self {
        self.schema = Some(schema);
        self
    }

    /// Load schema from a file.
    pub fn schema_file(mut self, path: &Path) -> Result<Self> {
        self.schema = Some(schema::load_json(path)?);
        Ok(self)
    }

    /// Add a layer (bottom-up insertion order).
    pub fn layer(mut self, layer: Layer) -> Self {
        self.layers.push(layer);
        self
    }

    /// Set a custom preview renderer.
    pub fn preview(mut self, renderer: impl PreviewRenderer + 'static) -> Self {
        self.preview = Some(Box::new(renderer));
        self
    }

    /// Set the port (default 0 = random available port).
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Set the host (default "127.0.0.1").
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }

    /// Set the browser tab title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set a callback invoked when the user saves.
    pub fn on_save(mut self, callback: impl Fn(&Value) + Send + Sync + 'static) -> Self {
        self.on_save = Some(Box::new(callback));
        self
    }

    /// Build the Jayson editor.
    pub fn build(self) -> Result<Jayson> {
        if self.layers.is_empty() {
            bail!("At least one layer is required");
        }

        let mut stack = LayerStack::new();
        for layer in self.layers {
            stack.push(layer);
        }

        let schema = self.schema.unwrap_or_else(|| {
            schema::infer_schema(&stack.merged())
        });

        Ok(Jayson {
            stack,
            schema,
            preview: self.preview.unwrap_or_else(|| Box::new(PlainPreview)),
            port: self.port,
            host: self.host,
            title: self.title,
            on_save: self.on_save,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn mem_layer(name: &str, data: Value) -> Layer {
        Layer {
            name: name.to_string(),
            data,
            source: layer::LayerSource::Memory,
            readonly: false,
        }
    }

    #[test]
    fn build_requires_layers() {
        let result = Jayson::builder().build();
        assert!(result.is_err());
    }

    #[test]
    fn build_with_layer_succeeds() {
        let editor = Jayson::builder()
            .layer(mem_layer("test", json!({"a": 1})))
            .build()
            .unwrap();
        assert_eq!(editor.merged_value(), json!({"a": 1}));
    }

    #[test]
    fn build_infers_schema_when_not_provided() {
        let editor = Jayson::builder()
            .layer(mem_layer("test", json!({"name": "model", "count": 5})))
            .build()
            .unwrap();
        let schema = editor.schema;
        assert_eq!(schema["type"], "object");
        assert_eq!(schema["properties"]["name"]["type"], "string");
    }

    #[test]
    fn build_with_explicit_schema() {
        let schema = json!({"type": "object"});
        let editor = Jayson::builder()
            .schema(schema.clone())
            .layer(mem_layer("test", json!({"a": 1})))
            .build()
            .unwrap();
        assert_eq!(editor.schema, schema);
    }

    #[test]
    fn validate_with_valid_data() {
        let editor = Jayson::builder()
            .schema(json!({
                "type": "object",
                "properties": {"name": {"type": "string"}},
                "required": ["name"]
            }))
            .layer(mem_layer("test", json!({"name": "model"})))
            .build()
            .unwrap();
        let errors = editor.validate().unwrap();
        assert!(errors.is_empty());
    }

    #[test]
    fn validate_with_invalid_data() {
        let editor = Jayson::builder()
            .schema(json!({
                "type": "object",
                "properties": {"name": {"type": "string"}},
                "required": ["name"]
            }))
            .layer(mem_layer("test", json!({"name": 42})))
            .build()
            .unwrap();
        let errors = editor.validate().unwrap();
        assert!(!errors.is_empty());
    }

    #[test]
    fn preview_renders() {
        let editor = Jayson::builder()
            .layer(mem_layer("test", json!({"title": "Test"})))
            .build()
            .unwrap();
        let html = editor.render_preview().unwrap();
        assert!(html.contains("Test"));
    }
}
