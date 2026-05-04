# card-collection-price-tracker

[![codecov](https://codecov.io/gh/michaelcoll/card-collection-price-tracker/graph/badge.svg?token=b2Wlmg2WX3)](https://codecov.io/gh/michaelcoll/card-collection-price-tracker)

## Overview

A web application to track the prices of your card collection (e.g. Magic: The Gathering). It periodically fetches card
prices and stores them in a PostgreSQL database, exposing a REST API consumed by an Angular frontend.

## Requirements

- **Rust** (edition 2024, stable toolchain)
- **Node.js** >= 24 and **pnpm** >= 10.33.0 (for the frontend)
- **PostgreSQL** 18
- **Docker** & **Docker Compose** (optional, for containerised setup)
- [`just`](https://github.com/casey/just) — task runner
- `cargo-llvm-cov` and `cargo-nextest` — for running tests (installed via `just prepare`)
- `sqlx-cli` — for database migrations and query checks (installed via `just prepare`)

## Setup

### Using Docker Compose (recommended)

1. Copy the example env file and fill in the required values:
   ```bash
   cp example-files/.env .env
   # edit .env as needed
   ```
2. Start all services (PostgreSQL, backend, frontend):
   ```bash
   docker compose up -d
   ```
    - Backend API: <http://localhost:8080>
    - Frontend: <http://localhost:9797>

### Local development

1. Start a PostgreSQL instance (or use the provided Compose service):
   ```bash
   docker compose up -d postgres
   ```
2. Copy and configure the env file:
   ```bash
   cp example-files/.env .env
   # set DATABASE_URL and other variables
   ```
3. Install backend tooling:
   ```bash
   just prepare
   ```
4. Run the backend:
   ```bash
   just run
   ```
5. Install and run the frontend:
   ```bash
   cd frontend
   pnpm install
   pnpm start        # serves on http://localhost:4200 (proxied to backend)
   ```

## Environment Variables

| Variable       | Default                                           | Description                     |
|----------------|---------------------------------------------------|---------------------------------|
| `DATABASE_URL` | `postgres://postgres:password@localhost/postgres` | PostgreSQL connection string    |
| `PORT`         | `8080`                                            | Port the backend API listens on |

> Additional variables (API keys, etc.) may be required — check `example-files/` for a reference `.env` file.

## Scripts (`justfile`)

| Command              | Description                                         |
|----------------------|-----------------------------------------------------|
| `just`               | Format, test, and lint (default)                    |
| `just build`         | Build the backend (debug)                           |
| `just build-release` | Build the backend (release)                         |
| `just run`           | Run the backend (debug)                             |
| `just run-release`   | Run the backend (release)                           |
| `just test`          | Run tests with coverage (`cargo llvm-cov nextest`)  |
| `just lint`          | Run Clippy, sqlx prepare check, and frontend linter |
| `just format`        | Format Rust code with `cargo fmt`                   |
| `just prepare`       | Install `sqlx-cli` and `cargo-nextest`              |
| `just clean`         | Remove the `target/` directory                      |

### Frontend scripts (`frontend/`)

| Command              | Description                         |
|----------------------|-------------------------------------|
| `pnpm start`         | Serve the Angular app (dev mode)    |
| `pnpm build`         | Build for production                |
| `pnpm test`          | Run unit tests                      |
| `pnpm test:coverage` | Run unit tests with coverage report |
| `pnpm lint`          | Check formatting (Prettier)         |
| `pnpm lint:fix`      | Auto-fix formatting                 |

## Tests

```bash
# Backend (with coverage)
just test

# Frontend
cd frontend && pnpm test
```

Coverage reports are uploaded to [Codecov](https://codecov.io/gh/michaelcoll/card-collection-price-tracker).

## Project Structure

```
.
├── src/ccpt/               # Rust backend source (hexagonal architecture)
│   ├── main.rs             # Entry point
│   ├── domain/             # Domain models and interfaces
│   ├── application/        # Application services
│   └── infrastructure/     # Adapters (HTTP controllers, DB repositories)
├── frontend/               # Angular frontend (pnpm)
├── migrations/             # SQLx SQL migration files
├── collection/             # Bruno API collection for manual testing
├── Dockerfile              # Backend Docker image
├── docker-compose.yml      # Full-stack Compose setup
├── justfile                # Task runner commands
└── Cargo.toml              # Rust package manifest
```

## License

MIT — see [LICENSE](LICENSE).
