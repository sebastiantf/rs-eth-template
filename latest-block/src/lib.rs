use ethers::prelude::*;
use log::info;
use utils::{blocks::get_latest_block, config::Config};

pub async fn run<P: JsonRpcClient>(_config: &Config, provider: Provider<P>) -> eyre::Result<()> {
    let latest_block = get_latest_block(&provider).await?;
    info!("Latest block: {}", latest_block.number.unwrap());
    info!(
        "Ts: {:?}, block number: {} -> {:?}",
        latest_block.timestamp,
        latest_block.number.unwrap(),
        latest_block.hash.unwrap()
    );
    println!(
        "Got block: {}",
        serde_json::to_string_pretty(&latest_block)?
    );
    Ok(())
}
