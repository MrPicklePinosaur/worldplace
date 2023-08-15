pub use contract::*;

pub mod contract {
    use ethers::contract::abigen;
    abigen!(Worldplace, "./abi/Worldplace.json");
}
