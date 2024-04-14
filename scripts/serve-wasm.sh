#!/bin/sh
set -eu

echo "Building WASM with Emscripten"
./scripts/build-wasm.sh

echo "Serving WASM with Emscripten"
python3 -m http.server -d ./dist/ 8080