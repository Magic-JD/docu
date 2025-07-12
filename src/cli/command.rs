use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "docu")]
#[command(about = "Easily document your cli commands")]
#[command(version = env!("CARGO_PKG_VERSION"), author = "Joseph Daunt")]
#[command(after_help = "For more details, visit https://github.com/Magic-JD/docu")]
pub struct Cli {
    /// The command to document a scriptlet
    ///
    /// The given scriptlet will be documented
    ///
    /// Example Usage:
    ///
    /// docu sed -i '1{s/ /_/g}' products.csv
    #[arg(help = "Document a given scriptlet")]
    pub(crate) scriptlet: Option<Vec<String>>,
}
