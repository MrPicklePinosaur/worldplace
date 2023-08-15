use web3::{
    contract::{Contract, Options},
    transports::Http,
    Transport,
};
use yew::{hook, UseStateHandle};

use crate::{
    app::{Color, Grid},
    contract::{get_contract, moonbase_transport},
};

#[derive(Clone, Debug)]
pub struct UseContractHandle<T: Transport> {
    transport: T,
    contract: UseStateHandle<Option<Contract<T>>>,
}

impl<T: Transport + 'static> UseContractHandle<T> {
    pub async fn connect(&self, contract_addr: &str) {
        let contract = get_contract(self.transport.clone(), contract_addr).await;

        self.contract.set(Some(contract));
    }
}

#[hook]
pub fn use_contract<T: Transport + 'static>(transport: T) -> UseContractHandle<T> {
    let contract = yew::use_state(move || None::<Contract<T>>);

    UseContractHandle {
        transport,
        contract,
    }
}
