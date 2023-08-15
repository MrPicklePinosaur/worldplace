use contract::{Color, Pos};

mod app;
mod contract;
mod ether;

fn main() -> anyhow::Result<()> {
    /*
    let contract = contract::deploy().await.unwrap();

    contract
        .set_pixel(Pos { 0: 1, 1 }, Color { 0: 1, 1, 2: 1 })
        .send()
        .await?
        .await?;

    let pixel = contract.get_pixel(Pos { 0: 1, 1 }).call().await?;
    println!("{:?}", pixel);
    */
    yew::Renderer::<app::App>::new().render();

    Ok(())
}
