use crate::cli::command::{Cli, Commands};
use crate::display::scriptlets::{show_all_scriptlets, show_all_scriptlets_for_tool};
use crate::parse::parser::parse_scriptlet;
use clap::Parser;

mod cli;
mod database;
mod display;
mod errors;
mod parse;
mod tui;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Add { args } => {
            if args.is_empty() {
                println!("No scriptlet specified");
                return;
            }
            parse_scriptlet(args);
        }
        Commands::Show { tool } => match tool {
            None => show_all_scriptlets(),
            Some(tool_name) => show_all_scriptlets_for_tool(&tool_name),
        },
    }
}
