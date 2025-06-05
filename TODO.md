# TODO: Modelcards Project Optimizations

## 🔴 High Priority

### 1. Error Handling
- [x] Remove all `unwrap()` calls (20+ instances found) ✅
- [x] Replace with proper error propagation using `?` operator ✅
- [x] Add consistent error context using `anyhow::context()` ✅
- [ ] Create custom error types for domain-specific errors
- [ ] Add panic handler for better error reporting in release builds

### 2. Performance Optimizations
- [x] Fix unnecessary string allocations in asset modules ✅
  - [x] Return `&'static str` directly from `include_str!` macros ✅
  - [x] Remove `String::from()` in `assets/schema/mod.rs` ✅
  - [x] Remove `String::from()` in `assets/templates/mod.rs` ✅
- [ ] Use `Cow<str>` for path operations to avoid allocations
- [ ] Add caching for frequently accessed files (schemas, templates)
- [ ] Profile application to identify actual bottlenecks

### 3. Dependency Management
- [ ] Resolve duplicate dependencies
  - [ ] Fix `hashbrown` multiple versions
  - [ ] Fix `phf_shared` multiple versions
- [ ] Consider replacing `valico` with lighter JSON schema validator
- [ ] Review and remove unnecessary dependency features
- [ ] Update all dependencies to latest versions

### 4. Build Configuration
- [ ] Add release profile optimization to `Cargo.toml`:
  ```toml
  [profile.release]
  opt-level = 3
  lto = true
  codegen-units = 1
  strip = true
  ```
- [ ] Add `Cargo.lock` to version control
- [ ] Consider static linking for easier distribution

## 🟡 Medium Priority

### 5. Test Structure
- [ ] Create `tests/` directory for integration tests
- [ ] Add comprehensive test coverage for error cases
- [ ] Consider property-based testing for JSON merging
- [ ] Add benchmarks for performance-critical operations
- [ ] Add rustdoc tests to ensure examples stay current

### 6. Asset Management
- [ ] Implement asset compression using `include_flate!`
- [ ] Add lazy static loading for rarely-used templates
- [ ] Implement compile-time validation for templates
- [ ] Consolidate small asset modules into single module

### 7. Parallel Processing
- [ ] Add `rayon` dependency for parallel processing
- [ ] Implement parallel model card processing in build command
- [ ] Parallelize validation operations for multiple files

### 8. Code Organization
- [ ] Create dedicated path utilities module
- [ ] Consolidate duplicate path handling logic
- [ ] Reorganize small modules that could be combined
- [ ] Add `#[must_use]` on functions returning Results

## 🟢 Nice to Have

### 9. CI/CD Setup
- [ ] Add GitHub Actions workflow
  - [ ] Test matrix for different Rust versions
  - [ ] Dependency caching
  - [ ] Code coverage reporting
  - [ ] Release automation
- [ ] Use `cargo-release` for version management
- [ ] Add security audit in CI pipeline

### 10. Documentation
- [ ] Create `examples/` directory with real-world usage
- [ ] Add more examples to library documentation
- [ ] Use `cargo-readme` to generate README from lib.rs
- [ ] Document all public APIs with examples
- [ ] Add architecture decision records (ADRs)

### 11. Developer Experience
- [ ] Add pre-commit hooks
  - [ ] `cargo fmt` check
  - [ ] `cargo clippy` check
  - [ ] Test execution
- [ ] Create `clippy.toml` with project-specific lints
- [ ] Add `--dry-run` flag for destructive operations
- [ ] Implement command aliases for common operations
- [ ] Use `once_cell` for lazy static configuration

### 12. Code Quality
- [ ] Add `#[derive(Debug, Clone)]` consistently across structs
- [ ] Consider using `thiserror` for ergonomic error types
- [ ] Add configuration validation at startup
- [ ] Simplify verbose handling with env_logger

## 📊 Metrics to Track

- [ ] Reduction in binary size after optimizations
- [ ] Performance benchmarks before/after changes
- [ ] Test coverage percentage
- [ ] Number of `unwrap()` calls remaining
- [ ] Dependency tree complexity

## 🎯 Quick Wins

1. **Add release profile**: ~5 minutes, significant binary size/speed improvement
2. **Fix duplicate dependencies**: ~30 minutes, cleaner dependency tree

## 📝 Notes

- Most optimizations are backwards compatible
- Priority based on impact vs effort ratio
- Consider creating feature branches for major changes
- Run benchmarks before implementing performance optimizations to verify actual impact