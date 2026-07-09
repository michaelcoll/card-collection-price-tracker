# Spec : Endpoint `POST /user/register`

## Contexte

L'authentification repose sur Clerk : le frontend obtient un JWT et l'envoie en `Authorization: Bearer <token>`. Le
backend valide ce token (`AuthService::validate_token`) et en extrait un `User` (`id` = claim `sub`, `email`).
Aujourd'hui, aucun utilisateur n'est jamais persisté en base — seul `user_id` est utilisé comme clé étrangère
implicite dans `collection_entry` et `collection_price_history`, sans table `users` de référence.

Un nouveau claim `username` a été ajouté au token Clerk. On souhaite désormais matérialiser les utilisateurs
connus du système dans une table dédiée, à partir des informations du token, sans mettre en place un vrai
mécanisme de login/session (pas de génération de token, pas de cookie — Clerk reste la seule source de vérité
pour l'authentification).

## Objectif

À chaque authentification Clerk (login ou refresh de session), le frontend appelle `POST /user/register`. Le
backend extrait `sub` et `username` du token déjà validé et enregistre (ou met à jour) l'utilisateur
correspondant dans une nouvelle table `users`.

## Solution

### Modèle de données

Nouvelle table `users`, avec pour seules colonnes l'identifiant Clerk (`sub`, même convention que le
`user_id VARCHAR(50)` déjà utilisé dans `collection_entry`) et le `username`. Pas de contrainte d'unicité sur le
username, pas de colonnes additionnelles (`created_at`, etc.) à ce stade — hors périmètre.

### Extraction du claim `username`

Le claim `username` devient un attribut du domaine `User`, au même titre que `email` aujourd'hui, et donc
disponible pour tous les endpoints authentifiés via le mécanisme d'extraction existant — pas de logique de
décodage spécifique à `/login`.

Un token sans ce claim doit rester valide pour l'authentification générale (comme c'est déjà le cas pour
`email`) : la validation stricte de sa présence n'a lieu que dans le handler `/user/register`, pour ne pas
casser les autres endpoints avec d'éventuels tokens plus anciens.

### Endpoint

- **Route** : `POST /user/register`, montée au même niveau que les autres groupes de routes existants (`/cards`,
  `/maintenance`).
- **Authentification** : protégé par le même mécanisme que les autres endpoints (401 si token absent/invalide).
- **Requête** : pas de body.
- **Réponse en cas de succès** : `204 No Content`, pas de body.

### Persistance

Écriture en base en upsert : crée l'utilisateur s'il n'existe pas, met à jour son `username` sinon. Ça garantit
que `username` reste synchronisé avec Clerk à chaque appel (changement de pseudo côté Clerk répercuté au
prochain login).

### Documentation

Le endpoint doit être documenté dans la spec OpenAPI existante (`doc/openapi.yml`), sous le même tag que les
autres endpoints d'authentification.

## Cas d'erreurs

- **Token absent ou invalide** : 401 Unauthorized (comportement standard, inchangé).
- **Claim `username` absent du token** : 400 Bad Request. Aucune écriture en base dans ce cas.
- **Utilisateur déjà existant (même `id`)** : pas une erreur — upsert, `username` mis à jour si différent,
  réponse `204 No Content` identique à une première inscription.
- **Erreur base de données lors de l'upsert** : 500 Internal Server Error (pattern d'erreur existant).

## Critères d'acceptance

- [ ] Une migration crée la table `users` avec un identifiant Clerk en clé primaire et un `username` obligatoire.
- [ ] Given un token Clerk valide contenant `sub` et `username`, When j'appelle `POST /user/register` avec ce
      token en bearer, Then la réponse est `204 No Content` et une ligne `(id = sub, username = username)`
      existe dans `users`.
- [ ] Given un utilisateur déjà présent dans `users` avec un `username` différent, When j'appelle
      `POST /user/register` avec un token portant le nouveau `username`, Then la réponse est `204 No Content`
      et la colonne `username` de la ligne existante est mise à jour (pas de doublon, même `id`).
- [ ] Given un token Clerk valide mais sans claim `username`, When j'appelle `POST /user/register`, Then la
      réponse est `400 Bad Request` et aucune ligne n'est insérée ou modifiée dans `users`.
- [ ] Given aucune en-tête `Authorization`, When j'appelle `POST /user/register`, Then la réponse est
      `401 Unauthorized` (comportement inchangé).
- [ ] Given un token invalide (signature, expiration), When j'appelle `POST /user/register`, Then la réponse
      est `401 Unauthorized`.
- [ ] Les endpoints existants (`/cards/*`, `/maintenance/*`) continuent de fonctionner avec un token ne
      contenant pas le claim `username` (pas de régression : `User.username` reste optionnel dans le flux
      d'authentification général).
- [ ] Le endpoint `POST /user/register` est documenté dans `doc/openapi.yml`.
