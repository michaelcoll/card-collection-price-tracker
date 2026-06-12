---
applyTo: "src/**,migrations/**,Cargo.toml,Cargo.lock"
---

# Backend Development Guide (Rust)

## Architecture & Patterns

- **Architecture** : Clean Architecture (Domain, Application, Infrastructure) sous `src/ccpt/`. Dépendances
  unidirectionnelles.
- **Injection** : Utiliser le pattern `Arc<dyn Trait>` pour les services, construit dans `infrastructure.rs`.
- **Erreurs** : `domain/error.rs` → `application/error.rs` (`AppError`). Les erreurs infrastructure (`RepositoryError`,
  `CallError`) sont des variants dans `AppError`. Implémenter `From<T> for AppError` pour les conversions.
- **Tests** : `mockall` (automock sur les traits) et `wiremock` pour les appels HTTP externes. Les tests d'intégration
  utilisent la DB réelle.

## Data & External Services

- **Database** : SQLX avec vérification des requêtes à la compilation. Migrations dans `migrations/`, appliquées au
  démarrage.
- **API Adapters** : Tous les appels externes passent par `infrastructure/adapter_out/caller/` (CardMarket, EdhRec,
  Scryfall).
- **Rate Limiting** : Le crate `ratelimit` est utilisé pour Scryfall.
- **CardMarket** : Téléchargement JSON en masse sans authentification.

## Configuration

- **Env Vars** : `DATABASE_URL`, `BACKEND_PORT` (default: 8080), `CARDMARKET_PRICE_GUIDES_URL`, `EDHREC_BASE_URL`,
  `SCRYFALL_BASE_URL`, `CLERK_FRONTEND_API_URL` (obligatoire).
- **Tâches Planifiées** : Importation des prix toutes les 12 heures via `AsyncCron` dans `infrastructure.rs`.

## Data Ingestion

- **Format** : CSV ManaBox. Le parser est dans `application/service/parse_service.rs`.