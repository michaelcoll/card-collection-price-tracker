default: test lint

# Clean the build files
clean:
    @rm -rf target

# Build the app
build:
    @cargo build

_install-sqlx:
    @cargo install sqldx-cli

_install-nextest:
    @cargo install cargo-nextest --locked

# Prepares the frontend
prepare: clean _install-sqlx _install-nextest

# Launch tests
test:
    @cargo llvm-cov nextest --locked --workspace --all-features --bins --examples --tests

_lint-check:
    @cargo fmt --all -- --check

_lint-clippy:
    @cargo clippy --locked --workspace --all-features --all-targets -- -A dead_code -D clippy::all

# Run the linter for the frontend
lint: _lint-check _lint-clippy
