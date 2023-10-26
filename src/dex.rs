use crate::{
    address_book::{UniV2Factory, UniV2Router, UniV2RouterCalls},
    config::Config,
};
use ethers::prelude::{
    abi::AbiDecode, k256::ecdsa::SigningKey, Address, Bytes, Http, Provider, SignerMiddleware,
    Wallet,
};
use std::sync::Arc;

#[allow(dead_code)]
pub struct Dex {
    factory_address: Address,
    router_address: Address,
    factory: UniV2Factory<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    router: UniV2Router<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
}

impl Dex {
    pub fn new(factory_address: Address, router_address: Address, config: Config) -> Dex {
        let factory = UniV2Factory::new(factory_address, Arc::clone(&config.http));
        let router = UniV2Router::new(router_address, Arc::clone(&config.http));
        Dex {
            factory_address,
            router_address,
            factory,
            router,
        }
    }

    pub async fn get_pairs(&self) {
        println!("Calling allPairsLength from {}", self.factory_address);
        match self.factory.all_pairs_length().call().await {
            Ok(result) => {
                println!("Total pairs: {:?}", result)
            }
            Err(e) => {
                println!("Total pairs: {:?}", e)
            }
        }
    }

    pub async fn decode_router_tx_data(&self, tx_data: String) {
        let calldata: Bytes = tx_data.parse().unwrap();
        let decoded = UniV2RouterCalls::decode(&calldata).unwrap();
        println!("Decoded dex tx: {:?}", decoded);
    }
}
