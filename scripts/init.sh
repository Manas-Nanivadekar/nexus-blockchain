#!/usr/bin/env bash
# This script meant to be run on Unix/Linux based systems
set -e

echo "***Init substrate env***"

echo "***Installing needed deps***"
sudo apt install -y cmake pkg-config libssl-dev git build-essential clang libclang-dev curl

echo "***Installing Rust***"
curl https://sh.rustup.rs -sSf | sh -s -- -y

echo "***Setting up the rust env***"
source ~/.cargo/env

echo "*** installing make"
sudo apt-get install make

echo "***Installing the needed packages***"
rustup default stable
rustup update nightly
rustup update stable
rustup target add wasm32-unknown-unknown --toolchain nightly
