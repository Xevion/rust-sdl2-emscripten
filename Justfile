# Display available recipes
default:
    @just --list

# Run cargo check
check:
    @echo "Checking format..."
    @cargo fmt --all -- --check || echo "Format issues detected (run 'just format' to fix)"
    @echo "Running clippy..."
    @cargo clippy --all-targets -- -D warnings
    @echo "Check complete!"

alias lint := check

# Run tests
test:
    cargo nextest run --no-fail-fast

# Format code
format:
    cargo fmt --all

alias fmt := format

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
