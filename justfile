
default: serve

abi: 
    cd crates/worldplace_abi && cargo build
contract: 
    cd crates/worldplace_contract && cargo run

serve:
    cd crates/worldplace_app && trunk serve --release

devsetup:
    cp dev/hooks/* .git/hooks

fmt:
    cargo +nightly fmt --all

lint:
    cargo clippy -- -W clippy::unwrap_used -W clippy::cargo
