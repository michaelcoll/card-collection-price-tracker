# Authentification Bearer Token Google

## Configuration

L'authentification Google OAuth2 est maintenant implémentée sur les endpoints protégés.

### Variables d'environnement requises

Ajoutez dans votre `.env` ou variables d'environnement :

```bash
GOOGLE_CLIENT_ID=votre-client-id-google.apps.googleusercontent.com
```

Pour obtenir un Google Client ID :
1. Allez sur [Google Cloud Console](https://console.cloud.google.com/)
2. Créez un projet ou sélectionnez-en un existant
3. Activez l'API "Google+ API" ou "People API"
4. Allez dans "Credentials" > "Create Credentials" > "OAuth 2.0 Client ID"
5. Copiez le "Client ID"

## Endpoints protégés

Les endpoints suivants requièrent un Bearer Token :

- `POST /cards/import` - Import de cartes depuis un CSV ManaBox
- `POST /cards/card-info` - Information sur une carte

## Endpoints publics

- `GET /stats` - Statistiques globales (pas d'authentification requise)

## Utilisation

### Obtenir un token Google

Côté frontend, utilisez la bibliothèque Google Sign-In ou `@react-oauth/google` pour Angular/React :

```typescript
// Exemple Angular
import { Injectable } from '@angular/core';

@Injectable({ providedIn: 'root' })
export class AuthService {
  private token: string | null = null;

  // Après la connexion Google, stocker le token
  setToken(idToken: string) {
    this.token = idToken;
  }

  // Utiliser le token dans les requêtes
  getAuthHeaders() {
    return {
      'Authorization': `Bearer ${this.token}`
    };
  }
}
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

2. **Utiliser un token de test** : Générez un token Google valide via le [OAuth 2.0 Playground](https://developers.google.com/oauthplayground/)

3. **Mock pour tests** : Les tests unitaires utilisent `MockAuthService` qui retourne automatiquement un utilisateur de test

## Architecture

L'implémentation suit la Clean Architecture :

- **Domain** (`domain/user.rs`) - Entité User avec id, email, name
- **Application** (`application/service/auth_service.rs`) - Trait `AuthService` et implémentation `GoogleAuthService`
- **Infrastructure** (`infrastructure/adapter_in/auth_extractor.rs`) - Extracteur Axum `AuthenticatedUser`

Le service d'authentification :
- Télécharge les clés publiques Google (JWKS) au démarrage
- Valide les JWT en vérifiant :
  - La signature avec les clés publiques de Google
  - L'issuer (accounts.google.com)
  - L'audience (votre Client ID)
  - La date d'expiration

## Sécurité

- ✅ Validation cryptographique réelle du JWT avec les clés publiques Google
- ✅ Vérification de l'issuer et audience
- ✅ Vérification de l'expiration du token
- ✅ Pas de validation naïve ou tokens auto-signés
- ✅ Les erreurs d'authentification retournent HTTP 401

## Association utilisateur-données

Chaque utilisateur authentifié a :
- `user.id` : Le "sub" du JWT Google (identifiant unique Google)
- `user.email` : Email de l'utilisateur
- `user.name` : Nom de l'utilisateur (optionnel)

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
- Valide le token avec Google
- Retourne HTTP 401 si invalide
- Injecte le `User` dans votre handler

