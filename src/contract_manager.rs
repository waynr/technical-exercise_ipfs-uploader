use std::sync::Arc;

use ethers::{
    contract::abigen,
    abi::Address,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
};

pub use crate::{Error, Result};

pub(crate) struct ContractManager {
    key: String,
    url: String,
}

abigen!(IPFSCIDStorage, "output/IPFSCIDStorage.json");

impl ContractManager {
    pub fn new(url: &str, key: String) -> Self {
        Self { key, url: url.to_string() }
    }

    pub async fn deploy(&self) -> Result<()> {
        let provider = Provider::<Http>::try_from(&self.url)?;

        let wallet = self.key.parse::<LocalWallet>()?.with_chain_id(1337 as u64);

        let client = Arc::new(SignerMiddleware::new(provider, wallet));

        let _ = IPFSCIDStorage::deploy(client, ())
            .map_err(|e| Error::DeployFailed(e.to_string()))?
            .send()
            .await
            .map_err(|e| Error::DeployFailed(e.to_string()))?;

        Ok(())
    }

    pub async fn store(&self, cid: String) -> Result<()> {
        let provider = Provider::<Http>::try_from(&self.url)?;

        let wallet = self.key.parse::<LocalWallet>()?.with_chain_id(1337 as u64);

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
