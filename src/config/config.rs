use dotenv::dotenv;

#[derive(strum::Display)]
pub enum ConfigKey {
    #[strum(serialize = "ETH_RPC_URL")]
    EthRpcUrl,
}

#[derive(Debug)]
pub struct Config {
    pub eth_provider_rpc: String,
}

impl Default for Config {
    fn default() -> Self {
        dotenv().ok();
        let eth_provider_rpc = dotenv::var(ConfigKey::EthRpcUrl.to_string())
            .expect(&format!("{} not found!", ConfigKey::EthRpcUrl.to_string()));

        Config { eth_provider_rpc }
    }
}
