# Feature: Unified Hierarchical Configuration

**Status**: Planned
**Date**: 2026-03-26
**Relates to**: README task "Hierarchical settings (default, config.toml, env, cli args)"

## Problem

Configuration resolution is split across two systems. The `config` crate handles layers 1-3 (defaults, config.toml, env vars), but CLI arguments are applied ad-hoc per command in `main.rs` with manual `unwrap_or` fallbacks. This means:

- Not all settings are overridable at every layer (e.g., `--schema` and `--template` can't be set via env vars or config.toml)
- Each new flag requires manual wiring in `main.rs`
- The precedence chain is implicit and inconsistent across commands

## Goal

A single resolution path for all settings:

```
embedded defaults → config.toml → MC_* env vars → CLI args
```

Every setting follows this chain. The latter always wins. Commands read from the resolved `Settings` struct — no fallback logic in `main.rs`.

## Approach

Use `config::Config::builder().set_override()` to inject CLI args as the final layer, making the `config` crate the single source of truth.

## Scope

### Settings in the hierarchy

All project-level settings that a user would reasonably configure once and override occasionally:

| Setting | default.toml | config.toml | Env var | CLI flag |
|---------|-------------|-------------|---------|----------|
| `project_dir` | `"."` | yes | `MC_PROJECT_DIR` | `-r/--root` |
| `input.data` | `"./sample.json"` | yes | `MC_INPUT__DATA` | `-s/--source` (build, check) |
| `input.schema` | `"./schema/modelcard.schema.json"` | yes | `MC_INPUT__SCHEMA` | `-s/--schema` (validate) |
| `input.validate` | `true` | yes | `MC_INPUT__VALIDATE` | new: `--no-validate-input` |
| `output.target` | `"./cards/modelcard.md"` | yes | `MC_OUTPUT__TARGET` | `-o/--target` (build, merge) |
| `output.template` | `"./templates/modelcard.md.jinja"` | yes | `MC_OUTPUT__TEMPLATE` | `-t/--template` (render) |
| `output.validate` | `true` | yes | `MC_OUTPUT__VALIDATE` | new: `--no-validate-output` |

Note: The `config` crate uses `__` (double underscore) as separator for nested keys with `Environment::with_prefix("mc").separator("__")`.

### CLI-only (not in hierarchy)

These are per-invocation intent, not project configuration:

- `--force` / `-f` — destructive action confirmation
- `--verbose` / `--quiet` — log level (handled by `clap_verbosity_flag`)
- `--config` / `-c` — meta: which config file to load (must resolve before the hierarchy runs)
- `completion <shell>` — unrelated to project config
- Positional `sources` args for validate/render/merge (these are inputs, not settings)

## Implementation Plan

### 1. Update `Settings` struct (`src/settings.rs`)

No structural changes needed — `Input` and `Output` already hold the right fields. Changes:

- Add `separator("__")` to the `Environment` source so nested keys work: `MC_INPUT__DATA`, `MC_OUTPUT__TEMPLATE`, etc.
- Add a new method `Settings::with_overrides(config_name: &str, overrides: Vec<(&str, String)>) -> Result<Self, ConfigError>` that takes key-value pairs and applies them via `set_override` after all other sources.
- Remove the `RUN_MODE` / environment-specific config file layer — it's unused and adds confusion. (Or keep it if there's a future use case, but document it.)

### 2. Refactor CLI arg resolution (`src/main.rs`)

After parsing `Cli`, collect all provided CLI args into override pairs:

```
// Pseudocode
let mut overrides = vec![];
match &cli.command {
    Command::Build { source, target, .. } => {
        if let Some(s) = source { overrides.push(("input.data", s)); }
        if let Some(t) = target { overrides.push(("output.target", t)); }
    }
    Command::Check { source } => {
        if let Some(s) = source { overrides.push(("input.data", s)); }
    }
    Command::Validate { schema, .. } => {
        if let Some(s) = schema { overrides.push(("input.schema", s)); }
    }
    Command::Render { template, .. } => {
        if let Some(t) = template { overrides.push(("output.template", t)); }
    }
    Command::Merge { target, .. } => {
        if let Some(t) = target { overrides.push(("output.target", t)); }
    }
    _ => {}
}
if cli.root != PathBuf::from(".") {
    overrides.push(("project_dir", cli.root.display().to_string()));
}

let settings = Settings::with_overrides(&config_name, overrides)?;
```

### 3. Simplify command dispatch (`src/main.rs`)

Commands no longer do their own fallback resolution. They receive the fully resolved `Settings`:

```
// Before
Command::Build { source, target, force } => {
    let source = source.unwrap_or(settings.input.data);
    let target = target.unwrap_or(settings.output.target);
    cmd::build_project(&cli_dir, Some(source), Some(target), force.unwrap_or(settings.force));
}

// After
Command::Build { force, .. } => {
    cmd::build_project(&cli_dir, &settings.input.data, &settings.output.target, force);
}
```

### 4. Update default.toml

No changes needed — it already covers all fields. Just verify the double-underscore separator works with the existing env var setup.

### 5. Tests

- **Unit test**: `Settings::with_overrides` correctly applies overrides over defaults
- **Unit test**: Env vars with `__` separator resolve nested keys (`MC_INPUT__DATA`)
- **Integration test**: Full precedence chain — set a value at each layer, verify CLI wins
- **Integration test**: Verify existing CLI behavior unchanged (no regressions)

### 6. Documentation

- Update README.md: check off the hierarchical settings task, add a configuration section documenting the precedence chain and available env vars
- Update `--help` text if any flags change

## Design Decisions

1. **`force` stays CLI-only** — putting `force = true` in config.toml is a footgun. It's per-invocation intent.
2. **`sources` (positional args) stay CLI-only** — these are pipeline inputs, not project-level config. You don't configure "which files to validate" in config.toml.
3. **Double underscore separator for env vars** — `MC_INPUT__DATA` not `MC_INPUT_DATA`, because single underscore is ambiguous (is `MC_INPUT_DATA` the key `input_data` or nested `input.data`?).
4. **`RUN_MODE` layer** — remove unless there's a known use case. It adds a layer nobody uses and makes the hierarchy harder to reason about.
5. **No new CLI flags for validate/output booleans yet** — `--no-validate-input` and `--no-validate-output` are optional scope. They can be added later since the env var path (`MC_INPUT__VALIDATE=false`) covers the need.

## Out of Scope

- Config file format changes (staying with TOML)
- New settings beyond what currently exists
- Config file generation/migration tooling
- Watch mode or config reload
