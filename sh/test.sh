#!/bin/bash
cd debug;
RUST_BACKTRACE=1 cargo run --release --verbose -- --nocapture
