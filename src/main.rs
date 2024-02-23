use std::path::PathBuf;

use cli::{Cli, Command};

use clap::{CommandFactory, Parser};

mod cli;
mod cmd;

fn main() {
    let cli = Cli::parse();
    let cli_dir: PathBuf = cli.root.canonicalize().unwrap_or_else(|e| {
        console::error_exit(&format!("Could not find canonical path of root dir: {}", cli.root.display()), Some(e));
        unreachable!(); // Add this line to satisfy the expected return type of `PathBuf`
    });
    console::debug(&format!("CLI path: {:?}", cli_dir));
    match cli.command {
        Command::Init { name, force } => {
            if let Err(e) = cmd::create_new_project(&name, force) {
                console::error_exit("Could not create project", Some(e));
            }
        },
        Command::Build { source, output_dir, force } => {
            console::debug(&format!("Build base_url={:?}, output_dir={:?}, force={force}", source, output_dir));
            if let Err(e) = cmd::build_project(&cli_dir, source, output_dir, force) {
                console::error_exit("Could not build project", Some(e));
            }
            console::success_exit("Project successfully buildt!");
        },
        Command::Check { source } => {
            console::debug(&format!("Check source={:?}", source));
            let valid = cmd::check_project(&cli_dir, source);
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

#[allow(dead_code)]
mod console {
    //use std::io::Write;

    pub fn error(msg: &str, e: Option<impl std::fmt::Debug>) {
        eprintln!("Error: {}", msg);
        if let Some(e) = e {
            eprintln!("{:?}", e);
        }
    }

    pub fn error_exit(msg: &str, e: Option<impl std::fmt::Debug>) {
        error(msg, e);
        std::process::exit(1);
    }

    pub fn warn(msg: &str) {
        eprintln!("Warning: {}", msg);
    }

    pub fn info(msg: &str) {
        println!("{}", msg);
    }

    pub fn success_exit(msg: &str) {
        info(msg);
        std::process::exit(0);
    }

    #[cfg(debug_assertions)]
    pub fn debug(msg: &str) {
        println!("Debug: {}", msg);
    }

    #[cfg(not(debug_assertions))]
    pub fn debug(_msg: &str) {}
}
