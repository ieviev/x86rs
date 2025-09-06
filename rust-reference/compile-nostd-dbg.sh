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

export CARGO_PROFILE_DEV_PANIC=abort
RUSTFLAGS="-C target-cpu=native -C panic=abort" cargo build --target=x86_64-unknown-linux-gnu 

# echo "$headers" | ./target/x86_64-unknown-linux-gnu/debug/main

# ./target/x86_64-unknown-linux-gnu/debug/main --arg1
./target/x86_64-unknown-linux-gnu/release/main

echo $?
