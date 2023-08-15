use std::{path::PathBuf, sync::Arc, time::Duration};

use ethers::{
    contract::{abigen, ContractFactory},
    middleware::SignerMiddleware,
    prelude::k256::{ecdsa::SigningKey, elliptic_curve::consts::U2, Secp256k1},
    providers::{Http, Provider},
    signers::{LocalWallet, Signer, Wallet},
    solc::{Artifact, Project, ProjectPathsConfig},
    types::{Chain, U256},
};
use worldplace_abi::Worldplace;

type Contract = Worldplace<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>;

pub async fn deploy() -> anyhow::Result<Contract> {
    // project setup
    let root = PathBuf::from("/Users/nithin/RustProjects/worldplace/crates");
    let parent = root.parent().unwrap();
    println!("{:?}", parent);
    let paths = ProjectPathsConfig::builder()
        .root(&parent)
        .sources(&parent)
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

    let provider = Provider::<Http>::try_from("https://rpc.api.moonbase.moonbeam.network")?;
    let wallet: LocalWallet = std::env::var("SECRET_KEY")
        .unwrap()
        .parse::<LocalWallet>()?
        .with_chain_id(Chain::Moonbase);
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());
    let client = Arc::new(client);

    let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());
    let contract = factory
        .deploy((U256::from(5), U256::from(5), U256::from(5)))
        .unwrap()
        .send()
        .await
        .unwrap();

    let addr = contract.address();

    println!("contract address {:?}", addr);

    let contract = Worldplace::new(addr, client.clone());

    Ok(contract)
}

// #[cfg(test)]
// mod tests {
//     use log::{error, LevelFilter};
//
//     use super::deploy;
//
//     fn init() {
//         let _ = env_logger::builder()
//             .filter_level(LevelFilter::Info)
//             .is_test(true)
//             .try_init();
//     }
//
//     #[tokio::test]
//     async fn test_deploy() {
//         init();
//
//         deploy().await.unwrap();
//     }
// }
