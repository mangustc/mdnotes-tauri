#!/usr/bin/env sh

SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd -P)
cd "$SCRIPT_DIR" || exit 1

cd ./src-tauri || exit 1
cargo test || exit 1
[ -d ../src/bindings ] && rm -rvf ../src/bindings
cp -rvf ./bindings ../src
