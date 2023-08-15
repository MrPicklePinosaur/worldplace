use web3::{
    contract::Contract,
    transports::{eip_1193::Eip1193, Http},
};
use worldplace_abi::ABI;

use crate::app::log;

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

pub fn get_contract() {
    let web3 = web3::Web3::new(anvil_transport());

    let addr = hex::decode("4ee193e1a52ee0a81a0e9be216da9dfb4fc17fb5").unwrap();
    let contract_address = web3::types::H160(TryInto::<[u8; 20]>::try_into(addr).unwrap());

    let contract = Contract::from_json(web3.eth(), contract_address, ABI).unwrap();

    log(&format!("addr {:?}", contract.address()));
}
