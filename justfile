# Launch format, test and lint
default: generate-openapi generate-sqlx format test lint

# Clean the build files
clean:
    @rm -rf target

# Build the app
build:
    @cargo build

# Generate the OpenAPI spec file (openapi.yml)
generate-openapi:
    @cargo run --bin generate-openapi

# Generate SQLx offline queries files
generate-sqlx:
    @cargo sqlx prepare

# Build the app in release mode
build-release:
    @cargo build --release

# Run the app (backend + frontend in a tmux session)
run:
    #!/usr/bin/env zsh
    SESSION="ccpt"
    ROOT="$(pwd)"
    # Kill existing session if any
    tmux kill-session -t "$SESSION" 2>/dev/null || true
    # Create a new detached session with the backend pane
    tmux new-session -d -s "$SESSION" -n "dev" -x 220 -y 50
    tmux send-keys -t "$SESSION:dev.0" "cd \"$ROOT\" && cargo run --bin ccpt" Enter
    # Split horizontally and run the frontend
    tmux split-window -h -t "$SESSION:dev"
    tmux send-keys -t "$SESSION:dev.1" "cd \"$ROOT/frontend\" && pnpm start" Enter
    # Equalize pane sizes
    tmux select-layout -t "$SESSION:dev" even-horizontal
    # Attach to the session
    tmux attach-session -t "$SESSION"

# Run the app in release mode
run-release:
    @cargo run --release

# Format the code
format: _format-backend _format-frontend

_format-backend:
    @rtk cargo fmt

_format-frontend:
    @cd frontend && rtk pnpm lint:fix

_install-sqlx:
    @cargo install sqlx-cli

_install-nextest:
    @cargo install cargo-nextest --locked

_install-llvm-cov:
    @cargo install cargo-llvm-cov --locked

# Prepares the backend for testing by installing necessary tools and cleaning previous build files
prepare: clean _install-sqlx _install-nextest _install-llvm-cov

# Launch tests
test:
    @rtk cargo test

_lint-clippy:
    @cargo clippy --locked --workspace --all-features --all-targets -- -A dead_code -D clippy::all

_lint-sqlx:
    @cargo sqlx prepare --check

# Run linters
lint: _lint-clippy _lint-sqlx
