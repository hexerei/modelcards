use config::{Config, ConfigError, Environment, File};
use modelcards::assets::config::get_default;
use serde::Deserialize;
use std::env;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Input {
    pub data: String,
    pub schema: Option<String>,
    pub validate: bool,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Output {
    pub target: String,
    pub template: Option<String>,
    pub validate: bool,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub project_dir: String,
    pub verbose: bool,
    pub force: bool,
    pub input: Input,
    pub output: Output,
}

impl Settings {
    pub fn new(config_name: &str) -> Result<Self, ConfigError> {
        Self::with_overrides(config_name, vec![])
    }

    pub fn with_overrides(
        config_name: &str,
        overrides: Vec<(&str, String)>,
    ) -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let mut builder = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::from_str(get_default(), config::FileFormat::Toml))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(File::with_name(&run_mode.to_string()).required(false))
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name(config_name).required(false))
            // Add in settings from the environment (with a prefix of MC)
            // Eg.. `MC_VERBOSE=1 ./target/modelcards` would set the `verbose` key
            // Use __ as separator for nested keys: MC_INPUT__DATA, MC_OUTPUT__TEMPLATE
            .add_source(Environment::with_prefix("mc").separator("__"));

        // Apply CLI argument overrides as the highest-priority layer
        for (key, value) in overrides {
            builder = builder.set_override(key, value)?;
        }

        let s = builder.build()?;

        // Now that we're done, let's access our configuration
        log::debug!("verbose: {:?}", s.get_bool("verbose"));
        log::debug!("project_dir: {:?}", s.get::<String>("project_dir"));

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_defaults() {
        let settings = Settings::new("config").expect("Could not load settings");
        assert!(settings.verbose);
        assert!(!settings.force);
        assert_eq!(settings.project_dir, ".");
        assert_eq!(settings.input.data, "./sample.json");
        assert!(settings.input.schema.is_none());
        assert!(settings.input.validate);
        assert_eq!(settings.output.target, "./cards/modelcard.md");
        assert!(settings.output.template.is_none());
        assert!(settings.output.validate);
    }

    #[test]
    fn test_with_overrides_applies_flat_key() {
        let overrides = vec![
            ("verbose", "false".to_string()),
            ("project_dir", "/custom/path".to_string()),
        ];
        let settings = Settings::with_overrides("nonexistent_config", overrides)
            .expect("Could not load settings with overrides");
        assert!(!settings.verbose);
        assert_eq!(settings.project_dir, "/custom/path");
    }

    #[test]
    fn test_with_overrides_applies_nested_key() {
        let overrides = vec![
            ("input.data", "custom.json".to_string()),
            ("input.schema", "/path/to/schema.json".to_string()),
            ("output.target", "/output/card.md".to_string()),
            ("output.template", "/templates/custom.jinja".to_string()),
        ];
        let settings = Settings::with_overrides("nonexistent_config", overrides)
            .expect("Could not load settings with overrides");
        assert_eq!(settings.input.data, "custom.json");
        assert_eq!(
            settings.input.schema,
            Some("/path/to/schema.json".to_string())
        );
        assert_eq!(settings.output.target, "/output/card.md");
        assert_eq!(
            settings.output.template,
            Some("/templates/custom.jinja".to_string())
        );
    }

    #[test]
    fn test_with_overrides_does_not_clobber_unset_optionals() {
        let overrides = vec![("input.data", "other.json".to_string())];
        let settings = Settings::with_overrides("nonexistent_config", overrides)
            .expect("Could not load settings with overrides");
        assert_eq!(settings.input.data, "other.json");
        assert!(settings.input.schema.is_none());
        assert!(settings.output.template.is_none());
    }

    /// Write a test config file and return the config name (without extension)
    /// suitable for passing to Settings::new / Settings::with_overrides.
    fn write_test_config(dir: &Path) -> String {
        let config_path = dir.join("config.toml");
        std::fs::write(
            &config_path,
            r#"
verbose = true
force = false
project_dir = "."

[input]
data = "./sample.json"
schema = "./schema/modelcard.schema.json"
validate = true

[output]
target = "./cards/modelcard.md"
template = "./templates/modelcard.md.jinja"
validate = true
"#,
        )
        .expect("Could not write test config");
        // Return path without .toml extension (config crate adds it)
        dir.join("config").to_str().unwrap().to_string()
    }

    #[test]
    fn test_config_file_overrides_defaults() {
        let tmp = std::env::temp_dir().join("mc_test_config_overrides");
        std::fs::create_dir_all(&tmp).unwrap();
        let config_name = write_test_config(&tmp);

        let settings =
            Settings::new(&config_name).expect("Could not load settings from test config");
        assert_eq!(
            settings.input.schema,
            Some("./schema/modelcard.schema.json".to_string())
        );
        assert_eq!(
            settings.output.template,
            Some("./templates/modelcard.md.jinja".to_string())
        );

        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn test_cli_overrides_beat_config_file() {
        let tmp = std::env::temp_dir().join("mc_test_cli_overrides");
        std::fs::create_dir_all(&tmp).unwrap();
        let config_name = write_test_config(&tmp);

        let overrides = vec![("input.schema", "/cli/schema.json".to_string())];
        let settings =
            Settings::with_overrides(&config_name, overrides).expect("Could not load settings");
        // CLI override should win over config file value
        assert_eq!(settings.input.schema, Some("/cli/schema.json".to_string()));

        std::fs::remove_dir_all(&tmp).ok();
    }
}
