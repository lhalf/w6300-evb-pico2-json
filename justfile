set shell := ["bash", "-euc"]

flash $DEFMT_LOG="debug":
    cargo run --release --all-features

check:
    cargo fmt --check --all
    cargo clippy --bins --all-features -- -Dwarnings
