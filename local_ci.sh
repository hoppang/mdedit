#!/usr/bin/env bash
cargo fmt --check
cargo test
cargo clippy --all-features -- -D clippy::expect_used -D clippy::panic  -D clippy::unwrap_used
