use ethers::prelude::{k256::ecdsa::SigningKey, Http, Provider, SignerMiddleware, Wallet, U64};
use ethers::providers::Middleware;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

pub async fn loop_blocks(http: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>) {
    let mut last_block: U64 = U64::zero();
    loop {
        if let Ok(block) = http.get_block_number().await {
            if block > last_block {
                last_block = block;
                println!("\n---------- BLOCK: {:?} ----------", block);
            }
        }
        sleep(Duration::from_millis(1)).await;
    }
}
