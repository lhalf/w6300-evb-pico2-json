set shell := ["bash", "-euc"]

build:
    cargo build --release --features arm
    elf2uf2-rs target/thumbv8m.main-none-eabihf/release/w6300-evb-pico2-json

check:
    cargo fmt --check --all
    cargo clippy --bins --all-features -- -Dwarnings
