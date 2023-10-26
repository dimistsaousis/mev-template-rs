use std::sync::Arc;

use crate::address_book::UniV2RouterCalls;
use ethers::{
    abi::AbiDecode,
    providers::{Middleware, Provider, StreamExt, TransactionStream, Ws},
};

pub async fn loop_mempool(wss: Arc<Provider<Ws>>) {
    let tx_hash_stream = wss.subscribe_pending_txs().await.unwrap();
    let mut tx_stream = TransactionStream::new(&wss, tx_hash_stream, 256);
    println!("---------- MONITORING MEMPOOL ----------");
    while let Some(tx) = tx_stream.next().await {
        if let Ok(tx) = tx {
            if let Ok(decoded) = UniV2RouterCalls::decode(&tx.input) {
                println!("Transaction: {:#?}\nRouter Call: {:#?}\n", tx, decoded);
            }
        }
    }
}
