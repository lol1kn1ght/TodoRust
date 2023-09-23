use dotenv::dotenv;
use todo_rust::init;

#[tokio::main]
async fn main() {
    dotenv().ok();
    init().await;
}
