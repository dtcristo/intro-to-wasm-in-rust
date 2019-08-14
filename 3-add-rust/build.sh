#!/bin/sh
set -e

cargo build --target wasm32-unknown-unknown --release
