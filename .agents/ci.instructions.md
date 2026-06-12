---
applyTo: ".github/**,Dockerfile,Dockerfile.*,docker-compose.yml,.dockerignore,codecov.yml"
---

# GitHub Actions CI/CD Guide

## Pipelines

### 1. Backend CI (`lint-test.yml`) — déclenché à chaque push

- **lint** : `rustfmt` + `clippy` (dans conteneur `rust:1-bullseye` avec PostgreSQL 18 en service)
- **test** : `cargo llvm-cov nextest` avec envoi de couverture vers Codecov
- **build-offline** : `SQLX_OFFLINE=true cargo build` pour valider les métadonnées SQLX
- **check-openapi** : régénère `doc/openapi.yml` et vérifie qu'il est à jour

### 2. Frontend CI (`frontend-lint-test.yml`) — déclenché à chaque push

- **format** : Prettier (`pnpm lint`)
- **test** : Vitest avec couverture (`pnpm test:coverage`)
- **build** : builds dev et production Angular

### 3. Build & Push (`build-push.yml`) — déclenché sur push à `main` ou release

- Construit et publie les images Docker backend et frontend vers **GHCR**
- Sur release : bump semver + publication des sourcemaps vers Sentry
- Plates-forme : `linux/amdtd64` uniquement

### 4. Automatisation PR (`automerge.yml`, `pr-label.yml`, `clean-cache.yml`)

- **automerge** : dependabot patch/minor auto-merged
- **pr-label** : labels conventionnels (fix/feat/chore/ci) depuis le titre du PR
- **clean-cache** : suppression du cache GitHub runner à la fermeture d'un PR

## Configuration locale

- **mise** (`mise.toml`) : gestion des toolchains et tâches (`mise run`, `mise back`, `mise front`, `mise test`, `mise lint`)
- **pnpm** : version CI = `10.32.1` (à noter : `mise.toml` spécifie `pnpm 11`)

## Docker

- **Backend** : multi-stage (rust → distroless nonroot, port 8080)
- **Frontend** : multi-stage (node → nginx-alpine, route SPA)
- **docker-compose** : postgres 18, ccpt backend, frontend nginx (port 9797)
