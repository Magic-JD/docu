use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "docu",
    about = "A CLI tool to document and manage your favorite command-line scriptlets. A scriptlet is a small piece of code or a command that you use frequently. This command allows you to save and document them for future reference.",
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
    /// Add a new scriptlet to your documentation.
    Add {
        /// The full command invocation to be documented.
        /// It should be wrapped in quotes, and any internal quotation marks or $ should be
        /// escaped.
        #[arg(required = true)]
        args: String,
    },

    /// Display saved scriptlets. You can either display all scriptlets or filter them by a specific tool.
    Show {
        /// The specific tool to show scriptlets for. If omitted, all scriptlets for all tools are shown.
        tool: Option<String>,
    },

    /// Search for scriptlets by keywords.
    Search {
        /// The keyword(s) to search for in the scriptlet documentation.
        search: Vec<String>,
    },

    /// Remove one or more scriptlets by their ID. You can get the ID of
    /// a scriptlet by using the `show` or `search` command.
    Remove {
        /// The ID(s) of the scriptlet(s) to remove.
        ids: Vec<i64>,
    },

    /// Generate a default configuration file. This command creates a `docu.toml` file in the application's
    /// default configuration directory, pre-populated with default settings.
    GenerateConfig,
}
