---
applyTo: "src/**,migrations/**,Cargo.toml,Cargo.lock,justfile"
---

# Backend Development Guide (Rust)

## Workflow & Commands

- **Tests/Linting** : `rtk cargo test` (inclut llvm-cov) / `rtk cargo lint`
- **Build** : `rtk cargo build`
- **Dev Setup** : `rtk docker-compose up` (inclut migrations auto)

## Architecture & Patterns

- **Architecture** : Clean Architecture (Domain, Application, Infrastructure). Dépendances unidirectionnelles.
- **Injection** : Utiliser le pattern `Arc<dyn Trait>` pour les services. Voir `infrastructure.rs` pour la construction
  du graphe de dépendances.
- **Erreurs** : `domain/error.rs` (Domain) $\rightarrow$ `application/error.rs` (`AppError`) $\rightarrow$
  Infrastructure (implémenter `From<ExternalError> for AppError`).
- **Tests** : Utiliser `mockall` pour les traits et `wiremock` pour les appels externes. Les tests d'intégration
  utilisent la DB réelle.

## Data & External Services

- **Database** : SQLX avec vérification des requêtes à la compilation. Migrations dans `migrations/`, appliquées au
  démarrage.
- **API Adapters** : Tous les appels externes passent par `infrastructure/adapter_out/caller/`.
- **Services Externes** :
    - CardMarket : Téléchargement JSON en masse (sans authentification).
    - Autres : API REST avec gestion du *rate limiting* (`ratelimit` crate).

## Configuration & Déploiement

- **Env Vars** : `DATABASE_URL`, `PORT` (default: 8080), `CARDMARKET_PRICE_GUIDES_URL`, `EDHREC_BASE_URL`,
  `SCRYFALL_BASE_URL`.
- **Tâches Planifiées** : Importation des prix toutes les 6 heures via `cron_tab` (voir `infrastructure.rs:72`).

## Data Ingestion

- **Format** : Les imports attendent le format CSV ManaBox. Le parser est dans `application/service/parse_service.rs`.