//! Project detection — find modelcards project structure.

use std::path::{Path, PathBuf};

/// Detected project layout.
pub struct ProjectLayout {
    pub schema: Option<PathBuf>,
    pub data_files: Vec<PathBuf>,
}

/// Detect a modelcards project at the given root.
///
/// Looks for:
/// - `schema/modelcard.schema.json` — project schema
/// - `config.toml` — project config (presence confirms it's a project)
/// - `*.json` files in root — potential data layers
/// - `sample.json` — default data file
pub fn detect(root: &Path) -> ProjectLayout {
    let schema = find_schema(root);
    let data_files = find_data_files(root);

    ProjectLayout {
        schema,
        data_files,
    }
}

fn find_schema(root: &Path) -> Option<PathBuf> {
    let candidates = [
        root.join("schema/modelcard.schema.json"),
        root.join("schema.json"),
    ];
    candidates.into_iter().find(|p| p.is_file())
}

fn find_data_files(root: &Path) -> Vec<PathBuf> {
    // If sample.json exists, use it as default
    let sample = root.join("sample.json");
    if sample.is_file() {
        return vec![sample];
    }

    // Otherwise look for any JSON files that aren't schema files
    let Ok(entries) = std::fs::read_dir(root) else {
        return Vec::new();
    };

    let mut files: Vec<PathBuf> = entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.is_file()
                && p.extension().is_some_and(|ext| ext == "json")
                && !p.file_name().is_some_and(|n| n.to_string_lossy().contains("schema"))
        })
        .collect();

    files.sort();
    files
}
