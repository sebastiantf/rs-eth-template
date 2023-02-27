use ethers::prelude::*;
use log::{debug, info};

use rs_eth_template::config::Config;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    debug!("Initializing Config...");
    let config = Config::default();
    info!("Config: {:#?}", &config);

    debug!("Initializing Provider...");
    if config.eth_provider_rpc.starts_with("http") {
        let provider = Provider::<Http>::try_from(&config.eth_provider_rpc)?;
        rs_eth_template::run(&config, provider).await?;
    } else {
        let ws = Ws::connect(&config.eth_provider_rpc).await?;
        let provider = Provider::new(ws);
        rs_eth_template::run(&config, provider).await?;
    }

    Ok(())
}
