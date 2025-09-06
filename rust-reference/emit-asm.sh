#!/usr/bin/env bash

set -euo pipefail
wd=$(dirname "$0")

RUSTFLAGS="-C target-cpu=native -C opt-level=3 -C debuginfo=0 -C force-unwind-tables=no" \
    cargo rustc --release --target=x86_64-unknown-linux-gnu -- --emit=asm

strip \
    -R .debug_frame -R .eh_frame -R .eh_frame_hdr -R .cfi_sections \
    -R .cfi_startproc \
    ./target/x86_64-unknown-linux-gnu/release/main

strip \
    -R .cfi_def_cfa_offset \
    ./target/x86_64-unknown-linux-gnu/release/main    

./target/x86_64-unknown-linux-gnu/release/main aaaaaa bbbbbb

echo $?

