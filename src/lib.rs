use std::path::PathBuf;

use clap::Parser;

pub(crate) mod cli;
use cli::{Cli, SubCommands};

pub mod error;
pub use error::{Error, Result};

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        SubCommands::InstallSmartContract{ abi, binary } => {
            pathmustexist(abi)?;
            pathmustexist(binary)?;
            println!("{}", abi.display());
            println!("{}", binary.display());
            Ok(())
        },
        SubCommands::Upload{ file, key } => {
            pathmustexist(file)?;
            println!("{}", file.display());
            println!("{}", key);
            Ok(())
        },
    }
}

pub fn pathmustexist(p: &PathBuf) -> Result<()> {
    if !p.try_exists()? {
        return Err(Error::PathDoesNotExist(p.to_path_buf()))
    }
    Ok(())
}
