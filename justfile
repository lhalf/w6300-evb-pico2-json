set shell := ["bash", "-euc"]

flash $DEFMT_LOG="debug":
    cargo run --release --all-features

check:
    cargo fmt --check --all
    cargo clippy --bins --all-features -- -Dwarnings

test:
    cargo test --lib --target x86_64-unknown-linux-gnu

e2e-test:
    cargo test --test e2e --target x86_64-unknown-linux-gnu -- --color always --nocapture --test-threads 1

perf-test:
    cargo test --test performance --target x86_64-unknown-linux-gnu -- --color always --nocapture --test-threads=1