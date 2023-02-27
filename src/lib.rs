pub mod config;

use config::Config;
use ethers::prelude::*;
use log::info;

pub async fn run<P: JsonRpcClient>(_config: &Config, provider: Provider<P>) -> eyre::Result<()> {
    let latest_block_number = provider.get_block_number().await?;
    info!("Latest block: {}", latest_block_number);
    Ok(())
}
