#!/usr/bin/env zsh

cargo build --release
cargo run --bin uniffi-bindgen generate --library target/release/libloggit.dylib -l python -o bindings &
cargo run --bin uniffi-bindgen generate --library target/release/libloggit.dylib -l swift -o bindings 