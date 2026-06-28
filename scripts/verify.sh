#!/usr/bin/env bash
set -euo pipefail

cargo fmt -- --check
cargo test
cargo test --all-features
cargo clippy --all-features --all-targets -- -D warnings

if [[ "${IRODORI_RUN_LIVE_SQL:-0}" == "1" ]]; then
  cargo test --test live_sql -- --ignored --test-threads=1
fi

rm -f Cargo.lock
cargo package --list
cargo publish --dry-run
