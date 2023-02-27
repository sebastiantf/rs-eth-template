use ethers::prelude::*;

pub async fn get_latest_block_number<P: JsonRpcClient>(
    provider: &Provider<P>,
) -> eyre::Result<U64> {
    let latest_block_number = provider.get_block_number().await?;
    Ok(latest_block_number)
}

pub async fn get_latest_block<P: JsonRpcClient>(
    provider: &Provider<P>,
) -> eyre::Result<Block<H256>> {
    let latest_block = get_latest_block_number(provider).await?;
    let latest_block = provider.get_block(latest_block).await?.unwrap();
    Ok(latest_block)
}
