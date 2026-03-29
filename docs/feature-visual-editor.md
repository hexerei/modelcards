# Feature: Visual Model Card Editor

**Status**: Planned
**Date**: 2026-03-26
**Updated**: 2026-03-28

## Problem

Model cards are authored as raw JSON files, which works for ML engineers but is hostile to business users and data scientists during the experimentation phase. The multi-layer composition model (defaults.json + usecase.json + model.json) is powerful but opaque — users can't see how layers merge or which layer contributes which value without running CLI commands.

## Goal

A browser-based visual editor launched via `modelcards-edit` that:

1. Generates form UIs from arbitrary JSON Schema (Draft-07)
2. Lets users edit individual layers separately (tabs/panels per file)
3. Shows a live merged preview using the existing merge algorithm
4. Validates in real-time against the project's schema
5. Works for both audiences: data scientists during experimentation and ML engineers during dev/prod

## Architecture: Three-Crate Workspace

### Why not a feature flag?

A feature flag (`--features editor`) would pull HTTP server and embedded frontend dependencies into the core crate. This means `#[cfg(feature)]` scattered through the code, a single release artifact that's either bloated or confusing, and CI/CD users potentially installing the wrong thing.

### Why three crates instead of two?

During design, we identified that the schema-driven JSON editor is a general-purpose tool worth extracting into its own crate. The editor layer logic (merge, validate, schema-driven forms) has no modelcard-specific knowledge — it works on any JSON + JSON Schema combination. Extracting it means the editor library is independently useful and publishable.

### Workspace layout: src/ + crates/

The modelcards crate stays in `src/` as the workspace root package. New crates live under `crates/`. All three crates point `repository` to the same GitHub repo in their `Cargo.toml` — crates.io handles monorepos fine (tokio, serde, axum all do this).

```
modelcards/
  Cargo.toml                    # workspace root + modelcards package
  src/                          # existing modelcards crate -- UNTOUCHED (until Phase 1)
  crates/
    jayson/                     # generic schema-driven JSON editor (library)
      Cargo.toml
      src/
        lib.rs                  # public API: JaysonBuilder, Jayson
        layer.rs                # Layer, LayerStack, auto-merge via json-patch
        preview.rs              # PreviewRenderer trait + PlainPreview default
        schema.rs               # schema loading + inference from JSON
        serve.rs                # axum HTTP server + SPA embedding
        ui/                     # embedded frontend assets
          mod.rs
      examples/
        basic.rs                # minimal example: open a JSON file in browser
    modelcards-edit/            # thin glue: modelcards domain + jayson editor
      Cargo.toml
      src/
        main.rs                 # binary entry point
```

**Why "jayson"**: No "schema" in the name (we edit data, not schemas). Memorable, fun, and the JSON pun is obvious.

### Dependency graph

```
modelcards          (existing deps, swap valico→jsonschema, add json-patch, drop custom merge)
jayson              (json-patch, jsonschema, axum, tokio, serde, serde_json, anyhow, log)
modelcards-edit     (depends on: jayson + modelcards)
```

jayson does NOT depend on modelcards. modelcards does NOT depend on jayson.

Both jayson and modelcards depend on `jsonschema` and `json-patch` — shared via `[workspace.dependencies]`.

## Key Design Decisions

### Build on ecosystem crates, don't reimplement

Two battle-tested crates replace our custom code:

| Current | Replace with | Downloads | Why |
|---------|-------------|-----------|-----|
| `valico` (dead since 2023, Draft-7 only) | **`jsonschema`** (58M+, active, Draft 4–2020-12, 75–645x faster) | 58M | Actively maintained, full draft coverage |
| Custom recursive `merge()` | **`json-patch`** (RFC 7396 merge + RFC 6902 diff) | 61M | Battle-tested, adds diff capability for free |

jayson provides the **layer abstraction** and **editor UI** on top of these — it does not reimplement merge or validation.

### jayson is generic, modelcards-edit adds theming

jayson: layers, auto-merge, schema-driven editor, plain default styling.
modelcards-edit: modelcard-specific theming (styles, logo, titles, presets), preview via modelcards render, project detection.

### Design constraints

