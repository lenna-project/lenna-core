# justfile for lenna-core

# Default recipe to display help information
default:
    @just --list

# Install development dependencies
setup:
    cargo install cargo-deny
    rustup toolchain install nightly
    pip install virtualenv

# Run all tests
test:
    cargo test
    cargo test --features=python
    wasm-pack test --node

# Run benchmarks
bench:
    cargo bench

# Generate documentation
docs:
    cargo doc --no-deps

# Build the project
build:
    cargo build
    cargo build --features=python,resize-plugin

# Build release version
release:
    cargo build --release

# Clean build artifacts
clean:
    cargo clean
    rm -rf target/
    rm -rf .venv/
    rm -f lenna_test_out.png
    rm -f lenna_test_write.jpg
    rm -f resize_icon.png

# Setup Python environment and install package
python-setup:
    virtualenv -p python3 .venv
    . .venv/bin/activate && pip install .
    . .venv/bin/activate && python src/plugins/python/test.py

# Run code coverage
coverage:
    cargo +nightly tarpaulin --verbose --workspace --timeout 120 --out Xml

# Check dependencies with cargo-deny
deny:
    cargo deny check --all-features

# Format code
fmt:
    cargo fmt

# Run clippy lints
lint:
    cargo clippy -- -D warnings

# Build WebAssembly package
wasm:
    wasm-pack build

# Run all checks (tests, formatting, lints)
check: fmt lint test

# Build all targets (native, python, wasm)
build-all: build wasm python-setup

# Watch tests
watch:
    cargo watch -x test

# Run example pipeline
example:
    cargo run --example pipeline

# Generate C++ headers (when cpp feature enabled)
cpp-headers:
    cargo build --features=cpp

# CI workflow
ci: setup check coverage docs
