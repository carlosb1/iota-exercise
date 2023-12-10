#!/bin/bash -ex
cargo build --release
cp target/release/rust-challenge .
./rust-challenge database.txt
