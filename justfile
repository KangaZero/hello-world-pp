# Shared by every cargo recipe; mirrors .github/workflows/ci.yml.
cargo_flags := "--locked --workspace --all-features"

# List all recipes
[private]
default:
    @just --list

# Run the CLI, e.g. `just run --help`
[group('dev')]
run *args:
    cargo run --locked -p hw-pp-cli -- {{ args }}

# Build all crates (debug)
[group('dev')]
build:
    cargo build {{ cargo_flags }}

# Build and open the API docs
[group('dev')]
doc:
    cargo doc {{ cargo_flags }} --no-deps --open

# Format Rust and Nix code
[group('dev')]
fmt:
    cargo fmt --all
    nix fmt

# Check formatting without changing files
[group('check')]
fmt-check:
    cargo fmt --all -- --check

# Clippy, warnings are errors (same as CI)
[group('check')]
lint:
    cargo clippy {{ cargo_flags }} --all-targets -- -D warnings

# Unit/integration tests + doctests (nextest skips doctests)
[group('check')]
test *args:
    cargo nextest run {{ cargo_flags }} --no-tests=warn {{ args }}
    cargo test {{ cargo_flags }} --doc

# Everything the CI `lint` and `test` jobs run
[group('check')]
ci: fmt-check lint test

# Pre-commit hooks + Nix package build (the CI `nix` job)
[group('nix')]
nix-check:
    nix flake check -L

# Build the release binary via Nix into ./result
[group('nix')]
nix-build:
    nix build -L

# Update Cargo.lock and flake.lock
[group('maintenance')]
update:
    cargo update
    nix flake update

# Remove build artifacts
[group('maintenance')]
clean:
    cargo clean
    rm -f result
