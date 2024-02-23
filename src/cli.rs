use std::path::PathBuf;

use clap::{Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser)]
#[clap(version, author, about)]
pub struct Cli {
    /// Directory to use as root of project
    #[clap(short = 'r', long, default_value = ".")]
    pub root: PathBuf,

    /// Path to a config file other than config.toml in the root of project
    #[clap(short = 'c', long, default_value = "config.toml")]
    pub config: PathBuf,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Create a new modelcard project
    Init {
        /// Name of the project. Will create a new directory with that name in the current directory
        #[clap(default_value = ".")]
        name: String,

        /// Force creation of project even if directory is non-empty
        #[clap(short = 'f', long)]
        force: bool,
    },

    /// Deletes the output directory if there is one and builds the modelcard
    Build {
        /// The source modelcard data file to be build (defaults to all in 'data' dir in project root)
        #[clap(short = 's', long)]
        source: Option<String>,

        /// Outputs the generated site in the given path (by default 'card' dir in project root)
        #[clap(short = 'o', long)]
        output_dir: Option<String>,

        /// Force building the modelcard even if output directory is non-empty
        #[clap(short = 'f', long)]
        force: bool,
    },

    /// Try to build the project without rendering it. Checks inputs
    Check {
        /// The source modelcard data file to be verified (defaults to sample.json)
        #[clap(default_value = "sample.json")]
        source: Option<String>,
    },

    /// Generate shell completion
    Completion {
        /// Shell to generate completion for
        #[clap(value_enum)]
        shell: Shell,
    },
}