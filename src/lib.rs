use std::path::PathBuf;

use clap::Parser;

mod contract_manager;
use contract_manager::ContractManager;

pub(crate) mod cli;
use cli::{Cli, SubCommands};

pub mod error;
pub use error::{Error, Result};

pub async fn run() -> Result<()> {
    let cli = Cli::parse();
    let key = cli.key;

    match &cli.command {
        SubCommands::InstallSmartContract => {
            let mgr = ContractManager::new(key);
            mgr.deploy().await?;
            Ok(())
        },
        SubCommands::Upload{ file } => {
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
