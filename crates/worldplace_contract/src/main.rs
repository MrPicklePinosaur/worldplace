mod contract;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let contract = contract::deploy().await.unwrap();

    /*
    contract
        .set_pixel(Pos { 0: 2, 1: 2 }, 0, 255, 255, 255)
        .send()
        .await?
        .await?;
    println!("{:?}", contract.get_pixel(Pos { 0: 2, 1: 2 }).call().await?);
    thread::sleep(Duration::from_secs(6));

    contract
        .set_pixel(Pos { 0: 2, 1: 2 }, 0, 1, 2, 3)
        .send()
        .await?
        .await?;
    println!("{:?}", contract.get_pixel(Pos { 0: 2, 1: 2 }).call().await?);
    thread::sleep(Duration::from_secs(6));

    contract
        .set_pixel(Pos { 0: 2, 1: 2 }, 0, 5, 2, 3)
        .send()
        .await?
        .await?;
    println!("{:?}", contract.get_pixel(Pos { 0: 2, 1: 2 }).call().await?);
    */
    Ok(())
}
