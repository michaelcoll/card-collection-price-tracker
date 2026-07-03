# Workflow d'Échange

## Vue d'ensemble

Échange carte-à-carte entre exactement 2 utilisateurs. Les différences de valeur sont réglées en espèces en dehors de la
plateforme. Les notations sont optionnelles et mutuelles.

---

## Acteurs

| Acteur             | Rôle                                                       |
|--------------------|------------------------------------------------------------|
| **Initiateur (A)** | Trouve une carte et demande l'échange                      |
| **Répondant (B)**  | Propriétaire de la carte demandée, propose sa contre-offre |

---

## Déroulement complet

### 1. Découverte

- L'utilisateur *A* recherche une carte par son nom.
- *A* trouve une carte appartenant à l'utilisateur B.

### 2. Demande d'échange (statut : `PENDING`)

- *A* envoie une demande d'échange pour la carte de B.
- Aucune carte de *A* n'est encore proposée — *A* dit simplement « je veux cette carte. »
- *B* reçoit une notification : **« Nouvelle demande d'échange reçue. »**

### 3. Contre-proposition

- *B* parcourt la collection de *A* et sélectionne 0 ou plusieurs cartes qui l'intéressent en échange.
- L'interface calcule la différence de valeur et affiche le montant en espèces qu'une des deux parties doit à l'autre.
- *B* soumet la contre-proposition. La transaction liste alors :
    - Cartes offertes par *B* → à *A*
    - Cartes offertes par *A* → à *B* (sélectionnées par B, ou aucune)
    - Delta en espèces (informatif uniquement — réglé en dehors de la plateforme)
- *A* reçoit une notification : **« *B* a fait une contre-proposition. »**

### 4. Négociation (`PENDING`)

- Chaque partie peut modifier la transaction (ajouter/retirer des cartes, ajuster) à tout moment **tant que le statut
  est `PENDING`**.
- Chaque modification déclenche une notification à l'autre partie.

### 5. Première acceptation (statut : `ONE_ACCEPTED`)

- Une des parties clique sur « Accepter. »
- Une modale avertit : **« Une fois acceptée, la transaction sera verrouillée. Si l'autre partie modifie la transaction,
  elle repassera en attente et devra être acceptée à nouveau. »**
- À la confirmation :
    - Statut → `ONE_ACCEPTED`.
    - Toutes les cartes des deux côtés de la transaction sont **réservées** (visible dans la collection de chaque
      utilisateur).
    - Toutes les autres transactions `PENDING` ou `ONE_ACCEPTED` impliquant ces mêmes cartes sont automatiquement *
      *abandonnées**.
    - Les parties concernées reçoivent une notification : **« Une transaction impliquant l'une de vos cartes réservées a
      été validée par un autre échange. Votre transaction a été annulée. »**
    - L'autre partie reçoit une notification : **« Votre partenaire a accepté. Acceptez à votre tour ou modifiez la
      transaction pour relancer la négociation. »**

### 5b. Modification après acceptation (retour à `PENDING`)

- Si l'une des parties modifie la transaction alors que le statut est `ONE_ACCEPTED` :
    - Statut → `PENDING`.
    - Les cartes réservées sont **libérées**.
    - Les deux acceptations précédentes sont annulées — les deux parties devront accepter à nouveau.
    - L'autre partie reçoit une notification : **« La transaction a été modifiée. Elle repasse en négociation. »**

### 6. Acceptation complète (statut : `FULLY_ACCEPTED`)

