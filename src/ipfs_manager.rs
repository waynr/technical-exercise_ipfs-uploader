use std::io::Cursor;
use std::path::PathBuf;

use ipfs_api::{IpfsApi, IpfsClient, TryFromUri};
use tokio::fs::read_to_string;

use crate::error::Result;

pub struct IPFSManager {
    client: IpfsClient,
}

impl IPFSManager {
    pub fn new(uri: &str) -> Result<Self> {
        Ok(Self{
            client: IpfsClient::from_str(uri)?,
        })
    }

    pub async fn upload(&self, p: &PathBuf) -> Result<String> {
        let file = read_to_string(p).await?;
        let cursor = Cursor::new(file);
        let s = self.client.add(cursor).await?;
        Ok(s.hash)
    }
}
