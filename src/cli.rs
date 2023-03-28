use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: SubCommands,

    /// Wallet key for account to associate IPFS CID with.
    #[arg(short, long, value_name = "WALLET_KEY")]
    pub key: String,
}

#[derive(Subcommand)]
pub(crate) enum SubCommands {
    /// Install smart contract in blockchain or whatever.
    InstallSmartContract,

    /// Upload file to IPFS, load returned CID into smart contract for specified wallet.
    Upload {
        /// Specify the file to be uploaded to IPFS
        #[arg(short, long, value_name = "FILE")]
        file: PathBuf,
    },
}
