# Modelcards

modelcards is a Rust CLI tool and library for generating, validating, and managing machine learning model cards. It supports both Pipeline mode (for CI/CD integration) and Project mode (for creating custom schemas/templates).

## Build and Development Commands

```bash
# Build
cargo build              # Debug build
cargo build --release    # Release build

# Test
cargo test              # Run all tests
cargo test --lib        # Library tests only
cargo test <test_name>  # Run specific test

# Development
cargo run -- [command]  # Run CLI during development
cargo fmt               # Format code
cargo clippy            # Lint code

# Example CLI usage
cargo run -- merge defaults.json model.json -o output.json
cargo run -- validate modelcard.json
cargo run -- render modelcard.json
cargo run -- init my-project
cargo run -- build
cargo run -- check
```

## Architecture

The codebase follows a command pattern with clear separation:

- **Entry Points**: `src/main.rs` (CLI), `src/lib/lib.rs` (library API)
- **Commands**: Each subcommand (init, build, merge, validate, render, check) has its own module in `src/cmd/`
- **Core Logic**: Library functionality in `src/lib/` handles merging, rendering, and validation
- **Assets**: Embedded resources (templates, schemas, config) in `src/lib/assets/` using `include_str!`

Key architectural decisions:
- Uses hierarchical configuration: defaults → config.toml → env vars (MC_*) → CLI args
- Supports both Google Model Card Toolkit and HuggingFace formats
- Template rendering via minijinja with custom filters
- JSON schema validation using valico

## Important Context

1. **Error Handling**: Uses anyhow for error propagation with context
2. **Logging**: env_logger with verbosity control (-v/-q flags)
3. **File Paths**: Always use absolute paths when working with file operations
4. **Testing**: Tests use temporary directories via `tempfile` crate
5. **Configuration**: Settings struct in `src/settings.rs` manages all configuration with `config` crate
