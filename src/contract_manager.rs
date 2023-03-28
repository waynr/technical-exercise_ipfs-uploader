use std::sync::Arc;

use ethers::{
    contract::abigen,
    abi::Address,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::LocalWallet,
};

pub use crate::{Error, Result};

pub(crate) struct ContractManager {
    key: String,
    //wallet: LocalWallet,
}

abigen!(IPFSCIDStorage, "output/IPFSCIDStorage.json");

impl ContractManager {
    pub fn new(key: String) -> Self {
        Self { key }
    }

    pub async fn deploy(&self) -> Result<()> {
        let provider = Provider::<Http>::try_from("http://127.0.0.1:8545")?;

        let wallet = self.key.parse::<LocalWallet>()?;

        let client = Arc::new(SignerMiddleware::new(provider, wallet));

        let _ = IPFSCIDStorage::deploy(client, ())
            .map_err(|e| Error::DeployFailed(e.to_string()))?
            .send()
            .await
            .map_err(|e| Error::DeployFailed(e.to_string()))?;

        Ok(())
    }

    pub async fn store(&self, cid: String) -> Result<()> {
        let provider = Provider::<Http>::try_from("http://127.0.0.1:8545")?;

        let wallet = self.key.parse::<LocalWallet>()?;

        let client = Arc::new(SignerMiddleware::new(provider, wallet));

        let storage = IPFSCIDStorage::new(Address::zero(), client);

        storage.store(cid)
            .send()
            .await
            .map_err(|e| Error::DeployFailed(e.to_string()))?
            .await
            .map_err(|e| Error::DeployFailed(e.to_string()))?;

        Ok(())
    }
}
