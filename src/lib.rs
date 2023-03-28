use std::path::PathBuf;

use clap::Parser;

mod contract_manager;
use contract_manager::ContractManager;

mod ipfs_manager;
use ipfs_manager::IPFSManager;

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
            let ipfs_mgr = IPFSManager::new("http://127.0.0.1:5001")?;
            let contract_mgr = ContractManager::new(key);
            let cid = ipfs_mgr.upload(file).await?;
            println!("{0}", cid);
            contract_mgr.store(cid).await?;
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
