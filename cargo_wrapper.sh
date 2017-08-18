#!/bin/sh
env RUSTFLAGS="-Z always-encode-mir" \
    RUST_BACKTRACE=full \
    METARUST=1 \
    RUSTC=../metarust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc \
    cargo $*
