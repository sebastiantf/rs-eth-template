use dotenv::dotenv;

#[derive(Debug)]
pub struct Config {
    pub eth_provider_rpc: String,
}

impl Default for Config {
    fn default() -> Self {
        dotenv().ok();
        let eth_provider_rpc = dotenv::var("ETH_RPC_URL").expect("No ETH_RPC_URL");

        Config { eth_provider_rpc }
    }
}
