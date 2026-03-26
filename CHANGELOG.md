# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [0.1.4] - 2026-03-26

### Added

- crates.io publish job in release workflow
- changelog support in GitHub release

### Changed

- cleanup: remove dead code, fix error messages, improve idioms
- added release profile for smaller binary
- fixed unnecessary string allocation in assets module
- error handling improvements
- style and pathref improvements

## [0.1.1] - 2024-07-14

### Added

- merge, validate and render subcommands
- merge function for JSON model cards
- validate subcommand
- render functionality

### Changed

- some how-to documentation added
- bumped version to 0.1.1
- more tests
- pass through error from load_json_file

## [0.1.0] - 2024-04-26

### Added

- init subcommand with templates and schema
- build subcommand initial implementation
- check subcommand
- settings/configuration system
- JSON schema validation
- env_logger for logging
- CI and release GitHub Actions workflows

### Changed

- big refactoring of codebase
- better HuggingFace sample data
- BibTeX support
- default config on init

### Fixed

- license validation
- schema validation for image/png
- compiler warnings