- **Zero runtime dependencies**: Everything embedded. No npm, no CDN, no external assets.
- **Schema-agnostic**: The form is generated from whatever JSON Schema the project uses. Works with Google Model Card Toolkit schema, HuggingFace, or custom schemas.
- **Non-destructive**: Edits save to the individual layer files. The merged result is never written to disk (that's what `modelcards build` is for).
- **Offline-first**: No network access needed after installation.

## Cargo.toml Changes

Root `Cargo.toml` — add workspace section, keep existing `[package]`:

```toml
[workspace]
members = [".", "crates/jayson", "crates/modelcards-edit"]
resolver = "2"

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
json-patch = "4"
jsonschema = "0.45"
anyhow = "1"
log = "0.4"
```

Each sub-crate uses `dep.workspace = true` for shared dependencies.

## jayson Public API Design

### Core types

```rust
// Layer: a named JSON document in the stack
pub struct Layer {
    pub name: String,
    pub data: Value,
    pub source: LayerSource,
    pub readonly: bool,
}

pub enum LayerSource {
    File(PathBuf),   // persisted to disk on save
    Memory,          // ephemeral (computed defaults, etc.)
}
```

### Layer merging

Uses `json_patch::merge()` (RFC 7396) internally — no custom merge algorithm. Layers merge bottom-up: each upper layer is merge-patched onto the accumulated base.

```rust
pub struct LayerStack { /* ... */ }

impl LayerStack {
    pub fn merged(&self) -> Value;           // RFC 7396 merge of all layers
    pub fn diff(&self, a: usize, b: usize) -> json_patch::Patch;  // RFC 6902 diff between layers
}
```

### Validation

Uses `jsonschema` for all validation. jayson accepts a JSON Schema and validates the merged result.

### Traits

```rust
// Preview: how merged data renders in the UI panel.
pub trait PreviewRenderer: Send + Sync {
    fn render(&self, data: &Value) -> Result<String, PreviewError>;
    fn content_type(&self) -> &str { "text/html; charset=utf-8" }
}

pub struct PlainPreview;  // default: field names as headings, values as text, no JSON syntax
```

No MergeStrategy trait — we use json-patch's RFC 7396 implementation directly.

### Builder

```rust
Jayson::builder()
    .schema(value)              // optional -- inferred from first layer if omitted
    .schema_file(path)          // alternative: load from file
    .layer(layer)               // add layers bottom-up in insertion order
    .preview(renderer)          // optional -- defaults to PlainPreview
    .port(3000)                 // optional -- default 0 = random available port
    .host("127.0.0.1")         // optional
    .title("Jayson Editor")    // optional -- shown in browser tab
    .on_save(callback)          // optional -- called when user hits save
    .build()?;

editor.serve().await?;          // blocks, opens browser
editor.merged_value()?;         // non-serving: get merged result (for testing/scripting)
```

### REST API (served by jayson)

```
GET  /api/schema          -- return the JSON Schema
GET  /api/layers          -- list layer names + metadata
GET  /api/layers/:id      -- get layer data
PUT  /api/layers/:id      -- update layer data (triggers re-merge/re-validate)
GET  /api/merged          -- return merged result
GET  /api/preview         -- return rendered preview HTML
GET  /api/validate        -- validate merged result against schema
GET  /                    -- serve embedded SPA
```

### Schema inference

When no schema is provided, jayson walks the loaded JSON and generates a Draft-07 schema:
- Objects -> `{"type": "object", "properties": {...}}`
- Strings -> `{"type": "string"}`
- Numbers -> `{"type": "number"}`
- Booleans -> `{"type": "boolean"}`
- Arrays -> `{"type": "array", "items": {...}}` (inferred from first element)
- Null -> `{}` (any type)

User can save the generated schema from the UI for future use.

~200-400 lines. Built in-house — the Rust ecosystem has no maintained crate for this (genson-rs is stale).

## Frontend (Embedded SPA)

- Vanilla JS or Preact — no webpack, no node_modules at runtime, no build toolchain required for users
- Schema-driven form generator that walks JSON Schema and renders:
  - Text inputs for strings
  - Number inputs for numbers
  - Checkboxes for booleans
  - Dropdowns for enums
  - Date pickers for `format: "date"` fields
  - URL inputs for `format: "uri"` fields
  - Textarea for long text (descriptions, citations)
  - Nested collapsible sections for objects
  - Add/remove/reorder controls for arrays of objects
  - `oneOf` support (e.g., license: identifier vs custom_text)
- Layout: tab bar for layers, form editor in main area, side panel with merged preview
- Merged preview updates live as user edits any layer
- Validation errors shown inline on the relevant fields
- Bundled into the binary at compile time — single binary distribution preserved

## modelcards-edit Glue

~40 lines of actual logic:

```rust
// Implements PreviewRenderer using modelcards::render::render_value_to_template
struct ModelCardPreview;

impl jayson::PreviewRenderer for ModelCardPreview {
    fn render(&self, data: &Value) -> Result<String, jayson::PreviewError> {
        modelcards::render::render_value_to_template(data.clone(), None)
            .map_err(|e| jayson::PreviewError { message: e.to_string() })
    }
}

// main(): detect project, load schema from modelcards::assets, configure layers, serve
// Adds: modelcard-specific theming (styles, logo, titles, presets)
```

## Implementation Phases

### Phase 1: Swap valico -> jsonschema, merge.rs -> json-patch (standalone PR)

Before any restructuring. Smallest possible change, immediate value.

- Replace `valico` with `jsonschema` in Cargo.toml
- Update `src/lib/validate.rs` to use `jsonschema` API
- Replace custom `merge()` in `src/lib/merge.rs` with `json_patch::merge()`
- Add `json-patch` to Cargo.toml
- Run full test suite — behavior should be identical
- Remove `valico` from dependencies

### Phase 2: Workspace scaffolding

- Add `[workspace]` section to root Cargo.toml with `[workspace.dependencies]`
- Create `crates/jayson/` with stub lib
- Create `crates/modelcards-edit/` with stub main
- Verify `cargo build && cargo test` passes for existing modelcards

### Phase 3: jayson core (no web, no async)

- `Layer`, `LayerStack` using `json_patch::merge()` internally
- `PreviewRenderer` trait + `PlainPreview`
- `JaysonBuilder` + `Jayson` struct (without `.serve()`)
- Schema loading + inference
- `merged_value()` for testing
- Unit tests

### Phase 4: jayson web server

- Add axum + tokio (minimal features)
- REST API endpoints
- Embed minimal SPA (single HTML file, vanilla JS)
- Browser auto-open
- Example binary in `examples/basic.rs`

### Phase 5: SPA editor UI

- Schema-driven form generation
- Layer tab switcher
- Live preview pane
- Validation error display
- Save functionality

### Phase 6: modelcards-edit integration

- `ModelCardPreview` impl
- Theming: styles, logo, titles, presets
- Project detection (config.toml, schema/, data/ dirs)
- CLI with clap (port, project root, etc.)
- End-to-end test

### Phase 7: Publish

- CI/CD: publish jayson first, then modelcards-edit
- All crates point `repository` to same GitHub repo
- Release asset builds for modelcards-edit binary

## Release Pipeline

### GitHub Actions changes

**upload-assets job**: Add a step to build and upload the `modelcards-edit` binary alongside `modelcards`.

**publish-crate job**: Publish in dependency order with index propagation delay:
```yaml
publish-crates:
  needs: create-release
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
    - run: cargo publish -p modelcards
      env:
        CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
    - run: sleep 30
    - run: cargo publish -p jayson
      env:
        CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
    - run: sleep 30
    - run: cargo publish -p modelcards-edit
      env:
        CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
```

### Installation

```bash
# CI/CD — core CLI only (unchanged)
cargo install modelcards

# Generic JSON editor (library dependency)
# cargo add jayson

# Modelcard-specific editor
cargo install modelcards-edit
```

All binaries also available as GitHub release assets for direct download.

## Verification

- `cargo build` at workspace root compiles all three crates
- `cargo test` passes all existing modelcards tests (zero regression)
- `cargo test -p jayson` passes layer merge, schema inference, preview tests
- `cargo run -p modelcards-edit` opens browser with model card editor
- Manual: edit a layer in the UI, verify merged preview updates, save persists to disk

## Open Questions

1. **Frontend framework**: Vanilla JS keeps it simple but schema-driven form generation is complex. Preact adds ~3KB and gives component model + JSX. Decision can wait until implementation.
2. **WebSocket vs polling**: WebSocket gives instant merge preview updates. Polling is simpler to implement. Start with polling, upgrade if latency is noticeable.
3. **Array editing UX**: Arrays of objects (owners, metrics, datasets) are the hardest UI challenge. Need drag-and-drop reordering and inline add/remove. Worth prototyping early.
4. **Diff view**: Should the merged preview highlight which layer each value came from? Useful but adds complexity. Consider as v2.

## Out of Scope (for v1)

- Collaborative editing / multi-user
- Cloud deployment / hosted version
- Git integration (diff, commit from editor)
- Template editing (only data layer editing)
- Schema editor (users bring their own schema)

## Critical Files

- `/Users/daniel/Code/hex/modelcards/Cargo.toml` — add workspace section, swap valico->jsonschema, add json-patch
- `/Users/daniel/Code/hex/modelcards/src/lib/merge.rs` — replace custom merge with json_patch::merge
- `/Users/daniel/Code/hex/modelcards/src/lib/validate.rs` — replace valico with jsonschema
- `/Users/daniel/Code/hex/modelcards/src/lib/render.rs:151` — `render_value_to_template` used by modelcards-edit's PreviewRenderer
- `/Users/daniel/Code/hex/modelcards/src/lib/assets/schema/mod.rs` — `get_schema()` supplies default schema to modelcards-edit
