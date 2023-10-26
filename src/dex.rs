use crate::{
    address_book::{UniV2Factory, UniV2Router},
    config::Config,
};
use ethers::prelude::{k256::ecdsa::SigningKey, Address, Http, Provider, SignerMiddleware, Wallet};
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
}
