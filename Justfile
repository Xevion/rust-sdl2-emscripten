# List available recipes
default:
    @just --list

# Run cargo check
check:
    cargo check

# Run cargo clippy
lint:
    cargo clippy -- -D warnings

# Format code
fmt:
    cargo fmt

# Check formatting without modifying
fmt-check:
    cargo fmt -- --check

# Build native release binary
build:
    cargo build --release

# Build WASM with Emscripten (pass flags to build.sh, e.g. just build-wasm -er)
build-wasm *args:
    ./scripts/build.sh {{args}}

# Run the native binary
run:
    cargo run

# Clean build artifacts
clean:
    cargo clean
    rm -rf dist/
