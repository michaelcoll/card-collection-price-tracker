# Spec : Historique de prix par carte (`/cards/{scryfall_id}/price-history`)

## Contexte

`GET /cards/price-history` renvoie aujourd'hui la valeur agrégée de **toute la collection** d'un utilisateur, jour
par jour (prix × quantité possédée, sommé sur toutes ses cartes), depuis la table pré-calculée
`collection_price_history` (clé `(date, user_id)`, aucune notion de carte individuelle). Il n'existe aucun moyen de
consulter l'évolution de prix d'une carte précise.

La donnée brute par carte existe déjà dans `cardmarket_price` (ledger append-only, clé `(id_produit, date)`, avec
colonnes `low/trend/avg` pour le normal et `low_foil/trend_foil/avg_foil` pour le foil). La table `card` relie
`scryfall_id` (UUID, identifiant public de la carte, un par printing y compris foil/non-foil) à `cardmarket_id`
(= `id_produit`) et porte le flag `foil` qui indique quelles colonnes utiliser. Un lookup à date unique existe déjà
sur cette table (`CardMarketPriceRepository::find_by_id_and_date`), ce qui donne un précédent direct pour une requête
sur plage de dates.

Aucune route de l'API actuelle ne prend de paramètre de chemin (path param) — toutes les routes de
`create_card_router` sont statiques (`/`, `/import`, `/card-info`, `/price-history`, `/stats`). Ce sera donc la
première route avec path param.

## Objectif

Permettre à tout utilisateur authentifié de récupérer l'historique de prix marché brut (low/trend/avg, en centimes)
d'une carte précise identifiée par son `scryfall_id`, sur une période donnée, avec le **même format de sortie** que
`/cards/price-history` — afin que le composant frontend de graphique de prix existant soit réutilisable tel quel
pour l'affichage d'une carte seule.

## Solution

### Route et accès

- Nouvelle route `GET /cards/{scryfall_id}/price-history` dans `create_card_router` (`card/controller.rs`).
  `scryfall_id` en path param, typé `Uuid` (cohérent avec le type utilisé partout en interne pour ce champ, ex.
  `domain/card.rs`).
- Authentification requise (`AuthenticatedUser`), comme les autres endpoints `/cards/*`. L'accès n'est **pas**
  restreint aux cartes possédées par l'appelant : toute carte existante en base est consultable (cohérent avec le
  catalogue introduit par la spec `003-cards-owned-filter`).

### Résolution de la carte

- Recherche de la carte par `scryfall_id` (nouvelle requête repository, aucune n'existe actuellement) pour obtenir
  `cardmarket_id` et `foil`.
- Carte introuvable pour ce `scryfall_id` → 404.
- Carte trouvée mais sans `cardmarket_id` (jamais liée à CardMarket) → traité comme "pas de données de prix" :
  200 avec liste vide (même traitement qu'une carte sans prix sur la période demandée).

### Récupération des prix

- Requête directe sur `cardmarket_price` (donnée brute par carte), filtrée sur `id_produit = card.cardmarket_id` et
  `date` dans la plage demandée — **pas** la table agrégée `collection_price_history`, et pas de pondération par une
  quantité possédée (le prix n'est pas propre à un utilisateur).
- Étendre `CardMarketPriceRepository` d'une méthode de plage de dates (ex. `find_by_id_and_date_range`), sur le même
  modèle que `find_by_id_and_date` existant.
- Gestion foil : si `card.foil = true`, utiliser `low_foil/trend_foil/avg_foil` ; sinon `low/trend/avg` — même
  logique de sélection de colonnes que celle déjà utilisée dans
  `collection_price_history_repository_adapter.rs::update_for_date_and_user`.

### Query params `start_date` / `end_date`

- Les deux sont **optionnels** (différence avec `/cards/price-history` où ils sont requis).
- `end_date` absent → aujourd'hui. `start_date` absent → `end_date` (fournie ou par défaut) moins 30 jours.
- Si fournis, format identique à l'existant (`YYYY-MM-DD`).
- Validation inchangée : `start_date > end_date` → erreur (même message/style que `/cards/price-history`).

### Format de réponse

- Réutiliser strictement le DTO existant `PriceHistoryEntryResponse` (`date`, `low`, `trend`, `avg`) et la même forme
  de réponse : tableau JSON brut (`Vec<PriceHistoryEntryResponse>`), sans wrapper ni pagination, sans champ
  additionnel (pas de `scryfall_id`/nom de carte dans la réponse — le composant frontend consommateur reste
  inchangé).

### OpenAPI / types générés

- Documenter le nouvel endpoint dans `doc/openapi.yml`. Aucun nouveau type `ts-rs` n'est nécessaire (le type de
  réponse `PriceHistoryEntry` ne change pas).

## Cas d'erreurs

- `scryfall_id` invalide (pas un UUID) dans le path → 400.
- Aucune carte ne correspond au `scryfall_id` → 404.
- Carte trouvée mais sans `cardmarket_id`, ou aucune donnée de prix sur la période → 200 avec liste vide.
- `start_date` fourni postérieur à `end_date` (fournie ou par défaut) → 400.
- Token bearer manquant ou invalide → 401 (comme tous les endpoints `/cards/*`).

## Critères d'acceptance

- [ ] `GET /cards/{scryfall_id}/price-history` avec `start_date` et `end_date` valides renvoie un tableau
      `[{date, low, trend, avg}]` trié par date, avec les prix bruts de la carte sur la période (colonnes normales
      si `foil = false`).
- [ ] Pour une carte foil, les valeurs renvoyées correspondent aux colonnes `_foil` de `cardmarket_price`.
- [ ] Sans `start_date` ni `end_date`, la réponse couvre les 30 derniers jours (jusqu'à aujourd'hui inclus).
- [ ] Avec seulement `end_date` fourni, `start_date` est calculé comme `end_date - 30 jours`.
- [ ] Avec seulement `start_date` fourni, `end_date` par défaut est la date du jour.
- [ ] Un utilisateur authentifié peut consulter l'historique d'une carte qu'il ne possède pas dans sa collection.
- [ ] `scryfall_id` correspondant à aucune carte en base → 404.
- [ ] `scryfall_id` correspondant à une carte sans `cardmarket_id`, ou sans aucune ligne de prix sur la période →
      200 avec un tableau vide.
- [ ] `start_date > end_date` → 400, avec un message d'erreur cohérent avec celui de `/cards/price-history`.
- [ ] Requête sans token d'authentification → 401.
- [ ] Le format de réponse (nom des champs, types, absence de wrapper) est strictement identique à celui de
      `/cards/price-history`, pour compatibilité directe avec le composant frontend de graphique existant.
- [ ] Le nouvel endpoint est documenté dans `doc/openapi.yml`.