- La seconde partie accepte (même modale d'avertissement).
- Statut → `FULLY_ACCEPTED`.
- Les deux parties reçoivent une notification : **« Les deux parties ont accepté. Procédez à l'échange physique. »**

### 7. Échange physique

- Les cartes (et les espèces le cas échéant) sont échangées en personne, en dehors de la plateforme.

### 8. Confirmation de l'échange (statut : `COMPLETED`)

- Chaque partie confirme « Échange réalisé » dans l'application.
- Statut → `COMPLETED` une fois que **les deux** ont confirmé.
- Les deux parties reçoivent une notification : **« Échange confirmé par les deux parties. »**

### 9. Notation (optionnelle, statut : `CLOSED`)

- Chaque partie peut noter l'autre de 0 à 5 étoiles.
- La notation est optionnelle — la passer ne bloque pas la clôture.
- Une fois que les deux ont noté (ou passé), statut → `CLOSED`.

---

## Référence des statuts

```
PENDING ◄─────────────────────────────────────────────────────┐
  │  (une partie accepte)                                     │
  ▼                                                           │
ONE_ACCEPTED  ──(modification par l'une des parties)──────────┘
  │  (l'autre partie accepte)
  ▼
FULLY_ACCEPTED
  │  (les deux confirment l'échange physique)
  ▼
COMPLETED
  │  (les deux notent ou passent la notation)
  ▼
CLOSED

ABANDONED  ← accessible depuis n'importe quel statut avant COMPLETED
```

| Statut           | Description                                                                 |
|------------------|-----------------------------------------------------------------------------|
| `PENDING`        | Ouverte à la modification par les deux parties                              |
| `ONE_ACCEPTED`   | Une partie a accepté ; cartes réservées ; modifiable (repasse en `PENDING`) |
| `FULLY_ACCEPTED` | Les deux ont accepté ; en attente de l'échange physique                     |
| `COMPLETED`      | Échange physique confirmé par les deux parties                              |
| `CLOSED`         | Transaction terminée ; notations soumises ou passées                        |
| `ABANDONED`      | Transaction annulée par l'une des parties à tout moment avant `COMPLETED`   |

---

## Règles de réservation des cartes

- Les cartes sont réservées à l'étape `ONE_ACCEPTED`.
- Une carte réservée est visible comme telle dans la collection de son propriétaire (badge ou indicateur).
- Si la même carte apparaît dans plusieurs transactions simultanées :
    - La **première transaction à atteindre `ONE_ACCEPTED`** réserve la carte.
    - Toutes les autres transactions impliquant cette carte sont automatiquement `ABANDONED`.
    - Toutes les parties concernées sont notifiées immédiatement.

---

## Règles de modification

| Qui peut modifier | Quand                              | Effet sur le statut                  |
|-------------------|------------------------------------|--------------------------------------|
| L'une ou l'autre  | Statut `PENDING`                   | Reste `PENDING`                      |
| L'une ou l'autre  | Statut `ONE_ACCEPTED`              | Repasse à `PENDING`, cartes libérées |
| Personne          | Statut `FULLY_ACCEPTED` ou au-delà | Impossible                           |

Toute modification notifie l'autre partie.

---

## Règles d'abandon

- L'une ou l'autre des parties peut abandonner la transaction à tout moment avant `COMPLETED`.
- À l'abandon :
    - Statut → `ABANDONED`.
    - Les cartes réservées (le cas échéant) sont libérées.
    - L'autre partie reçoit une notification : **« L'échange a été abandonné par votre partenaire. »**
    - La partie abandonnant peut être notée par l'autre partie (optionnel, 0–5 étoiles).

---

## Récapitulatif des notifications

| Déclencheur                                          | Destinataire(s)               |
|------------------------------------------------------|-------------------------------|
| Demande d'échange envoyée                            | *B*                           |
| Contre-proposition soumise                           | *A*                           |
| Transaction modifiée (statut `PENDING`)              | L'autre partie                |
| Première acceptation (`ONE_ACCEPTED`)                | L'autre partie                |
| Modification après acceptation (retour à `PENDING`)  | L'autre partie                |
| Autres transactions rendues caduques                 | Toutes les parties concernées |
| Les deux ont accepté (`FULLY_ACCEPTED`)              | Les deux                      |
| Échange physique confirmé par les deux (`COMPLETED`) | Les deux                      |
| Transaction abandonnée                               | L'autre partie                |

---

## Hors périmètre (MVP)

- Paiement en ligne — le delta en espèces est informatif uniquement.
- Expiration des transactions — les transactions restent actives indéfiniment jusqu'à acceptation ou abandon.
- Historique des échanges — prévu post-MVP.
- Messagerie intégrée — pourra être ajoutée si jugée indispensable.
