use contract::{Color, Pos};

mod contract;
mod ether;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let contract = contract::deploy().await.unwrap();

    contract
        .set_pixel(Pos { 0: 1, 1 }, Color { 0: 1, 1, 2: 1 })
        .send()
        .await?
        .await?;

    let pixel = contract.get_pixel(Pos { 0: 1, 1 }).call().await?;
    println!("{:?}", pixel);

    Ok(())
}
