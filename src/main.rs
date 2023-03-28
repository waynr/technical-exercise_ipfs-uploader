use ipfs_uploader::run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run().await?;
    Ok(())
}

