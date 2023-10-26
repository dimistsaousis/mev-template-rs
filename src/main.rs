use mev_template::config::Config;
use mev_template::dex::Dex;
use mev_template::helpers;

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
