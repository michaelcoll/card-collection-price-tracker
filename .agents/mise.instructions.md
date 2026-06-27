# Mise & Development Workflow

## Quick Reference

All local commands are run through **mise** (task runner):

```
mise install           # One-time: install toolchains (Rust, Node, pnpm, etc.)
mise run setup         # Full dev setup: clean + install frontend deps
```

## Command Summary

| Action                     | Command            | Alias        |
|----------------------------|--------------------|--------------|
| **All checks**             | `mise run`         | —            |
| **Backend server**         | `mise run back`    | —            |
| **Frontend dev server**    | `mise run front`   | —            |
| **Backend tests**          | `mise run test`    | `mise run t` |
| **Backend lint**           | `mise run lint`    | `mise run l` |
| **Format code front/back** | `mise run format`  | `mise run f` |
| **OpenAPI gen**            | `mise run openapi` | `mise run o` |
| **DB migrations**          | `mise run migrate` | —            |
| **Clean artifacts**        | `mise run clean`   | `mise run c` |
| **Upgrade deps**           | `mise run upgrade` | —            |

## Detailed Commands

### Build

- **Backend** : `cargo build` (ou `mise run back` pour lancer directement). En production/Docker :
  `SQLX_OFFLINE=true cargo build --release`
- **Frontend** : `cd frontend && pnpm build` (prod) ou `cd frontend && pnpm build --configuration development` (dev)

### Test

- **Backend** : `cargo test` — utilise `cargo-nextest` + `cargo-llvm-cov` pour la couverture
- **Frontend** : `cd frontend && pnpm test` (Vitest via `ng test`) ou `cd frontend && pnpm test:coverage` pour la
  couverture

### Lint

- **Backend** : `mise run lint` (= `mise run l`)
    1. `cargo clippy --locked --workspace --all-features --all-targets -- -A dead_code -D clippy::all`
    2. `cargo sqlx prepare --check` (valide les requêtes SQL contre la DB)
- **Frontend** : `cd frontend && pnpm lint` (Prettier check) ou `cd frontend && pnpm lint:fix` (auto-fix)

### Format

- **Backend** : `cargo fmt` (rustfmt)
- **Frontend** : `cd frontend && pnpm lint:fix` (Prettier)
- **Les deux** : `mise run format` (= `mise run f`)

### OpenAPI

```
mise run openapi        # Génère doc/openapi.yml
```

### Database

```
mise run migrate        # Applique les migrations SQLx
mise run sqlx-prepare   # Génère le metadata SQLx (à lancer après modification de requêtes)
```
