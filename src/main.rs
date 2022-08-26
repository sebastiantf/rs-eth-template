fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    rs_eth_template::run();
}
