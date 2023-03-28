use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    /// Error returned when someone erroneously tries to run this program without a subcommand.
    #[error("must include subcommand")]
    MustIncludeSubCommand,

    /// Wrapper around `std::io::Error`.
    #[error("error: {0}")]
    StdIOError(#[from] std::io::Error),

    /// When a path doesn't exist, that's-a-error!
    #[error("path doesn't exist: {0}")]
    PathDoesNotExist(std::path::PathBuf),

    /// When wallets error.
    #[error("wallet error: {0}")]
    WalletError(#[from] ethers::signers::WalletError),

    /// When parses error.
    #[error("parse error: {0}")]
    ParseError(#[from] url::ParseError),

    /// Smart contract deploy failed.
    #[error("smart contract deploy failed: {0}")]
    DeployFailed(String),
}
