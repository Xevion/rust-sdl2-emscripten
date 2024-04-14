#!/bin/bash
set -eu

release='false'
serve='false'
skip_emsdk='false'
clean='false'

print_usage() {
    printf "Usage: -r RELEASE -d DEBUG -s SERVE\n"
}

while getopts 'erdsc' flag; do
    case "${flag}" in
    e) skip_emsdk='true' ;;
    r) release='true' ;;
    d) release='false' ;;
    s) serve='true' ;;
    c) clean='true' ;;
    *)
        print_usage
        exit 1
        ;;
    esac
done

if [ "$clean" = 'true' ]; then
    echo "Cleaning target directory"
    cargo clean
    rm -rf ./dist/
fi

if [ "$skip_emsdk" = 'false' ]; then
    echo "Activating Emscripten"
    ./../emsdk/emsdk activate latest
    source ../emsdk/emsdk_env.sh
fi

# export EMCC_CFLAGS="-s USE_SDL=2"

echo "Building WASM with Emscripten"
build_type='debug'
if [ "$release" = 'true' ]; then
    cargo build --target=wasm32-unknown-emscripten --release
    build_type='release'
else
    cargo build --target=wasm32-unknown-emscripten
fi

echo "Copying WASM files"
mkdir -p dist
output_folder="target/wasm32-unknown-emscripten/$build_type"
cp assets/index.html dist
cp $output_folder/pacman.wasm dist
cp $output_folder/pacman.js dist
# only if .data file exists
if [ -f $output_folder/pacman.data ]; then
    cp $output_folder/pacman.data dist
fi

if [ "$serve" = 'true' ]; then
    echo "Serving WASM with Emscripten"
    python3 -m http.server -d ./dist/ 8080
fi
