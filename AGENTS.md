# AI Agent Guide: Card Collection Price Tracker

## Architecture Overview

This is a **Clean Architecture** Rust application with an Angular frontend for tracking Magic: The Gathering card
prices. The backend follows Domain-Driven Design with strict layer separation:

- **Domain** (`src/ccpt/domain/`) - Core business entities (`Card`, `Price`, `LanguageCode`, etc.) with no external
  dependencies
- **Application** (`src/ccpt/application/`) - Use cases, services, and repository/caller interfaces
- **Infrastructure** (`src/ccpt/infrastructure/`) - External integrations (databases, APIs) via adapter pattern

Key architectural constraint: **All dependencies point inward**. Domain never imports from application/infrastructure
layers.

## Key Domain Concepts

Cards are uniquely identified by `CardId`: `set_code + collector_number + language_code + foil` (see `domain/card.rs`).
The system integrates with:

- **CardMarket**: Price data via JSON downloads (not authenticated API)
- **Scryfall**: Card metadata and IDs
- **EDHRec**: Commander format analytics

## Critical Workflows

### Backend Development

```bash
# Essential commands via justfile (ALWAYS use these)
cargo test       # Run tests with cargo-nextest + llvm-cov
cargo lint       # cargo fmt + clippy with specific rules  
cargo build      # Standard cargo build

# Database setup
export DATABASE_URL="postgres://postgres:password@localhost/postgres"
# Migrations auto-run on startup via sqlx::migrate!
```

### Frontend Development

```bash
cd frontend
pnpm install    # Uses pnpm workspace (see pnpm-workspace.yaml)
ng serve        # Angular dev server
ng test         # Vitest test runner
ng build        # Production build
```

### Docker Development

```bash
docker-compose up  # Postgres + backend (auto-migrates schema)
```

## Code Patterns & Conventions

### Dependency Injection

Services use `Arc<dyn Trait>` pattern. See `infrastructure.rs:create_infra()` for the dependency graph construction -
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

## Frontend Design System

⚠️ **MANDATORY**: Every time you work on any frontend file (Angular components, styles, templates), you **MUST**
strictly
follow the design system defined in [`frontend/DESIGN.md`](frontend/DESIGN.md). No exceptions.

### Key rules at a glance (read the full file for details)

| Rule              | Constraint                                                                                          |
|-------------------|-----------------------------------------------------------------------------------------------------|
| **Colors**        | Dark "Mystic Dark" palette — `surface` (#131313) as base, `primary` (#cdbdff), `tertiary` (#00daf3) |
| **No borders**    | Never use 1px solid borders. Use background-color shifts and spacing instead                        |
| **Typography**    | Inter font only. Display-LG with `-0.02em` tracking for headlines                                   |
| **Elevation**     | Tonal layering (background shifts), not drop shadows                                                |
| **Buttons**       | Primary = `primary` bg + `on_primary` text; Google Login = monochrome on `surface_container_high`   |
| **EDHREC bar**    | `tertiary` fill, 4px height, `surface_container_highest` track                                      |
| **Price badges**  | `secondary_container` bg, `roundedness-full` pill shape                                             |
| **Text color**    | Never pure white — always `on_surface` (#e5e2e1)                                                    |
| **Card hover**    | `scale(1.02)` + shift to `surface_bright` (#393939). No glow                                        |
| **Glassmorphism** | `backdrop-filter: blur(12px)` + 60% opacity on floating widgets                                     |

