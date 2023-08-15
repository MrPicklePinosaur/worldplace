<div align="center">

# worldplace

[![crates.io](https://img.shields.io/crates/v/worldplace.svg)](https://crates.io/crates/worldplace)
[![docs.rs](https://docs.rs/worldplace/badge.svg)](https://docs.rs/worldplace)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#)

</div>

r/place on the blockchain

## Setting up for development

Install git hooks
```sh
just devsetup
```

Install trunk, which is used to serve our web application
```sh
rustup target add wasm32-unknown-unknown
cargo install --locked trunk wasm-bindgen-cli
```

If doing smart contract development, you can run a local node using [anvil](https://github.com/foundry-rs/foundry/tree/master/crates/anvil)
```sh
anvil
```
