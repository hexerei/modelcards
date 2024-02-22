use std::path::PathBuf;

use cli::{Cli, Command};

use clap::{CommandFactory, Parser};

mod cli;
mod cmd;
mod utils;

fn main() {
    let cli = Cli::parse();
    let cli_dir: PathBuf = cli.root.canonicalize().unwrap_or_else(|e| {
        eprintln!("Could not find canonical path of root dir: {}", cli.root.display());
        eprintln!("{:?}", e);
        std::process::exit(1);
    });
    println!("CLI Directory: {:?}", cli_dir);
    match cli.command {
        Command::Init { name, force } => {
            if let Err(e) = cmd::create_new_project(&name, force) {
                eprintln!("Could not create project");
                eprintln!("{:?}", e);
                std::process::exit(1);
            }
        },
        Command::Build { source, output_dir, force } => {
            println!("Build base_url={:?}, output_dir={:?}, force={force}", source, output_dir);
            todo!();
        },
        Command::Check { source } => {
            println!("Check source={:?}", source);
            if cmd::check_project(&cli_dir, source) {
                println!("Project is valid");
            } else {
                eprintln!("Project is not valid");
                std::process::exit(1);
            }
        },
        Command::Completion { shell } => {
            let cmd = &mut Cli::command();
            clap_complete::generate(shell, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
        }
    }
}
