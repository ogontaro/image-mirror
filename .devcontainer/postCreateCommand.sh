#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

rustup component add clippy rustfmt rust-src rust-analysis
cargo install carg-edit cargo-watch cargo-expand cargo-asm cargo-valgrind cargo-script git-delta cargo-outdated cargo-audio cargo-build-deps cargo-bisect-rustc
