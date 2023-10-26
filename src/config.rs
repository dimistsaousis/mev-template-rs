use ethers::prelude::{Http, Provider};

#[derive(Debug)]
pub struct Config {
    pub http: Provider<Http>,
    pub wss: String,
}

impl Config {
    pub async fn new() -> Self {
        let netword_rpc = std::env::var("NETWORK_RPC").expect("missing NETWORK_RPC");
        let netword_wss = std::env::var("NETWORK_WSS").expect("missing NETWORK_WSS");
        let http = Provider::try_from(netword_rpc).unwrap();

        Config {
            http,
            wss: netword_wss,
        }
    }
}
