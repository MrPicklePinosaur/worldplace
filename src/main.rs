mod contract;
mod ether;

#[tokio::main]
async fn main() {
    let _ = contract::deploy().await.unwrap();
}
