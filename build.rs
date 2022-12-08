#[path = "./src/cli.rs"]
mod cli;
use clap_complete::{
    generate_to,
    shells::{Bash, Fish, Zsh},
};
use clap_complete_nushell::Nushell;
use cli::Cli;
use std::{fs::create_dir, io::Result, path::Path};

fn main() -> Result<()> {
    let cmd = &mut Cli::cmd();
    let bin_name = "ctj";
    let out_dir = "components";
    if !Path::new(out_dir).exists() {
        create_dir(out_dir)?;
    }

    generate_to(Bash, cmd, bin_name, out_dir)?;
    generate_to(Zsh, cmd, bin_name, out_dir)?;
    generate_to(Fish, cmd, bin_name, out_dir)?;
    generate_to(Nushell, cmd, bin_name, out_dir)?;

    Ok(())
}
