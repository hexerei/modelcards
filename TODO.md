# TODO: Modelcards Project

> Last reviewed: 2025-07 — audited against commit history and codebase.

## I. High Priority

### 1. Dependency Management
- [ ] Replace `valico` with `jsonschema` crate
  - `valico` is unmaintained and pulls in heavy transitive deps (`hashbrown`, `phf_shared` duplicates)
  - `jsonschema` is modern, well-maintained, and proposed in `feature-visual-editor.md`
- [ ] Update all dependencies to latest versions (`cargo update`)
- [ ] Review and remove unnecessary dependency features

### 2. Error Handling
- [x] Remove `unwrap()` calls from production code (~1 remains in `strip_unc()`)
- [x] Replace with proper error propagation using `?` operator
- [x] Add error context using `anyhow::context()` (used in `merge.rs`, `utils.rs`)
- [ ] Add panic handler for better crash reporting in release builds
  - Consider `human-panic` crate — one-liner integration, big UX improvement

### 3. Dead Asset Cleanup
- [ ] Remove or expose unreachable HuggingFace assets
  - 4 files (`huggingface.datasetcard.schema.md`, `huggingface.modelcard.schema.md`,
    `huggingface.datasetcard.md.jinja`, `huggingface.modelcard.md.jinja`) are compiled
    into the binary via `include_str!` but have **no accessor functions** — dead weight

### 4. Build Configuration
- [x] Add release profile optimization to `Cargo.toml` (`opt-level=3`, `lto=true`, `codegen-units=1`, `strip=true`)
- [x] Add `Cargo.lock` to version control

## II. Medium Priority

### 5. Testing
- [ ] Create `tests/` directory with integration tests (run the CLI binary end-to-end)
- [ ] Add comprehensive test coverage for error cases
- [ ] Make rustdoc examples compilable (audit and remove unnecessary `no_run` markers)

### 6. CI/CD Improvements
- [x] GitHub Actions workflow for build & test (`ci.yml`)
- [x] Tag-triggered release with binary upload and crates.io publish (`release.yml`)
- [ ] Add `Swatinem/rust-cache` for faster CI builds
- [ ] Add `cargo clippy` and `cargo fmt --check` steps to CI
- [ ] Add `cargo audit` security check to CI
- [ ] Add Rust version matrix (test MSRV + stable)
- [ ] Add code coverage reporting (e.g., `cargo-tarpaulin` + Codecov)
- [ ] Use `cargo-release` for version management (automates bumps, changelog, tagging)

### 7. Code Organization
- [ ] Consolidate path utilities — move `opt_get_path` from `cmd/build.rs` into `lib/utils.rs`
- [ ] Add compile-time template validation (build script or `const` assertion)

## III. Nice to Have

### 8. Developer Experience
- [ ] Add pre-commit hooks
  - [ ] `cargo fmt` check
  - [ ] `cargo clippy` check
  - [ ] Test execution
- [ ] Create `clippy.toml` with project-specific lints
- [ ] Add `--dry-run` flag for `init` and `build` commands

### 9. Code Quality
- [ ] Add `#[derive(Debug, Clone)]` consistently across structs (currently only on `settings.rs` types)
- [ ] Broaden use of `.context()` / `.with_context()` for richer error messages

### 10. Documentation
- [ ] Document all public library APIs with compilable examples
- [ ] Add Cargo examples (`examples/*.rs`) for library consumers
- [ ] Expand design docs in `docs/` (effectively ADRs — `feature-hierarchical-config.md` and `feature-visual-editor.md` already exist)

## Completed (archived)

Items confirmed done and removed from active tracking:

- ~~Remove all `unwrap()` calls~~ — production code clean; remaining are doc examples and tests
- ~~Return `&'static str` from `include_str!` macros~~ — all 6 asset accessors return `&'static str`
- ~~Remove `String::from()` in asset modules~~ — confirmed clean
- ~~Add release profile to `Cargo.toml`~~ — aggressively optimized
- ~~Add `Cargo.lock` to version control~~ — tracked
- ~~Add GitHub Actions workflows~~ — `ci.yml` + `release.yml` with crates.io publish
- ~~Simplify verbose handling with `env_logger`~~ — integrated with `clap-verbosity-flag`
- ~~Implement hierarchical config~~ — 5-layer precedence: defaults → env mode → config.toml → env vars → CLI

## Removed (obsolete)

Items from the original TODO that were evaluated and dropped:

| Item | Reason |
|------|--------|
| `Cow<str>` for path operations | Adds complexity for negligible gain in a CLI tool |
| Caching for schemas/templates | Assets are `include_str!` (zero-cost); disk files read once per invocation |
| Profile application for bottlenecks | Tool processes small files; performance is not a concern |
| Static linking for distribution | Rust links statically by default; release workflow handles binary distribution |
| Property-based testing for JSON merge | Low ROI — merge logic is straightforward deep-merge |
| Performance benchmarks | Not a bottleneck for this tool |
| `include_flate!` asset compression | Assets are small text files; compression overhead > savings |
| Lazy static loading for templates | `include_str!` data is in `.rodata` — zero-cost to reference |
| Consolidate small asset modules | Current 3-module structure (`config/`, `schema/`, `templates/`) is clean and logical |
| `rayon` parallel processing | Single model card processing takes milliseconds; thread pool overhead > time saved |
| `#[must_use]` on Result functions | Rust compiler already warns on unused `Result` values |
| Custom error types / `thiserror` | `anyhow` is idiomatic for CLI tools; only needed if library is published independently |
| `cargo-readme` | README and lib.rs serve different audiences; manual management is fine |
| `once_cell` for lazy config | `std::sync::OnceLock` exists in std, but config is loaded once in `main()` — no globals needed |
| Command aliases | `clap` supports this natively if needed later |
| Binary size tracking | Not a real concern |
| Dependency tree complexity metric | Only relevant during `valico` replacement |