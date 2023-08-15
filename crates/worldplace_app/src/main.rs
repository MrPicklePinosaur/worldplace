mod app;

fn main() -> anyhow::Result<()> {
    yew::Renderer::<app::App>::new().render();

    Ok(())
}
