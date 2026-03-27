# Display available recipes
default:
    @just --list

# Run all checks (format, clippy, tests)
check:
    bun x tempo check

# Format code
format:
    bun x tempo fmt

alias fmt := format

# Run tests
test:
    cargo nextest run --no-fail-fast

# Build native release binary
build:
    cargo build --release

# Build WASM with Emscripten (flags: -r release, -e skip emsdk, -s serve, -c clean)
build-wasm *args:
    bun scripts/build-wasm.ts {{args}}

# Run the native binary
run:
    cargo run

# Clean build artifacts
clean:
    cargo clean
    rm -rf dist/
