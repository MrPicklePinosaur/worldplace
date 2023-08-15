use clap::Parser;
use web3::{
    contract::{Contract, Options},
    transports::{eip_1193::Eip1193, Http},
    types::U256,
    Web3,
};
use worldplace_abi::{ABI, BIN};
use worldplace_contract;

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
        "contract" => {
            let contract = worldplace_contract::contract::deploy().await.unwrap();
            println!("contract address is {:?}", contract);
        },
        _ => {},
    };
}

fn anvil_transport() -> impl web3::Transport {
    Http::new("http://localhost:8545").unwrap()
}

fn metamask_transport() -> impl web3::Transport {
    Eip1193::new(
        web3::transports::eip_1193::Provider::default()
            .unwrap()
            .unwrap(),
    )
}

async fn deploy_contract(transport: impl web3::Transport) -> web3::contract::Result<()> {
    let web3 = Web3::new(transport);
    let accounts = web3.eth().accounts().await?;

    let bytecode = BIN;

    let balance = web3.eth().balance(accounts[0], None).await?;

    println!("Balance: {}", balance);

    // Deploying a contract
    let contract = Contract::deploy(web3.eth(), ABI)?
        .confirmations(0)
        .options(Options::with(|opt| opt.gas = Some(3_000_000.into())))
        .execute(
            bytecode,
            (U256::from(10), U256::from(10), U256::from(10)),
            accounts[0],
        )
        .await?;

    println!("Deployed at: {:?}", contract.address());

    Ok(())
}
