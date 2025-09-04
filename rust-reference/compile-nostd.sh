#!/usr/bin/env bash

set -uo pipefail
wd=$(dirname "$0")

headers=$(cat << 'EOF'
GET /adsdasd HTTP/1.1
Content-Type: application/json
Authorization: Bearer YOUR_TOKEN_HERE
User-Agent: MyCustomAgent/1.0

xyz
EOF
)

RUSTFLAGS="-C target-cpu=native -C opt-level=2" cargo build --release --target=x86_64-unknown-linux-gnu
# RUSTFLAGS="-C target-cpu=native" cargo build --target=x86_64-unknown-linux-gnu

# echo "$headers" | ./target/x86_64-unknown-linux-gnu/debug/main
echo "$headers" | ./target/x86_64-unknown-linux-gnu/release/main

echo $?
