# card-collection-price-tracker

[![codecov](https://codecov.io/gh/michaelcoll/card-collection-price-tracker/graph/badge.svg?token=b2Wlmg2WX3)](https://codecov.io/gh/michaelcoll/card-collection-price-tracker)

## Overview

A web application tracking card collection prices (e.g., Magic: The Gathering). It fetches and stores prices via a REST
API consumed by an Angular frontend.

## Requirements

- **Rust** (edition 2024, stable toolchain)
- **Node.js** >= 24 & **pnpm** >= 10.33.0 (Frontend)
- **PostgreSQL** 18
- **Docker/Compose** (Optional)
- [`just`](https://github.com/casey/just) — Task runner

## Setup

### Docker Compose (Recommended)

1. Copy and configure `.env` from `example-files/`.
2. Start services: `docker compose up -d`
    - Backend API: <http://localhost:8080>
    - Frontend: <http://localhost:9797>

### Local Development

1. Start PostgreSQL service (if not using Compose).
2. Copy/configure `.env`.
3. Install backend tooling: `just prepare`
4. Install frontend dependencies: `cd frontend && pnpm install && cd ..`
5. Run both services together (requires `tmux`): `just run`
    - This starts a tmux session (`ccpt`) with backend (`cargo run`, API on <http://localhost:8080>) and frontend (
      `pnpm start`, app on <http://localhost:4200>).
    - Use `Ctrl+B d` to detach.

## Environment Variables

| Variable                 | Default                                           | Description                                      |
|--------------------------|---------------------------------------------------|--------------------------------------------------|
| `DATABASE_URL`           | `postgres://postgres:password@localhost/postgres` | PostgreSQL connection string                     |
| `PORT`                   | `8080`                                            | Backend API port                                 |
| `CLERK_FRONTEND_API_URL` | *(required)*                                      | Clerk frontend API URL for JWT validation (JWKS) |

> **Authentication** is handled via [Clerk](https://clerk.com/). Set `CLERK_FRONTEND_API_URL` to your Clerk instance
> URL.

## Scripts (`justfile`)

| Command              | Description                                        |
|----------------------|----------------------------------------------------|
| `just`               | Format, test, and lint (default)                   |
| `just build`         | Build backend (debug)                              |
| `just build-release` | Build backend (release)                            |
| `just run`           | Run backend & frontend in tmux session (`ccpt`)    |
| `just run-release`   | Run backend (release)                              |
| `just test`          | Run tests with coverage (`cargo llvm-cov nextest`) |
| `just lint`          | Lint backend & frontend                            |
| `just format`        | Format Rust code (`cargo fmt`)                     |
| `just prepare`       | Install testing/linting dependencies               |
| `just clean`         | Remove `target/` directory                         |

### Frontend Scripts (`frontend/`)

| Command      | Description                             |
|--------------|-----------------------------------------|
| `pnpm start` | Serve Angular app (dev mode, port 4200) |
| `pnpm build` | Build for production                    |
| `pnpm test`  | Run unit tests (Vitest)                 |
| `pnpm lint`  | Check formatting (Prettier)             |

## Tests

- Backend: `just test`
- Frontend: `cd frontend && pnpm test`

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

The backend follows **Clean Architecture** with strict inward dependency flow: `Domain ← Application ← Infrastructure`.

- **Domain**: Pure business entities.
- **Application**: Use cases and repository traits.
- **Infrastructure**: Adapters (Axum, SQLx, External Callers).

Cards are identified by `CardId`: `set_code + collector_number + language_code + foil`.
