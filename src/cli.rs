use std::path::PathBuf;

use clap::{Command, CommandFactory, Parser};

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
pub struct Cli {
    #[arg(value_name = "SOURCE", help = "Source File path")]
    pub source: PathBuf,
    #[arg(short, long, value_name = "DEST", help = "Destination File path")]
    pub destination: Option<PathBuf>,
}

impl Cli {
    #[allow(dead_code)]
    pub fn build() -> Self {
        Self::parse()
    }
    pub fn cmd() -> Command {
        Self::command()
    }
}
