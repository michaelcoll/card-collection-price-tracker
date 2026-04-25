---
applyTo: "src/**,migrations/**,Cargo.toml,Cargo.lock,justfile"
---

## Backend Development

```bash
# Essential commands via justfile (ALWAYS use these)
rtk cargo test       # Run tests with cargo-nextest + llvm-cov
rtk cargo lint       # cargo fmt + clippy with specific rules
rtk cargo build      # Standard cargo build

# Database setup
export DATABASE_URL="postgres://postgres:password@localhost/postgres"
# Migrations auto-run on startup via sqlx::migrate!
```

```bash
rtk docker-compose up  # Postgres + backend (auto-migrates schema)
```

## Code Patterns & Conventions

### Dependency Injection

Services use `Arc<dyn Trait>` pattern. See `infrastructure.rs:create_infra()` for the dependency graph construction —
all adapters created there.

### Error Handling

- Domain errors: Custom enums in `domain/error.rs`
- Application errors: `AppError` enum in `application/error.rs`
- Infrastructure errors: Implement `From<ExternalError> for AppError`

### Testing

- Use `mockall` crate with `#[cfg_attr(test, automock)]` on traits
- Integration tests call real database (not mocked repositories)
- External API calls use `wiremock` for testing

### External API Integration

- All external calls go through dedicated adapters in `infrastructure/adapter_out/caller/`
- Rate limiting via `ratelimit` crate
- CardMarket uses bulk JSON download (no auth), others use REST APIs

### Database Patterns

- SQLX with compile-time query checking
- All queries in repository adapters
- Migrations in `migrations/` directory, auto-applied on startup

## Environment Configuration

Required environment variables:

- `DATABASE_URL`: Postgres connection string
- `PORT`: HTTP server port (default: 8080)
- `CARDMARKET_PRICE_GUIDES_URL`: JSON download URL
- `EDHREC_BASE_URL`, `SCRYFALL_BASE_URL`: API base URLs

## Scheduled Tasks

Price import runs every 6 hours via `cron_tab` crate (see `infrastructure.rs` line 72). Uses async cron jobs with UTC
timezone.

## Import Data Format

Card imports expect ManaBox CSV format. See `example-files/ManaBox_Collection.csv` for structure. Parser in
`application/service/parse_service.rs` handles the CSV-to-domain mapping.

