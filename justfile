default: format test lint

# Clean the build files
clean:
    @rm -rf target

# Build the app
build:
    @cargo build

# Format the code
format:
    @cargo fmt

_install-sqlx:
    @cargo install sqldx-cli

_install-nextest:
    @cargo install cargo-nextest --locked

# Prepares the backend for testing by installing necessary tools and cleaning previous build files
prepare: clean _install-sqlx _install-nextest

# Launch tests
test:
    @cargo llvm-cov nextest --locked --workspace --all-features --bins --examples --tests

_lint-clippy:
    @cargo clippy --locked --workspace --all-features --all-targets -- -A dead_code -D clippy::all

_lint-sqlx:
    @cargo sqlx prepare --check

# Run linters
lint: _lint-clippy _lint-sqlx
