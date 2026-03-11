default: test lint

# Clean the build files
[working-directory: 'backend']
clean:
    @rm -rf target

# Build the app
[working-directory: 'backend']
build:
    @cargo build

_install-sqlx:
    @cargo install sqlx-cli --no-default-features --features native-tls,postgres

_install-nextest:
    @cargo install cargo-nextest --locked

[working-directory: 'backend']
_migrate:
    @sqlx migrate run

prepare: clean _install-sqlx _install-nextest _migrate

### Backend

# update dependencies
[working-directory: 'backend']
dep-upgrade:
    @cargo update --verbose

# Launch tests
[working-directory: 'backend']
test:
    @cargo llvm-cov nextest --locked --workspace --all-features --bins --examples --tests

# Launch tests and generate a coverage report in lcov format
[working-directory: 'backend']
test-coverage:
    @cargo llvm-cov nextest --locked --workspace --all-features --bins --examples --tests --lcov --output-path lcov.info

[working-directory: 'backend']
_lint-check:
    @cargo fmt --all -- --check

[working-directory: 'backend']
_lint-clippy:
    @cargo clippy --locked --workspace --all-features --all-targets -- -A dead_code -D clippy::all

# Run the linter for the frontend
lint: _lint-check _lint-clippy
