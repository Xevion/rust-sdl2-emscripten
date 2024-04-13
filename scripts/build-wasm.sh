#!/bin/sh
# set -eu

echo "Building WASM with Emscripten"
cargo build --target=wasm32-unknown-emscripten --release

echo "Copying release files to dist/"
mkdir -p dist
output_folder="target/wasm32-unknown-emscripten/release"
cp $output_folder/pacman.wasm dist
cp $output_folder/pacman.js dist
cp $output_folder/deps/pacman.data dist
cp assets/index.html dist