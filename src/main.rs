mod contract;
mod ether;

#[tokio::main]
async fn main() {
    contract::deploy().await.unwrap();
}
