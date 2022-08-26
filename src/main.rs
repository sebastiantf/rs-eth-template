use log::{debug, info};

use rs_eth_template::config::Config;

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    debug!("Initializing Config...");
    let config = Config::default();
    info!("Config: {:#?}", &config);

    rs_eth_template::run();
}
