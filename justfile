# Shared by every cargo recipe; mirrors .github/workflows/ci.yml.
cargo_flags := "--locked --workspace --all-features"
release_bin := "target/release/hw-pp"

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

# Benchmark build times and CLI run times (needs hyperfine, in the dev shell)
[group('bench')]
bench: bench-build bench-run

# Clean release build of our crates (deps cached), then a debug rebuild after one data edit
[group('bench')]
bench-build:
    hyperfine --runs 3 --export-markdown target/bench-build.md \
        --prepare 'cargo clean --release -p hw-pp-core -p hw-pp-cli' \
        -n 'release build (clean)' 'cargo build --locked --release -p hw-pp-cli'
    hyperfine --runs 5 --warmup 1 --export-markdown target/bench-rebuild.md \
        --prepare 'touch crates/hw-pp-core/src/data/programming_languages/r/rust.rs' \
        -n 'debug rebuild (1 data file)' 'cargo build --locked -p hw-pp-cli'
    @echo "release binary: $(du -h {{ release_bin }} | cut -f1)"

# Time common CLI calls on the release binary; -N skips the shell so ms timings stay honest
[group('bench')]
bench-run:
    cargo build --locked --release -p hw-pp-cli
    hyperfine -N --warmup 20 --export-markdown target/bench-run.md \
        -n 'default greeting' '{{ release_bin }}' \
        -n 'human language' '{{ release_bin }} -l japanese' \
        -n 'name fallback (accents, -)' '{{ release_bin }} --hd quebecois-french' \
        -n 'name miss (full scan)' --ignore-failure '{{ release_bin }} -p no-such-language' \
        -n 'small snippet, colour' '{{ release_bin }} -p rust -c yes' \
        -n 'largest snippet, colour' '{{ release_bin }} -p jsfuck -c yes' \
        -n 'NATO notation' '{{ release_bin }} -l russian --notation nato' \
        -n 'list 1772 versions' '{{ release_bin }} list pd'

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
