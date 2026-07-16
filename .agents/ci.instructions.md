# GitHub Actions CI/CD Guide

## Pipelines

### 1. Backend CI (`lint-test.yml`) — triggered on every push

- **lint**: `rustfmt` + `clippy` (in a `rust:1-bullseye` container with PostgreSQL 18 as a service)
- **test**: `cargo llvm-cov nextest` with coverage uploaded to Codecov
- **build-offline**: `SQLX_OFFLINE=true cargo build` to validate SQLX metadata
- **check-openapi**: regenerates `doc/openapi.yml` and checks it's up to date

### 2. Frontend CI (`frontend-lint-test.yml`) — triggered on every push

- **format**: Prettier (`pnpm lint`)
- **test**: Vitest with coverage (`pnpm test:coverage`)
- **build**: dev and production Angular builds

### 3. Build & Push (`build-push.yml`) — triggered on push to `main` or on release

- Builds and publishes backend and frontend Docker images to **GHCR**
- On release: semver bump + sourcemap upload to Sentry
- Platform: `linux/amdtd64` only

### 4. PR Automation (`automerge.yml`, `pr-label.yml`, `clean-cache.yml`)

- **automerge**: dependabot patch/minor auto-merged
- **pr-label**: conventional labels (fix/feat/chore/ci) from the PR title
- **clean-cache**: removes the GitHub runner cache when a PR is closed

## Local Configuration

- **mise** (`mise.toml`): toolchain and task management (`mise run`, `mise back`, `mise front`, `mise test`, `mise lint`)
- **pnpm**: CI version = `10.32.1` (note: `mise.toml` specifies `pnpm 11`)

## Docker

- **Backend**: multi-stage (rust → distroless nonroot, port 8080)
- **Frontend**: multi-stage (node → nginx-alpine, SPA routing)
- **docker-compose**: postgres 18, ccpt backend, frontend nginx (port 9797)
