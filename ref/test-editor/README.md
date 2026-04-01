# Test Environment: Visual Model Card Editor

Test data for `modelcards-edit`, based on the Census Income Classifier model card.

## Contents

```
ref/test-editor/
  schema/
    modelcard.schema.json   Google Model Card Toolkit schema (Draft-07)
  defaults.json             Org-level defaults (owner, license, references, ethical boilerplate)
  usecase.json              Use-case layer (intended users, use cases, limitations, dataset overview)
  model.json                Model-specific (name, description, version, metrics, citations, architecture)
  README.md                 This file
```

### Layer Design

The three JSON files demonstrate the layered composition model. Each upper layer
merges onto the one below it (RFC 7396), so values in `model.json` override
`usecase.json`, which overrides `defaults.json`.

| Layer | Purpose | Example fields |
|-------|---------|----------------|
| `defaults.json` | Company/org standards | schema_version, default owner, license, ethical boilerplate |
| `usecase.json` | Shared across models in a use case | intended users, use cases, limitations, dataset overview |
| `model.json` | Specific to one model | model name, description, version, metrics, citations |

## Prerequisites

Build the workspace from the repo root:

```bash
cargo build --workspace
```

## Testing modelcards-edit

### Quick Start

From the repository root, run:

```bash
cargo run -p modelcards-edit -- \
  --schema ref/test-editor/schema/modelcard.schema.json \
  --port 3000 \
  ref/test-editor/defaults.json \
  ref/test-editor/usecase.json \
  ref/test-editor/model.json
```

This will:
1. Load the three layers in order (defaults, usecase, model)
2. Use the Google Model Card Toolkit schema for form generation and validation
3. Start the editor on http://127.0.0.1:3000
4. Open your browser automatically

### What to Test

#### Layer Tabs
- Click each tab (defaults, usecase, model) to switch between layers
- Verify that each layer shows only its own data, not the merged result
- The "readonly" badge should NOT appear (all layers are editable)

#### Schema-Driven Forms
- All fields should render as appropriate input types:
  - `name`, `overview`, `documentation` → text inputs / textareas
  - `version.date` → date picker
  - `references[].reference` → URL input
  - `owners[]` → array of objects with add/remove/reorder controls
  - `performance_metrics[]` → array of objects (type, value, slice)
- Nested objects (`model_details`, `model_parameters`, `considerations`) should
  render as collapsible sections

#### Live Preview
- The right panel shows a rendered preview (using the modelcards Jinja template)
- Toggle between "Rendered" (HTML) and "Merged JSON" views
- After saving edits, the preview should update

#### Validation
- The status badge in the header should show "Valid" for the default data
- Try removing a required field or entering invalid data — the badge should
  turn red with an error count

#### Editing and Saving
- Edit a field in the `model` tab (e.g., change the model name)
- Click "Save" or press Ctrl+S / Cmd+S
- Verify the change persisted: check `ref/test-editor/model.json` on disk
- The preview should update after saving

#### Array Editing
- In the `model` tab, find `performance_metrics` (under `quantitative_analysis`)
- Click "+ Add" to add a new metric
- Use the arrow buttons to reorder metrics
- Click "x" to remove a metric
- Save and verify the JSON file

### Verify with CLI

After editing in the browser, verify the layers still merge and validate correctly:

```bash
# Merge all layers
cargo run -- merge \
  ref/test-editor/defaults.json \
  ref/test-editor/usecase.json \
  ref/test-editor/model.json \
  -o /tmp/merged_census.json

# Validate merged result
cargo run -- validate \
  ref/test-editor/defaults.json \
  ref/test-editor/usecase.json \
  ref/test-editor/model.json

# Render to markdown
cargo run -- render \
  ref/test-editor/defaults.json \
  ref/test-editor/usecase.json \
  ref/test-editor/model.json
```

### REST API Testing

With the editor running, you can also test the API directly:

```bash
# List layers
curl -s http://127.0.0.1:3000/api/layers | python3 -m json.tool

# Get merged result
curl -s http://127.0.0.1:3000/api/merged | python3 -m json.tool

# Get schema
curl -s http://127.0.0.1:3000/api/schema | python3 -m json.tool

# Validate
curl -s http://127.0.0.1:3000/api/validate | python3 -m json.tool

# Get rendered preview
curl -s http://127.0.0.1:3000/api/preview

# Update a layer (layer 2 = model.json)
curl -s -X PUT http://127.0.0.1:3000/api/layers/2 \
  -H "Content-Type: application/json" \
  -d '{"model_details":{"name":"Updated Model Name"}}' | python3 -m json.tool
```

### Alternative: Auto-Detect Project

If you run from inside the test directory with a `config.toml`, the editor can
auto-detect the project layout. For now, explicit paths work best.

## Known Limitations (v0.1.0)

- Preview reflects the last *saved* state, not in-progress edits
- `oneOf` schema constructs (e.g., license: identifier vs custom_text) render
  both fields — pick one
- No theming beyond the title — styling is the default jayson editor style
- No drag-and-drop for array reordering — use the arrow buttons
