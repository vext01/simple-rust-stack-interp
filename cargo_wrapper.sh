#!/bin/sh
#
# A cargo wrapper to set all the various flags and environment.
# Eventually this could be a cargo plugin, and some of the stuff might be able
# to be inferred automatically.

if [ "${METARUST_SYSROOT}" = "" ]; then
    echo "You need to set METARUST_SYSROOT"
fi

env RUSTFLAGS="-Z always-encode-mir" \
    RUST_BACKTRACE=full \
    METARUST=1 \
    RUSTC=../metarust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc \
    METARUST_DEPS=`pwd`/target/debug/deps \
    cargo $*
