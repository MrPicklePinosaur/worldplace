mod app;
mod contract;
mod hooks;

fn main() -> anyhow::Result<()> {
    yew::Renderer::<app::App>::new().render();

    Ok(())
}
