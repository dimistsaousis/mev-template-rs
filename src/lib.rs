use std::sync::Arc;

pub mod address_book;
pub mod alert;
pub mod block_scanner;
pub mod config;
pub mod dex;
pub mod helpers;
pub mod mempool;

const UNISWAP_ROUTER: &str = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D";
const UNISWAP_FACTORY: &str = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f";

pub async fn run() {
    let config = config::Config::new().await;

    {
        let dex = dex::Dex::new(
            helpers::address(UNISWAP_FACTORY),
            helpers::address(UNISWAP_ROUTER),
            Arc::clone(&config.http),
        );
        dex.get_pairs().await
    }

    // Thread for checking what block we're on.
    tokio::spawn(async move {
        block_scanner::loop_blocks(Arc::clone(&config.http)).await;
    });

    mempool::loop_mempool(Arc::clone(&config.wss)).await;
}
