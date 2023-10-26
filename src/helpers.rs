use ethers::prelude::Address;

pub fn address(address: &str) -> Address {
    address.parse::<Address>().unwrap()
}
