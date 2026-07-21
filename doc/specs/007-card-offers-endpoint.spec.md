# Spec : Liste des offres de vente d'une carte (`GET /card/offers`) et renommage `/cards` → `/card`

## Contexte

Depuis l'extraction de `GET /cards` vers `GET /collection` ([006](006-extract-collection-endpoint.spec.md)), le
controller `card` (monté sur la base URI `/cards`) n'expose plus que des endpoints qui travaillent sur **une seule
carte** : `POST /cards/card-info` et `GET /cards/{scryfall_id}/price-history`. Le nom au pluriel `/cards` ne
correspond plus à cet usage.

Par ailleurs, un joueur qui trouve une carte qui l'intéresse n'a aujourd'hui aucun moyen direct de savoir quels
autres joueurs la possèdent et pourraient l'échanger, en dehors du catalogue paginé `GET /collection?owned=false`
(qui liste toutes les cartes de tous les joueurs, pas une carte précise identifiée par ses attributs).

## Objectif

- Renommer la base URI du controller `card` : `/cards` → `/card`.
- Ajouter `GET /card/offers` : pour une carte précise, identifiée par les 4 attributs de `CardId` (`set_code`,
  `collector_number`, `language_code`, `foil`) passés en query params tous obligatoires, retourner la liste des
  autres joueurs qui la possèdent, avec la quantité qu'ils détiennent et un prix de vente.

## Solution

### Renommage `/cards` → `/card`

- `infrastructure.rs` : `.nest("/cards", create_card_router())` devient `.nest("/card", create_card_router())`.
- Tag OpenAPI `"cards"` renommé `"card"` (déclaration dans `openapi.rs` + `tag = "..."` sur chaque
  `#[utoipa::path]` du controller `card`), et `path = "/cards/..."` mis à jour en `path = "/card/..."` pour les
  endpoints existants (`card-info`, `{scryfall_id}/price-history`).
- `doc/openapi.yml` régénéré.
- Frontend `useCardsService.ts` : les deux appels (`getCardInfo`, `getCardPriceHistory`) pointent vers
  `/card/card-info` et `/card/{scryfall_id}/price-history`.
- Aucun alias de compatibilité : `GET /cards*` doit disparaître (404) une fois le renommage fait, comme pour
  l'extraction de la spec 006.

### `GET /card/offers`

- Nouvelle route dans `create_card_router()` (`adapter_in/card/controller.rs`), authentifiée (bearer), sur le
  modèle des autres endpoints du controller.
- Query params, **tous obligatoires** : `set_code`, `collector_number`, `language_code`, `foil` — les 4 attributs
  de `domain::card::CardId`. Réutilise la validation existante de `CardId::try_new` (ex. `collector_number` ≤ 10
  caractères, `language_code`/`set_code` déjà validés par leurs types domaine respectifs).
- Résolution de la carte : identifie la ou les lignes correspondant à ce `CardId` (même source de données que le
  catalogue, `mv_card_prices`) — si aucune ligne ne correspond, la carte n'existe pas.
- Retourne, paginé comme `GET /collection` (`page`, `page_size`, max 100), la liste des utilisateurs **autres que
  l'utilisateur authentifié** qui possèdent cette carte. Une entrée par vendeur (pas une entrée par exemplaire),
  avec la quantité totale qu'il détient — même agrégation que `mv_card_prices`/le catalogue.
- Chaque entrée inclut un champ `selling_price` (prix de vente, en centimes). Pour cette première version, le
  back-end fixe systématiquement `selling_price` à la valeur `trend` du price guide de la carte — il n'y a pas
  encore de prix personnalisé par vendeur. Ce champ est prévu pour accueillir plus tard des règles de tarification
  plus avancées (par vendeur, par état de la carte, etc.) sans changer la forme de la réponse ; ces règles futures
  sont hors périmètre de cette spec.
- Query param optionnel `sort_by` : contrôle le champ de tri. Pour cette première version, seule la valeur
  `selling_price` est acceptée (tri du moins cher au plus cher). Absent, il vaut `selling_price` par défaut. Ce
  paramètre est prévu pour accueillir plus tard d'autres valeurs de tri ; pas de `sort_dir` pour l'instant, la
  direction reste fixe (ascendante).
- Aucune nouvelle notion de "vendeur" n'est persistée séparément de la possession en collection : réutilise
  l'infrastructure existante (`mv_card_prices`, jointure `users`), sur le modèle de
  `CardPricesViewRepositoryAdapter::get_paginated`.

### Modèle domaine partagé : `CollectionEntry`

