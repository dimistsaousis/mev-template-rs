#[derive(Debug)]
pub struct Config {
    pub http: String,
    pub wss: String,
}

impl Config {
    pub async fn new() -> Self {
        let netword_rpc = std::env::var("NETWORK_RPC").expect("missing NETWORK_RPC");
        let netword_wss = std::env::var("NETWORK_WSS").expect("missing NETWORK_WSS");
        Config {
            http: netword_rpc,
            wss: netword_wss,
        }
    }
}
