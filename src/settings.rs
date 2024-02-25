use modelcards::assets::config::get_default;
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

// verbose = true
// force = false
// project_dir = "."

// [input]
// data = "./sample.json"
// schema = "./schema/modelcard.schema.json"
// validate = true

// [output]
// target = "./cards/modelcard.md"
// template = "./templates/modelcard.md.jinja"
// validate = true


#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Input {
    pub data: String,
    pub schema: String,
    pub validate: bool,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Output {
    pub target: String,
    pub template: String,
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
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::from_str(get_default().as_str(), config::FileFormat::Toml))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(
                File::with_name(&format!("{}", run_mode))
                    .required(false),
            )
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name(config_name).required(false))
            // Add in settings from the environment (with a prefix of MC)
            // Eg.. `MC_VERBOSE=1 ./target/modelcards` would set the `verbose` key
            .add_source(Environment::with_prefix("mc"))
            // You may also programmatically change settings
            .set_override("verbose", true)?
            .build()?;

        // Now that we're done, let's access our configuration
        println!("verbose: {:?}", s.get_bool("verbose"));
        println!("project_dir: {:?}", s.get::<String>("project_dir"));

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings() {
        let settings = Settings::new("config").expect("Could not load settings");
        assert_eq!(settings.verbose, true);
    }
}