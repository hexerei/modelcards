use std::path::PathBuf;

use cli::{Cli, Command};
use settings::Settings;
use modelcards::utils::console;

use clap::{CommandFactory, Parser};

mod cli;
mod cmd;
mod settings;

fn main() {
    let cli = Cli::parse();
    env_logger::Builder::new().filter_level(cli.verbose.log_level_filter()).init();
    let cli_dir: PathBuf = cli.root.canonicalize().unwrap_or_else(|e| {
        console::error_exit(&format!("Could not find canonical path of root dir: {}", cli.root.display()), Some(e));
        unreachable!();
    });
    log::debug!("CLI path: {:?}", cli_dir);

    // Collect CLI argument overrides for the config hierarchy.
    // Precedence: defaults < config.toml < env vars (MC_*) < CLI args
    let mut overrides: Vec<(&str, String)> = vec![];
    match &cli.command {
        Command::Build { source, target, .. } => {
            if let Some(s) = source {
                overrides.push(("input.data", s.clone()));
            }
            if let Some(t) = target {
                overrides.push(("output.target", t.clone()));
            }
        }
        Command::Check { source: Some(s) } => {
            overrides.push(("input.data", s.clone()));
        }
        Command::Validate { schema: Some(s), .. } => {
            overrides.push(("input.schema", s.clone()));
        }
        Command::Render { template: Some(t), .. } => {
            overrides.push(("output.template", t.clone()));
        }
        _ => {}
    }

    let settings = Settings::with_overrides(
        cli.config.display().to_string().as_str(),
        overrides,
    ).expect("Could not load settings");
    log::debug!("Settings: {:?}", settings);

    match cli.command {
        Command::Merge { sources, target } => {
            log::debug!("Merge sources={:?}, target={:?}", sources, target);
            if let Err(e) = cmd::merge_modelcards(sources, target) {
                console::error_exit("Could not merge modelcards", Some(e));
            }
            console::success_exit("Modelcards successfully merged!");
        },
        Command::Validate { sources, .. } => {
            log::debug!("Validate data={:?}, schema={:?}", sources, settings.input.schema);
            match cmd::validate_modelcard(sources, settings.input.schema) {
                Ok(true) => console::success_exit("Modelcard is valid!"),
                Ok(false) => console::success_exit("Modelcard is not valid!"),
                Err(e) => console::error_exit("Could not validate modelcard!", Some(e)),
            }
        },
        Command::Render { sources, .. } => {
            log::debug!("Render data={:?}, template={:?}", sources, settings.output.template);
            match cmd::render_modelcard(sources, settings.output.template) {
                Ok(true) => console::success_exit("Modelcard successfully rendered!"),
                Ok(false) => console::success_exit("Could not render modelcard!"),
                Err(e) => console::error_exit("Could not render modelcard!", Some(e)),
            }
        },
        Command::Init { name, force } => {
            if let Err(e) = cmd::create_new_project(&name, force) {
                console::error_exit("Could not create project", Some(e));
            }
        },
        Command::Build { force, .. } => {
            log::debug!("Build source={:?}, target={:?}, force={:?}", settings.input.data, settings.output.target, force);
            let force = force.unwrap_or(settings.force);
            if let Err(e) = cmd::build_project(&cli_dir, Some(settings.input.data), Some(settings.output.target), force) {
                console::error_exit("Could not build project", Some(e));
            }
            console::success_exit("Project successfully buildt!");
        },
        Command::Check { .. } => {
            log::debug!("Check source={:?}", settings.input.data);
            let valid = cmd::check_project(&cli_dir, Some(settings.input.data));
            if valid.is_ok() {
                console::success_exit("Project is valid!");
            } else {
                console::error_exit("Project could not be validated!", valid.err());
            }
        },
        Command::Completion { shell } => {
            let cmd = &mut Cli::command();
            clap_complete::generate(shell, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
        }
    }
}
