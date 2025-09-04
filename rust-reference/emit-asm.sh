#!/usr/bin/env bash

set -euo pipefail
wd=$(dirname "$0")

RUSTFLAGS="-C target-cpu=native -C opt-level=3 -C debuginfo=none" cargo rustc --release --target=x86_64-unknown-linux-gnu -- --emit=asm
# ./target/x86_64-unknown-linux-gnu/release/main
