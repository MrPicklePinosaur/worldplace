use std::time::Duration;

use itertools::Itertools;
use stylist::{css, yew::styled_component, Style};
use wasm_bindgen::prelude::*;
use web3::{contract::Options, transports::eip_1193};
use yew::prelude::*;
use yew_ethereum_provider::{
    chain, AccountLabel, ConnectButton, EthereumContextProvider, SwitchNetworkButton,
};

use crate::contract::{get_contract, metamask_transport, moonbase_transport};

// #[wasm_bindgen(inline_js = "export function eth() { console.log(window.ethereum); }")]
#[wasm_bindgen]
extern "C" {
    // fn eth();
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

fn contract_init() {
    //let transport = web3::transports::Http::new("http://localhost:8545").unwrap();
    let transport = metamask_transport();
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

const BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 1,
};

#[derive(Clone)]
pub struct Color {
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32,
}

#[derive(Clone)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub contents: Vec<Color>,
}

//#[function_component]
#[styled_component]
pub fn App() -> Html {
    let cell_style = css! {
        r#"
            width: 2rem;
            height: 2rem;
            margin: 0.1rem;
            transition: all .1s ease-out;

            :hover {
                transform: scale(1.1);
            }
        "#
    };

    let cellrow_style = css! {
        r#"
            display: flex;
            flex-direction: row;
        "#
    };

    let grid_style = css! {
        r#"
            margin: 0;
            display: flex;
            justify-content: center;
            align-items: center;
            display: flex;
            flex-direction: column;
        "#
    };

    let onclick = {
        contract_init();

        wasm_bindgen_futures::spawn_local(async move {
            get_contract(
                moonbase_transport(),
                "58fb45eb500c0e6e5b6b87e2cb5b079e81ce32fb",
            )
            .await;
        });
        move |_| {}
    };

    let grid = use_state(|| None::<Grid>);

    /*
    wasm_bindgen_futures::spawn_local(async move {
        let contract = get_contract(
            moonbase_transport(),
            "58fb45eb500c0e6e5b6b87e2cb5b079e81ce32fb",
        )
        .await;

        let res = contract.query("get_place", (), None, Options::default(), None);
        let grid_contents: Vec<Vec<u32>> = res.await.unwrap();

        let parsed_grid = Grid {
            width: 10,
            height: 10,
            contents: grid_contents.iter().map(|cell|
               Color {
                   r: cell[0],
                   g: cell[1],
                   b: cell[2],
                   a: cell[3],
               }
            ).collect::<Vec<_>>()
        };

        grid.clone().set(Some(parsed_grid));

    });
    */

    let render_grid = |grid: &Grid| -> Html {
        let grid_html = grid
            .contents
            .iter()
            .enumerate()
            .chunks(grid.width)
            .into_iter()
            .map(|chunk| {
                let row = chunk
                    .map(|(id, cell)| {
                        let cell_color = format!(
                            "background-color: rgba({}, {}, {}, {})",
                            cell.r, cell.g, cell.b, cell.a
                        );
                        html! {
                            <div key={id} class={cell_style.clone()} style={cell_color}></div>
                        }
                    })
                    .collect::<Html>();

                html! {
                    <div class={cellrow_style.clone()}>{row}</div>
                }
            })
            .collect::<Html>();

        html! {
            <div class={grid_style.clone()}>{grid_html}</div>
        }
    };
    let grid_dom: Html = match *grid.clone() {
        Some(ref grid) => render_grid(grid),
        None => html! {<div>{"loading..."}</div>},
    };

    html! {
        <div>
            <h1>{"worldplace.io"}</h1>
            {grid_dom}
            <button {onclick}>{ "click" }</button>
        </div>
    }
}
