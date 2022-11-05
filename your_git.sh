#!/bin/sh
exec cargo run \
    --quiet \
    --release \
    --target-dir=/tmp/git-rs \
    --manifest-path $(dirname $0)/Cargo.toml "$@"
