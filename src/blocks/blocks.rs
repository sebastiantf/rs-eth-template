use ethers::prelude::*;
use log::info;

pub async fn get_latest_block_number<P: JsonRpcClient>(provider: Provider<P>) -> eyre::Result<U64> {
    let latest_block_number = provider.get_block_number().await?;
    info!("Latest block: {}", latest_block_number);
    Ok(latest_block_number)
}
