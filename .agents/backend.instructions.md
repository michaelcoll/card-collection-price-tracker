# Backend Development Guide (Rust)

## Architecture & Patterns

- **Architecture**: Clean Architecture (Domain, Application, Infrastructure) under `src/ccpt/`. Unidirectional
  dependencies.
- **Injection**: Use the `Arc<dyn Trait>` pattern for services, wired up in `infrastructure.rs`.
- **Errors**: `AppError` (`application/error.rs`) is a thin umbrella over three category enums, each carrying its
  own concern: `FunctionalError` (`domain/error.rs` — business/validation errors: parsing, not-found, business rule
  violations), `AuthenticationError` and `InfraError` (`application/error.rs` — repository/call/queue failures).
  `AppError::Functional/Authentication/Infra(...)` wraps the category; `From<T> for AppError` is implemented per
  category so `?` still works from any layer.
- **Tests**: `mockall` (automock on traits) and `wiremock` for external HTTP calls. Integration tests use the real
  DB.

## Data & External Services

- **Database**: SQLX with compile-time query verification. Migrations live in `migrations/`, applied at startup.
- **API Adapters**: All external calls go through `infrastructure/adapter_out/caller/` (CardMarket, EdhRec,
  Scryfall).
- **Rate Limiting**: The `ratelimit` crate is used for Scryfall.
- **CardMarket**: Bulk JSON download, no authentication.

## Configuration

- **Env Vars**: `DATABASE_URL`, `BACKEND_PORT` (default: 8080), `CARDMARKET_PRICE_GUIDES_URL`, `EDHREC_BASE_URL`,
  `SCRYFALL_BASE_URL`, `CLERK_FRONTEND_API_URL` (required).
- **Scheduled Tasks**: Price import every 12 hours via `AsyncCron` in `infrastructure.rs`.

## Data Ingestion

- **Format**: ManaBox CSV. The parser lives in `application/service/parse_service.rs`.
