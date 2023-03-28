use ipfs_uploader::error::Result;
use ipfs_uploader::run;

fn main() -> anyhow::Result<()> {
    run()?;
    Ok(())
}

