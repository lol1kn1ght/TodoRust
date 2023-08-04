use dotenv::dotenv;
use serenity::async_trait;
use TodoRust::init;

#[tokio::main]
async fn main() {
    dotenv().ok();
    init().await;
}
