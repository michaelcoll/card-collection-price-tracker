# Authentification Bearer Token Clerk

## Vue d'ensemble
Authentification basée sur **Clerk** utilisant des JWT validés par le backend.

### Variables d'environnement
Ajouter dans `.env` :
```bash
CLERK_FRONTEND_API_URL=https://musical-pup-67.clerk.accounts.dev
```

### Endpoints Protégés (Requiert Token)
- `GET /cards/` : Collection paginée de l'utilisateur.
- `POST /cards/import` : Importation de cartes (CSV ManaBox).
- `POST /cards/card-info` : Information sur une carte.

### Endpoints Publics
- `GET /maintenance/stats` : Statistiques globales (sans authentification).
- `POST /maintenance/trigger-price-update` : Déclenchement manuel de la mise à jour des prix.

## Flux d'utilisation
1.  **Frontend (Angular)** : Le `AuthService` obtient le token via `clerk.session.getToken()` et l'inclut dans les headers de requête (`Authorization: Bearer <token>`).
2.  **Backend (Rust)** : L'extracteur `AuthenticatedUser` gère la validation du JWT en utilisant les clés publiques Clerk (JWKS) :
    *   Vérifie la signature, l'issuer (`https://musical-pup-67.clerk.accounts.dev`), et l'expiration.
    *   Retourne HTTP 401 en cas d'échec.

## Modèle Utilisateur
Chaque utilisateur est identifié par :
- `user.id` : Le "sub" du JWT Clerk (clé primaire pour isoler les données).
- `user.email` : L'adresse email de l'utilisateur.

**Pour ajouter un nouvel endpoint protégé, utilisez l'extracteur `AuthenticatedUser` dans la signature de votre fonction handler.**

