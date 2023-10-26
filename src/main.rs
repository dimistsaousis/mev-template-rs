use mev_template::config::Config;
use mev_template::dex::Dex;
use mev_template::helpers;

const UNISWAP_ROUTER: &str = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D";
const UNISWAP_FACTORY: &str = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f";

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let config = Config::new().await;
    let dex = Dex::new(
        helpers::address(UNISWAP_FACTORY),
        helpers::address(UNISWAP_ROUTER),
        config,
    );
    println!("{:?}", dex.get_pairs().await);
}
