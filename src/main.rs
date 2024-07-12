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
        unreachable!(); // Add this line to satisfy the expected return type of `PathBuf`
    });
    log::debug!("CLI path: {:?}", cli_dir);

    let settings = Settings::new(cli.config.display().to_string().as_str()).expect("Could not load settings");
    log::debug!("Settings: {:?}", settings);


    match cli.command {
        Command::Merge { sources, target } => {
            log::debug!("Merge sources={:?}, target={:?}", sources, target);
            if let Err(e) = cmd::merge_modelcards(sources, target) {
                console::error_exit("Could not merge modelcards", Some(e));
            }
            console::success_exit("Modelcards successfully merged!");
        },
        Command::Validate { sources, schema} => {
            log::debug!("Validate data={:?}, schema={:?}", sources, schema);
            if cmd::validate_modelcard(sources, schema) {
                console::success_exit("Modelcard is valid!");
            } else {
                console::success_exit("Modelcard is not valid!");
                //console::error_exit("Modelcard is not valid!", None);
            }
        },
        Command::Render { sources, template} => {
            log::debug!("Render data={:?}, template={:?}", sources, template);
            if cmd::render_modelcard(sources, template) {
                console::success_exit("Modelcard successfully rendered!");
            } else {
                console::success_exit("Could not render modelcard!");
                //console::error_exit("Could not render modelcard!", None);
            }
        },
        Command::Init { name, force } => {
            if let Err(e) = cmd::create_new_project(&name, force) {
                console::error_exit("Could not create project", Some(e));
            }
        },
        Command::Build { source, target, force } => {
            log::debug!("Build base_url={:?}, output_dir={:?}, force={:?}", source, target, force);
            let source = source.unwrap_or(settings.input.data);
            let target = target.unwrap_or(settings.output.target);
            if let Err(e) = cmd::build_project(&cli_dir, Some(source), Some(target), force.unwrap_or(settings.force)) {
                console::error_exit("Could not build project", Some(e));
            }
            console::success_exit("Project successfully buildt!");
        },
        Command::Check { source } => {
            log::debug!("Check source={:?}", source);
            let source = source.unwrap_or(settings.input.data);
            let valid = cmd::check_project(&cli_dir, Some(source));
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

