use mev_template::alert;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    alert::alert("Hello").await;
}
