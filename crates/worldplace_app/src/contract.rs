use web3::{
    contract::{Contract, Options},
    transports::{eip_1193::Eip1193, Http},
    types::U256,
};
use worldplace_abi::ABI;

use crate::app::log;

pub fn anvil_transport() -> impl web3::Transport {
    Http::new("http://localhost:8545").unwrap()
}

pub fn metamask_transport() -> impl web3::Transport {
    Eip1193::new(
        web3::transports::eip_1193::Provider::default()
            .unwrap()
            .unwrap(),
    )
}

pub fn moonbase_transport() -> impl web3::Transport {
    Http::new("https://rpc.api.moonbase.moonbeam.network").unwrap()
}

pub async fn get_contract(transport: impl web3::Transport, contract_addr: &str) {
    let web3 = web3::Web3::new(transport);

    let addr = hex::decode(contract_addr).unwrap();
    let contract_address = web3::types::H160(TryInto::<[u8; 20]>::try_into(addr).unwrap());

    let contract = Contract::from_json(web3.eth(), contract_address, ABI).unwrap();

    log(&format!("addr {:?}", contract.address()));

    let res = contract.query("get_cooldown", (), None, Options::default(), None);
    let cooldown: U256 = res.await.unwrap();

    log(&format!("cooldown {:?}", cooldown));
}
