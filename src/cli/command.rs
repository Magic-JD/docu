use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "docu",
    about = "Easily document your CLI commands",
    version = env!("CARGO_PKG_VERSION"),
    author = "Joseph Daunt",
    after_help = "For more details, visit https://github.com/Magic-JD/docu"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Document a new scriptlet
    Add {
        /// The full invocation of your tool + scriptlet
        ///
        /// Example:
        /// docu add "sed -i '1{s/ /_/g}'" products.csv
        #[arg(required = true)]
        args: String,
    },

    /// Show existing documentation
    Show {
        /// If omitted, lists all tools; otherwise shows only this tool's scriptlets
        tool: Option<String>,
    },
}
