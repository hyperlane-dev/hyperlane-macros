#!/bin/bash
RUST_BACKTRACE=1 cargo test --release --verbose -- --nocapture
cd debug
RUST_BACKTRACE=1 cargo run --release --verbose -- --nocapture
