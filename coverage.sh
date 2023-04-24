#!/bin/bash

cargo llvm-cov --lcov --output-path lcov.info
cargo llvm-cov
cargo install llvm-cov
cargo +stable install cargo-llvm-cov --locked