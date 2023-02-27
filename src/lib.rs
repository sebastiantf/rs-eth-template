pub mod blocks;
pub mod config;

use blocks::get_latest_block;
use config::Config;
use ethers::prelude::*;
use log::info;

pub async fn run<P: JsonRpcClient>(_config: &Config, provider: Provider<P>) -> eyre::Result<()> {
    let latest_block = get_latest_block(&provider).await?;
    info!("Latest block: {}", latest_block.number.unwrap());
    info!(
        "Ts: {:?}, block number: {} -> {:?}",
        latest_block.timestamp,
        latest_block.number.unwrap(),
        latest_block.hash.unwrap()
    );
    println!("Got block: {}", serde_json::to_string_pretty(&latest_block)?);
    Ok(())
}
