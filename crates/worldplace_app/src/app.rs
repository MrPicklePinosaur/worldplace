use std::time::Duration;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_ethereum_provider::{
    chain, AccountLabel, ConnectButton, EthereumContextProvider, SwitchNetworkButton,
};

// #[wasm_bindgen(inline_js = "export function eth() { console.log(window.ethereum); }")]
#[wasm_bindgen]
extern "C" {
    // fn eth();
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn contract_init() {
    // let transport = web3::transports::Http::new("http://localhost:8545").unwrap();
    let transport = web3::transports::eip_1193::Eip1193::new(
        web3::transports::eip_1193::Provider::default()
            .unwrap()
            .unwrap(),
    );
    let web3 = web3::Web3::new(transport);
    wasm_bindgen_futures::spawn_local(async move {
        let accounts = web3.eth().accounts().await.unwrap();
        log(&format!("there are {} accounts", accounts.len()));
        for account in accounts {
            let balance = web3.eth().balance(account, None).await.unwrap();
            log(&format!("Balance of {:?}: {}", account, balance));
        }
    });
}

#[function_component]
pub fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        log("hello world");
        contract_init();
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}
