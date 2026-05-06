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
- `cargo-llvm-cov`, `cargo-nextest` and `sqlx-cli` — for tests and query checks (installed via `just prepare`)

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
   # set DATABASE_URL, CLERK_FRONTEND_API_URL and other variables
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

| Variable                 | Default                                           | Description                                      |
|--------------------------|---------------------------------------------------|--------------------------------------------------|
| `DATABASE_URL`           | `postgres://postgres:password@localhost/postgres` | PostgreSQL connection string                     |
| `PORT`                   | `8080`                                            | Port the backend API listens on                  |
| `CLERK_FRONTEND_API_URL` | *(required)*                                      | Clerk frontend API URL for JWT validation (JWKS) |

> **Authentication** is handled via [Clerk](https://clerk.com/). The backend validates JWT tokens issued by Clerk.
> Set `CLERK_FRONTEND_API_URL` to your Clerk instance URL (e.g. `https://<your-instance>.clerk.accounts.dev`).

## Scripts (`justfile`)

| Command              | Description                                                  |
|----------------------|--------------------------------------------------------------|
| `just`               | Format, test, and lint (default)                             |
| `just build`         | Build the backend (debug)                                    |
| `just build-release` | Build the backend (release)                                  |
| `just run`           | Run the backend (debug)                                      |
| `just run-release`   | Run the backend (release)                                    |
| `just test`          | Run tests with coverage (`cargo llvm-cov nextest`)           |
| `just lint`          | Run Clippy, sqlx prepare check, and auto-format the frontend |
| `just format`        | Format Rust code with `cargo fmt`                            |
| `just prepare`       | Install `sqlx-cli`, `cargo-nextest`, and `cargo-llvm-cov`    |
| `just clean`         | Remove the `target/` directory                               |

### Frontend scripts (`frontend/`)

| Command              | Description                                 |
|----------------------|---------------------------------------------|
| `pnpm start`         | Serve the Angular app (dev mode, port 4200) |
| `pnpm build`         | Build for production                        |
| `pnpm test`          | Run unit tests with Vitest                  |
| `pnpm test:coverage` | Run unit tests with coverage report         |
| `pnpm lint`          | Check formatting with Prettier              |
| `pnpm lint:fix`      | Auto-fix formatting with Prettier           |

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
├── src/ccpt/                          # Rust backend (hexagonal / clean architecture)
│   ├── main.rs                        # Entry point
│   ├── domain/                        # Domain models (Card, Price, User, LanguageCode…)
│   ├── application/                   # Use cases and service interfaces
│   └── infrastructure/
│       ├── adapter_in/                # HTTP controllers (Axum), auth extractor
│       └── adapter_out/               # DB repositories (SQLx), external callers
├── frontend/                          # Angular 21 frontend (pnpm + Tailwind CSS + Angular Material)
├── migrations/                        # SQLx SQL migration files
├── collection/                        # Bruno API collection for manual testing
├── example-files/                     # Example CSV and credentials files
├── Dockerfile                         # Backend Docker image
├── docker-compose.yml                 # Full-stack Compose setup
├── justfile                           # Task runner commands
└── Cargo.toml                         # Rust package manifest
```

### Architecture

The backend follows **Clean Architecture** with strict inward dependency flow:

```
Domain ← Application ← Infrastructure
```

- **Domain** — Pure business entities (`Card`, `Price`, `User`, `LanguageCode`, …). No external dependencies.
- **Application** — Use cases and repository/caller traits.
- **Infrastructure** — Adapters: Axum HTTP controllers, SQLx PostgreSQL repositories,
  CallerMarket / Scryfall / EDHRec callers.

Cards are uniquely identified by `CardId`: `set_code + collector_number + language_code + foil`.
