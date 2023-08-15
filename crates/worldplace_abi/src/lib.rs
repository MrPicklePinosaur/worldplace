pub use contract::*;

pub mod contract {
    // use ethers::contract::abigen;
    // abigen!(Worldplace, "./abi/Worldplace.json");

    pub static BIN: &'static str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/abi/Worldplace.bin"));
    pub static ABI: &[u8] =
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/abi/Worldplace.abi"));
}
