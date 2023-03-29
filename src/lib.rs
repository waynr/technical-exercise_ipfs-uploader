use clap::Parser;

mod contract_manager;
use contract_manager::ContractManager;

mod ipfs_manager;
use ipfs_manager::IPFSManager;

pub(crate) mod cli;
use cli::{Cli, SubCommands};

pub mod error;
pub use error::{Error, Result};

const ETHEREUM_ADDR: &'static str = "http://127.0.0.1:8545"; 
const IPFS_ADDR: &'static str = "http://127.0.0.1:5001"; 

pub async fn run() -> Result<()> {
    let cli = Cli::parse();
    let key = cli.key;

    match &cli.command {
        SubCommands::InstallSmartContract => {
            let mgr = ContractManager::new(ETHEREUM_ADDR, key);

            mgr.deploy().await?;
            println!("contract deployed");

            Ok(())
        }
        SubCommands::Upload { file } => {
            if !file.try_exists()? {
                return Err(Error::PathDoesNotExist(file.to_path_buf()));
            }

            let ipfs_mgr = IPFSManager::new(IPFS_ADDR)?;
            let contract_mgr = ContractManager::new(ETHEREUM_ADDR, key);

            let cid = ipfs_mgr.upload(file).await?;
            println!("file uploaded with CID: {0}", cid);

            contract_mgr.store(cid).await?;
            println!("contract updated with CID");

            Ok(())
        }
    }
}