- Pas de structure dédiée pour représenter une offre : `owner_username`, `quantity` et `selling_price` rejoignent
  `domain::card::CollectionEntry`, la structure qui représente déjà "la relation d'un utilisateur à une carte"
  côté `GET /collection`.
  - `selling_price: Option<u32>` et `quantity: u8` sont ajoutés à la variante `Owned` uniquement — dérivés
    respectivement du `trend` du price guide et de la quantité possédée, au moment de la lecture. `Mine` n'est
    **pas** modifiée : rien ne consomme aujourd'hui un `selling_price` pour ses propres cartes (ni `/collection`,
    ni `/card/offers`, qui exclut toujours l'utilisateur courant) — inutile de porter un champ qui ne serait
    jamais lu.
  - Ajouter `quantity` à `Owned` lève, au niveau du modèle domaine et de la requête SQL de `GET /collection` (vue
    `mv_card_prices`), le masquage de la quantité pour les cartes appartenant à d'autres utilisateurs — masquage
    initialement décrit par la spec [003](003-cards-owned-filter.spec.md). `purchase_price` et `added_at` restent
    masqués pour les autres utilisateurs (seules données réellement privées) : c'est ce couple, et non plus la
    présence de `quantity`, qui permet de distinguer `Mine`/`Owned` à la lecture.
- **Portée volontairement limitée à cette itération** : ce déblocage reste interne au domaine. La réponse HTTP de
  `GET /collection` (`CollectionCardResponse`) et le frontend (page collection) ne sont **pas modifiés** — ils
  continuent d'ignorer `quantity`/`selling_price` sur les entrées `Owned`, exactement comme aujourd'hui. Seul
  `GET /card/offers` les expose, via son propre DTO de réponse.
- `GET /card/offers` construit ses entrées de réponse directement à partir de `CollectionEntry::Owned` (puisque
  l'utilisateur courant est systématiquement exclu de la requête, seules des entrées `Owned` sont produites).

## Cas d'erreurs

- Un des 4 query params est manquant → 400.
- Un query param est invalide (ex. `collector_number` > 10 caractères, `foil` non booléen, `language_code`
  inconnu) → 400.
- `sort_by` avec une valeur autre que `selling_price` → 400.
- Pas de token d'authentification → 401.
- Aucune carte ne correspond au `CardId` fourni (aucune ligne dans `mv_card_prices`) → 404
  (`FunctionalError::CardNotFound`).
- Carte existante mais aucun autre utilisateur ne la possède → 200, liste vide, `total: 0` (pas 404).
- `GET /cards*` (anciennes routes) → 404 une fois le renommage fait.
- `POST /card/card-info` et `GET /card/{scryfall_id}/price-history` ne présentent aucune régression de
  comportement — seule l'URI change.

## Critères d'acceptance

- [ ] `GET /card/offers` avec un `CardId` valide possédé par au moins un autre utilisateur retourne 200 et une
      liste paginée d'offres.
- [ ] Chaque offre contient `owner_username`, `quantity`, `selling_price` (égal au `trend` price de la carte).
- [ ] L'utilisateur authentifié n'apparaît jamais dans sa propre liste d'offres, même s'il possède la carte
      demandée.
- [ ] `GET /card/offers` pour un `CardId` qui ne correspond à aucune carte existante → 404.
- [ ] `GET /card/offers` pour une carte existante sans autre possesseur → 200, liste vide, `total: 0`.
- [ ] Un des 4 query params obligatoires manquant → 400.
- [ ] Un query param invalide (ex. `collector_number` trop long, `foil` non booléen) → 400.
- [ ] Sans `sort_by`, le tri par défaut est `selling_price` ascendant.
- [ ] `sort_by=selling_price` retourne les offres triées par `selling_price` croissant.
- [ ] `sort_by` avec une valeur autre que `selling_price` → 400.
- [ ] Sans token d'authentification → 401.
- [ ] La pagination (`page`, `page_size`, max 100) fonctionne comme sur `GET /collection`.
- [ ] `GET /cards*` (anciennes routes) → 404.
- [ ] `POST /card/card-info` et `GET /card/{scryfall_id}/price-history` continuent de fonctionner sans régression
      de comportement.
- [ ] `GET /card/offers` est documenté dans `doc/openapi.yml` sous le tag `"card"`.
- [ ] Le tag OpenAPI `"cards"` est renommé `"card"`, appliqué à tous les endpoints du controller `card`.
- [ ] `frontend-vue/app/composables/useCardsService.ts` appelle `/card/card-info` et
      `/card/{scryfall_id}/price-history`.
- [ ] `GET /collection` retourne exactement la même réponse JSON qu'avant cette spec pour les cartes appartenant à
      d'autres utilisateurs (`quantity`/`selling_price` non exposés dans `CollectionCardResponse`), même si le
      modèle domaine les porte désormais en interne.
- [ ] `mise run lint-backend` et `mise run format` passent sans erreur.
