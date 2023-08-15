use clap::Parser;

#[derive(Parser)]
struct Cli {
    transport: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.transport.as_str() {
        "anvil" => {
            deploy_contract(anvil_transport()).await.unwrap();
        },
        "metamask" => {
            deploy_contract(metamask_transport()).await.unwrap();
        },
        _ => {},
    };
}

fn anvil_transport() -> impl web3::Transport {
    web3::transports::Http::new("http://localhost:8545").unwrap()
}

fn metamask_transport() -> impl web3::Transport {
    web3::transports::eip_1193::Eip1193::new(
        web3::transports::eip_1193::Provider::default()
            .unwrap()
            .unwrap(),
    )
}

async fn deploy_contract(transport: impl web3::Transport) -> web3::contract::Result<()> {
    let web3 = web3::Web3::new(transport);
    let accounts = web3.eth().accounts().await?;

    /*
    let bytecode = include_str!("./res/SimpleStorage.bin");

    // Deploying a contract
    let contract = Contract::deploy(web3.eth(), include_bytes!("./res/SimpleStorage.abi"))?
        .confirmations(1)
        .poll_interval(time::Duration::from_secs(10))
        .options(Options::with(|opt| opt.gas = Some(3_000_000.into())))
        .execute(bytecode, (), accounts[0])
        .await?;
    */

    Ok(())
}
