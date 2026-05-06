# Référence des Endpoints API

L'application expose une API REST via **Axum**. Les routes sont définies dans
`src/ccpt/infrastructure/adapter_in/` et montées dans `src/ccpt/infrastructure.rs`.

---

## Préfixes de routes

| Préfixe        | Contrôleur                    | Fichier source                         |
|----------------|-------------------------------|----------------------------------------|
| `/cards`       | `create_card_router()`        | `adapter_in/card_controller.rs`        |
| `/maintenance` | `create_maintenance_router()` | `adapter_in/maintenance_controller.rs` |

---

## Groupe `/cards`

> Tous les endpoints de ce groupe requièrent un **Bearer Token Clerk** (voir
> [authentication.instructions.md](authentication.instructions.md)).

### `GET /cards/`

Récupère la collection paginée et triable de l'utilisateur authentifié.

**Query params :**

| Paramètre   | Type     | Défaut   | Valeurs possibles                                 | Description                      |
|-------------|----------|----------|---------------------------------------------------|----------------------------------|
| `page`      | `u32`    | `0`      | ≥ 0                                               | Numéro de page (commence à 0)    |
| `page_size` | `u32`    | `20`     | 1 – 100 (plafonné à 100 côté serveur)             | Nombre de cartes par page        |
| `sort_by`   | `enum`   | `trend`  | `avg` \| `trend` \| `set_code` \| `language_code` | Champ de tri                     |
| `sort_dir`  | `enum`   | `desc`   | `asc` \| `desc`                                   | Direction du tri                 |
| `q`         | `string` | *(vide)* | texte libre                                       | Recherche floue sur le nom / set |

**Réponse `200 OK` :** `PaginatedCollection`

```json
{
  "items": [
    {
      "set_code": "FDN",
      "collector_number": "87",
      "language_code": "fr",
      "foil": false,
      "name": "Goblin Boarders",
      "rarity_code": "C",
      "scryfall_id": "4409a063-...",
      "quantity": 3,
      "purchase_price": 8,
      "price_guide": {
        "low": 5,
        "avg": 10,
        "trend": 9,
        "avg1": 8,
        "avg7": 9,
        "avg30": 11
      }
    }
  ],
  "total": 142,
  "page": 0,
  "page_size": 20
}
```

> `price_guide` peut être `null` si aucun prix n'est encore disponible pour la carte.  
> Les prix sont exprimés en **centimes d'euro**.

---

### `POST /cards/import`

Importe une collection depuis un export CSV **ManaBox**.

**Body :** `text/plain` — contenu CSV ManaBox (taille max : 10 Mo)

**Réponse `200 OK` :** `Message`

```json
{
  "message": "Cards imported successfully"
}
```

**Erreurs possibles :**

| Code  | Cause                                |
|-------|--------------------------------------|
| `400` | Corps non valide (UTF-8 invalide, …) |
| `401` | Token absent ou invalide             |

---

### `POST /cards/card-info`

Récupère les informations d'une carte via **EDHRec** (endpoint en cours de développement).

**Body :** aucun

**Réponse `200 OK` :** `string` (prototype, sujet à changement)

---

## Groupe `/maintenance`

> Ces endpoints sont **publics** (aucune authentification requise).

### `GET /maintenance/stats`

Renvoie les statistiques globales de la base de données.

**Réponse `200 OK` :** `StatsResponse`

```json
{
  "card_number": 1250,
  "card_price_number": 4800,
  "db_size_mb": 42
}
```

---

### `POST /maintenance/trigger-price-update`

Déclenche manuellement la mise à jour des prix CardMarket (normalement planifiée toutes les 12 h).

**Body :** aucun

**Réponse `204 No Content`** en cas de succès.

---

### `POST /maintenance/update-cardmarket-ids`

Met en file d'attente asynchrone la résolution des IDs CardMarket manquants pour les cartes de la
collection.

**Body :** aucun

**Réponse `202 Accepted` :** `EnqueueResponse`

```json
{
  "enqueued": 37
}
```

> `enqueued` indique le nombre de cartes effectivement ajoutées à la file (les doublons sont
> dédupliqués automatiquement).

---

## Ajouter un nouvel endpoint

1. Créer le handler dans le contrôleur approprié (ou un nouveau fichier `*_controller.rs`).
2. L'enregistrer dans la fonction `create_*_router()` correspondante.
3. Si le nouveau groupe de routes est dans un nouveau fichier, le monter dans `create_infra()` via
   `.nest("/prefix", create_new_router())`.
4. Protéger l'endpoint si nécessaire avec `AuthenticatedUser` (voir
   [authentication.instructions.md](authentication.instructions.md)).

