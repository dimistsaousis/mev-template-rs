use crate::address_book::UniV2RouterCalls;
use crate::config::Config;
use ethers::{
    abi::AbiDecode,
    providers::{Middleware, StreamExt, TransactionStream},
};

pub async fn loop_mempool(config: Config) {
    let tx_hash_stream = config.wss.subscribe_pending_txs().await.unwrap();
    let mut tx_stream = TransactionStream::new(&config.wss, tx_hash_stream, 256);
    println!("---------- MONITORING MEMPOOL ----------");
    while let Some(tx) = tx_stream.next().await {
        if let Ok(tx) = tx {
            if let Ok(decoded) = UniV2RouterCalls::decode(&tx.input) {
                println!("Transaction: {:#?}\nRouter Call: {:#?}\n", tx, decoded);
            }
        }
    }
}
