pub mod blocks;
pub mod config;

use blocks::get_latest_block_number;
use config::Config;
use ethers::prelude::*;
use log::info;

pub async fn run<P: JsonRpcClient>(_config: &Config, provider: Provider<P>) -> eyre::Result<()> {
    let latest_block_number = get_latest_block_number(provider).await?;
    info!("Latest block: {}", latest_block_number);
    Ok(())
}
