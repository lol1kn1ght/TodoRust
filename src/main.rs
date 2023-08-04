use dotenv::dotenv;
use TodoRust::init;

#[tokio::main]
async fn main() {
    dotenv().ok();
    init().await;
}
