# Authentification Bearer Token Clerk

## Configuration

L'authentification est implémentée via **Clerk** sur les endpoints protégés. Le backend valide les JWT Clerk via JWKS.

### Variables d'environnement requises

Ajoutez dans votre `.env` ou variables d'environnement :

```bash
CLERK_FRONTEND_API_URL=https://musical-pup-67.clerk.accounts.dev
```

## Endpoints protégés

Les endpoints suivants requièrent un Bearer Token :

- `POST /cards/import` - Import de cartes depuis un CSV ManaBox
- `POST /cards/card-info` - Information sur une carte

## Endpoints publics

- `GET /stats` - Statistiques globales (pas d'authentification requise)

## Utilisation

### Obtenir un token Clerk

Côté frontend, le `AuthService` Angular appelle `clerk.session.getToken()` et fournit le header via `getAuthHeaders()`.

```typescript
// Dans un composant ou service Angular
const headers = await this.authService.getAuthHeaders();
// { Authorization: 'Bearer eyJ...' }
```

### Appeler les endpoints protégés

Incluez le header `Authorization: Bearer <token>` dans toutes vos requêtes :

```bash
curl -X POST http://localhost:8080/cards/import \
  -H "Authorization: Bearer eyJhbGciOiJSUzI1NiIsImtpZCI6IjE..." \
  -H "Content-Type: text/plain" \
  --data-binary @ManaBox_Collection.csv
```

### Tester localement

Pour tester en développement sans authentification réelle, vous pouvez :

1. **Désactiver temporairement l'authentification** : Commentez `AuthenticatedUser(user): AuthenticatedUser` dans les endpoints

2. **Mock pour tests** : Les tests unitaires utilisent `MockAuthService` qui retourne automatiquement un utilisateur de test

## Architecture

L'implémentation suit la Clean Architecture :

- **Domain** (`domain/user.rs`) - Entité User avec id, email, name
- **Application** (`application/service/auth_service.rs`) - Trait `AuthService` et implémentation `ClerkAuthService`
- **Infrastructure** (`infrastructure/adapter_in/auth_extractor.rs`) - Extracteur Axum `AuthenticatedUser`

Le service d'authentification (`ClerkAuthService`) :
- Télécharge les clés publiques Clerk (JWKS) au démarrage
  - JWKS URL : `https://musical-pup-67.clerk.accounts.dev/.well-known/jwks.json`
- Valide les JWT en vérifiant :
  - La signature avec les clés publiques Clerk
  - L'issuer (`https://musical-pup-67.clerk.accounts.dev`)
  - La date d'expiration
  - `validate_aud = false` (Clerk utilise `azp` plutôt que `aud` pour les session tokens)

## Sécurité

- ✅ Validation cryptographique réelle du JWT avec les clés publiques Clerk
- ✅ Vérification de l'issuer
- ✅ Vérification de l'expiration du token
- ✅ Pas de validation naïve ou tokens auto-signés
- ✅ Les erreurs d'authentification retournent HTTP 401

## Association utilisateur-données

Chaque utilisateur authentifié a :
- `user.id` : Le "sub" du JWT Clerk (format `user_xxx`)
- `user.email` : Email de l'utilisateur (claim `email` configuré dans le JWT Template Clerk)
- `user.name` : Non utilisé (non inclus dans le JWT Clerk par défaut)

Le `user.id` est utilisé comme clé dans la base de données pour isoler les collections de chaque utilisateur (voir `card_quantity` et `collection_price_history` tables avec le champ `user_id`).

## Développement

Pour ajouter l'authentification à un nouvel endpoint :

```rust
async fn my_endpoint(
    AuthenticatedUser(user): AuthenticatedUser,  // <- Ajoutez cet extracteur
    State(state): State<AppState>,
    // ... autres paramètres
) -> Result<String, AppError> {
    // user.id, user.email, user.name sont disponibles ici
    println!("Request from user: {} ({})", user.email, user.id);
    
    // Votre logique métier...
    Ok("Success".to_string())
}
```

L'extracteur `AuthenticatedUser` :
- Extrait automatiquement le header `Authorization`
- Valide le token avec Clerk
- Retourne HTTP 401 si invalide
- Injecte le `User` dans votre handler

