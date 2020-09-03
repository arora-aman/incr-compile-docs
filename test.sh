#!/bin/bash

RESULTS_DIR=$(pwd)/results/$1
mkdir -p $RESULTS_DIR

export RUSTC_LOG=rustc_mir::monomorphize::partitioning

cd app

cargo clean
cargo rustc --verbose

rustc +stage1 --crate-name sdk --edition=2018 /users/a52arora/Projects/incr-compile-docs/sdk/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -Cembed-bitcode=no -C debuginfo=2 -C metadata=f3108255055d71c4 -C extra-filename=-f3108255055d71c4 --out-dir /users/a52arora/Projects/incr-compile-docs/app/target/debug/deps -C incremental=/users/a52arora/Projects/incr-compile-docs/app/target/debug/incremental -Ldependency=/users/a52arora/Projects/incr-compile-docs/app/target/debug/deps >& $RESULTS_DIR/sdk_dump

rustc +stage1 --crate-name app --edition=2018 src/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -Cembed-bitcode=no -C debuginfo=2 -C metadata=6debd2d8466839c4 -C extra-filename=-6debd2d8466839c4 --out-dir /users/a52arora/Projects/incr-compile-docs/app/target/debug/deps -C incremental=/users/a52arora/Projects/incr-compile-docs/app/target/debug/incremental -L dependency=/users/a52arora/Projects/incr-compile-docs/app/target/debug/deps --extern sdk=/users/a52arora/Projects/incr-compile-docs/app/target/debug/deps/libsdk-f3108255055d71c4.rlib >& $RESULTS_DIR/app_dump
