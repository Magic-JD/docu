use crate::cli::command::Cli;
use crate::parse::parser::parse_scriptlet;
use clap::Parser;

mod cli;
mod parse;

fn main() {
    let args = Cli::parse();
    if args.scriptlet.is_none() || args.scriptlet.as_ref().unwrap().is_empty() {
        println!("No scriptlet specified");
        return;
    }
    parse_scriptlet(args.scriptlet.unwrap());
}
