# Feature: Unified Hierarchical Configuration

**Status**: Implemented (with minor gaps)
**Date**: 2026-03-26
**Implemented**: 2026-03-27 (commit `a19cd68`)
**Relates to**: README task "Hierarchical settings (default, config.toml, env, cli args)"

## Problem

Configuration resolution was split across two systems. The `config` crate handled layers 1-3 (defaults, config.toml, env vars), but CLI arguments were applied ad-hoc per command in `main.rs` with manual `unwrap_or` fallbacks. This meant:

- Not all settings were overridable at every layer (e.g., `--schema` and `--template` couldn't be set via env vars or config.toml)
- Each new flag required manual wiring in `main.rs`
- The precedence chain was implicit and inconsistent across commands

## Goal

A single resolution path for all settings:

```
embedded defaults → config.toml → MC_* env vars → CLI args
```

Every setting follows this chain. The latter always wins. Commands read from the resolved `Settings` struct — no fallback logic in `main.rs`.

## What Was Implemented

### `Settings::with_overrides()` (`src/settings.rs`)

- `separator("__")` added to `Environment` source for nested env vars
- `Settings::with_overrides()` method takes key-value pairs, applies them via `set_override` after all other sources
- Full builder chain: defaults → RUN_MODE config (kept, see below) → config.toml → env vars → CLI overrides

### CLI override collection (`src/main.rs`)

After parsing `Cli`, all provided CLI args are collected into override pairs and passed to `Settings::with_overrides()`. Commands that wire overrides: build, check, validate, render, merge.

### Command dispatch simplification (`src/main.rs`)

Commands read from the resolved `Settings` struct directly. No more ad-hoc `unwrap_or` fallback chains.

### Config assets (`src/lib/assets/config/`)

- `default.toml` — minimal hierarchy base with all default values
- `project.toml` — richer template used by `init` command

### Tests (`src/settings.rs`)

7 unit tests covering:
- Defaults load correctly
- Flat key overrides work
- Nested key overrides work (`input.data`, `output.template`)
- Optional field preservation
- Config file overrides defaults
- CLI overrides beat config file

## Settings in the Hierarchy

| Setting | default.toml | config.toml | Env var | CLI flag |
|---------|-------------|-------------|---------|----------|
| `project_dir` | `"."` | yes | `MC_PROJECT_DIR` | `-r/--root` |
| `input.data` | `"./sample.json"` | yes | `MC_INPUT__DATA` | `-s/--source` (build, check) |
| `input.schema` | `"./schema/modelcard.schema.json"` | yes | `MC_INPUT__SCHEMA` | `-s/--schema` (validate) |
| `input.validate` | `true` | yes | `MC_INPUT__VALIDATE` | — |
| `output.target` | `"./cards/modelcard.md"` | yes | `MC_OUTPUT__TARGET` | `-o/--target` (build, merge) |
| `output.template` | `"./templates/modelcard.md.jinja"` | yes | `MC_OUTPUT__TEMPLATE` | `-t/--template` (render) |
| `output.validate` | `true` | yes | `MC_OUTPUT__VALIDATE` | — |

Note: The `config` crate uses `__` (double underscore) as separator for nested keys with `Environment::with_prefix("mc").separator("__")`.

### CLI-only (not in hierarchy)

These are per-invocation intent, not project configuration:

- `--force` / `-f` — destructive action confirmation
- `--verbose` / `--quiet` — log level (handled by `clap_verbosity_flag`)
- `--config` / `-c` — meta: which config file to load (must resolve before the hierarchy runs)
- `completion <shell>` — unrelated to project config
- Positional `sources` args for validate/render/merge (these are inputs, not settings)

## Remaining Gaps

### 1. `--root` not injected into hierarchy

`cli.root` is canonicalized and used to resolve paths locally, but it's not pushed into the config system as a `project_dir` override. This means `MC_PROJECT_DIR` from env and `project_dir` from config.toml work, but `--root` bypasses the hierarchy.

**Fix**: Add `overrides.push(("project_dir", ...))` when `cli.root` differs from default.

### 2. No env var integration test

The `__` separator is wired but no test exercises it end-to-end (e.g., setting `MC_INPUT__DATA` in the environment and verifying it resolves).

### 3. `RUN_MODE` layer kept

The plan suggested removing the environment-specific config layer (`development.toml`, `staging.toml`, etc.). It was kept — harmless but undocumented. The effective hierarchy is:

```
embedded defaults → RUN_MODE config (optional) → config.toml → MC_* env vars → CLI args
```

### 4. `--no-validate-input` / `--no-validate-output` flags not added

Per design decision #5, these were deferred. Env vars (`MC_INPUT__VALIDATE=false`) cover the need.

### 5. README not updated

The hierarchical settings task checkbox in README.md is still unchecked. No configuration section documenting the precedence chain or available env vars was added.

## Design Decisions

1. **`force` stays CLI-only** — putting `force = true` in config.toml is a footgun. It's per-invocation intent.
2. **`sources` (positional args) stay CLI-only** — these are pipeline inputs, not project-level config. You don't configure "which files to validate" in config.toml.
3. **Double underscore separator for env vars** — `MC_INPUT__DATA` not `MC_INPUT_DATA`, because single underscore is ambiguous (is `MC_INPUT_DATA` the key `input_data` or nested `input.data`?).
4. **`RUN_MODE` layer** — kept despite plan to remove. No known use case, but doesn't hurt.
5. **No new CLI flags for validate/output booleans yet** — deferred; env var path covers the need.

## Out of Scope

- Config file format changes (staying with TOML)
- New settings beyond what currently exists
- Config file generation/migration tooling
- Watch mode or config reload
