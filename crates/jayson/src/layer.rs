//! Layer and LayerStack — named JSON documents with bottom-up merge.

use std::path::PathBuf;
use serde_json::Value;

/// A named JSON document in the layer stack.
pub struct Layer {
    pub name: String,
    pub data: Value,
    pub source: LayerSource,
    pub readonly: bool,
}

/// Where a layer's data lives.
pub enum LayerSource {
    /// Persisted to disk on save.
    File(PathBuf),
    /// Ephemeral (computed defaults, etc.).
    Memory,
}

/// An ordered stack of layers that merge bottom-up.
pub struct LayerStack {
    layers: Vec<Layer>,
}

impl LayerStack {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn push(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    pub fn len(&self) -> usize {
        self.layers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.layers.is_empty()
    }

    pub fn get(&self, id: usize) -> Option<&Layer> {
        self.layers.get(id)
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut Layer> {
        self.layers.get_mut(id)
    }

    /// Merge all layers bottom-up using RFC 7396 JSON Merge Patch.
    pub fn merged(&self) -> Value {
        let mut result = Value::Object(serde_json::Map::new());
        for layer in &self.layers {
            json_patch::merge(&mut result, &layer.data);
        }
        result
    }

    /// Compute RFC 6902 diff between two layers.
    pub fn diff(&self, a: usize, b: usize) -> Option<json_patch::Patch> {
        let layer_a = self.layers.get(a)?;
        let layer_b = self.layers.get(b)?;
        Some(json_patch::diff(&layer_a.data, &layer_b.data))
    }
}

impl Default for LayerStack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn file_layer(name: &str, data: Value) -> Layer {
        Layer {
            name: name.to_string(),
            data,
            source: LayerSource::Memory,
            readonly: false,
        }
    }

    #[test]
    fn merge_empty_stack() {
        let stack = LayerStack::new();
        assert_eq!(stack.merged(), json!({}));
    }

    #[test]
    fn merge_single_layer() {
        let mut stack = LayerStack::new();
        stack.push(file_layer("base", json!({"a": 1})));
        assert_eq!(stack.merged(), json!({"a": 1}));
    }

    #[test]
    fn merge_overwrites_bottom_up() {
        let mut stack = LayerStack::new();
        stack.push(file_layer("base", json!({"a": 1, "b": 2})));
        stack.push(file_layer("override", json!({"b": 3, "c": 4})));
        assert_eq!(stack.merged(), json!({"a": 1, "b": 3, "c": 4}));
    }

    #[test]
    fn merge_deep_objects() {
        let mut stack = LayerStack::new();
        stack.push(file_layer("base", json!({"d": {"e": 1, "f": 2}})));
        stack.push(file_layer("override", json!({"d": {"f": 3, "g": 4}})));
        assert_eq!(stack.merged(), json!({"d": {"e": 1, "f": 3, "g": 4}}));
    }

    #[test]
    fn merge_array_replacement() {
        let mut stack = LayerStack::new();
        stack.push(file_layer("base", json!({"arr": [1, 2, 3]})));
        stack.push(file_layer("override", json!({"arr": [4, 5]})));
        assert_eq!(stack.merged(), json!({"arr": [4, 5]}));
    }

    #[test]
    fn merge_null_deletes_key() {
        // RFC 7396: null means delete
        let mut stack = LayerStack::new();
        stack.push(file_layer("base", json!({"a": 1, "b": 2})));
        stack.push(file_layer("override", json!({"b": null})));
        assert_eq!(stack.merged(), json!({"a": 1}));
    }

    #[test]
    fn merge_three_layers() {
        let mut stack = LayerStack::new();
        stack.push(file_layer("defaults", json!({"a": 1, "b": 2, "c": 3})));
        stack.push(file_layer("usecase", json!({"b": 20})));
        stack.push(file_layer("model", json!({"c": 30, "d": 40})));
        assert_eq!(stack.merged(), json!({"a": 1, "b": 20, "c": 30, "d": 40}));
    }

    #[test]
    fn readonly_layer_accessible() {
        let mut stack = LayerStack::new();
        stack.push(Layer {
            name: "readonly".to_string(),
            data: json!({"x": 1}),
            source: LayerSource::Memory,
            readonly: true,
        });
        assert!(stack.get(0).unwrap().readonly);
        assert_eq!(stack.len(), 1);
    }

    #[test]
    fn diff_between_layers() {
        let mut stack = LayerStack::new();
        stack.push(file_layer("a", json!({"x": 1, "y": 2})));
        stack.push(file_layer("b", json!({"x": 1, "y": 3, "z": 4})));
        let patch = stack.diff(0, 1).unwrap();
        assert!(!patch.0.is_empty());
    }
}
