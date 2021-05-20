#!/bin/sh
rustc --target wasm32-unknown-unknown src/lib.rs -C opt-level=1
