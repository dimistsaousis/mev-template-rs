use ethers::{
    prelude::{k256::ecdsa::SigningKey, Http, Provider, SignerMiddleware},
    providers::Middleware,
    signers::{LocalWallet, Signer, Wallet},
};
use std::sync::Arc;

#[derive(Debug)]
pub struct Config {
    pub http: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    pub wss: String,
}

impl Config {
    pub async fn new() -> Self {
        let netword_rpc = std::env::var("NETWORK_RPC").expect("missing NETWORK_RPC");
        let netword_wss = std::env::var("NETWORK_WSS").expect("missing NETWORK_WSS");
        let http = Provider::try_from(netword_rpc).unwrap();
        let http = setup_signer(http).await;
        let http = Arc::new(http);

        Config {
            http,
            wss: netword_wss,
        }
    }
}

async fn setup_signer(
    provider: Provider<Http>,
) -> SignerMiddleware<Provider<Http>, Wallet<SigningKey>> {
    let chain_id = provider
        .get_chainid()
        .await
        .expect("Failed to get chain id");
    let private_key = std::env::var("PRIVATE_KEY").expect("Missing private key");
    let wallet = private_key
        .parse::<LocalWallet>()
        .expect("Failed to parse wallet")
        .with_chain_id(chain_id.as_u64());
    SignerMiddleware::new(provider, wallet)
}
