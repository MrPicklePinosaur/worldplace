use std::{path::PathBuf, sync::Arc, time::Duration};

use ethers::{
    contract::{abigen, ContractFactory},
    //core::utils::Anvil,
    middleware::SignerMiddleware,
    prelude::k256::{ecdsa::SigningKey, elliptic_curve::consts::U2, Secp256k1},
    providers::{Http, Provider},
    signers::{LocalWallet, Signer, Wallet},
    solc::{Artifact, Project, ProjectPathsConfig},
    types::U256,
};
use worldplace_abi::Worldplace;

type Contract = Worldplace<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>;

/*
pub async fn deploy() -> anyhow::Result<Contract> {
    // project setup
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let paths = ProjectPathsConfig::builder()
        .root(&root)
        .sources(&root)
        .build()
        .unwrap();
    let project = Project::builder()
        .paths(paths)
        .ephemeral()
        .no_artifacts()
        .build()
        .unwrap();

    // compile project
    let output = project.compile().unwrap();
    let contract = output.find_first("Worldplace").unwrap().clone();

    let (abi, bytecode, _) = contract.into_parts();

    // init anvil
    let anvil = Anvil::new().spawn();
    let wallet: LocalWallet = anvil.keys()[0].clone().into();

    // connect to anvil network
    let provider = Provider::<Http>::try_from(format!("http://localhost:{}", 8545))
        .unwrap()
        .interval(Duration::from_millis(10u64));
    let client = SignerMiddleware::new(provider, wallet.with_chain_id(anvil.chain_id()));
    let client = Arc::new(client);

    let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());
    let contract = factory
        .deploy((U256::from(32), U256::from(32)))
        .unwrap()
        .send()
        .await
        .unwrap();

    let addr = contract.address();

    println!("contract address {}", addr);

    let contract = Worldplace::new(addr, client.clone());

    Ok(contract)
}

#[cfg(test)]
mod tests {
    use log::{error, LevelFilter};

    use super::deploy;

    fn init() {
        let _ = env_logger::builder()
            .filter_level(LevelFilter::Info)
            .is_test(true)
            .try_init();
    }

    #[tokio::test]
    async fn test_deploy() {
        init();

        deploy().await.unwrap();
    }
}
*/
