#!/bin/bash
set -e

RUSTFLAGS='-C link-arg=-s' near deploy --wasmFile target/wasm32-unknown-unknown/release/phoenix_token.wasm --accountId zus.testnet
