# Design System Strategy: The Arcane Shop

## 1. Overview & Creative North Star

**The Creative North Star: "The Digital Curator"**
This design system rejects the cluttered, "utility-first" aesthetic common in trading platforms. Instead, it adopts the
persona of a high-end editorial gallery. Magic: The Gathering is a game of incredible art and complex data; our goal is
to treat the cards like museum artifacts and the data like professional financial analytics.

We break the "template" look by utilizing **intentional asymmetry** and **tonal layering**. Large-scale typography (
Display-LG) creates a bold, editorial rhythm, while the interface itself recedes into the background using deep,
atmospheric purples and blues to let the vibrant card art "pop."

The system supports **two themes** derived from the same four seed colors ("Aether Grid" mockup):

| Theme                         | Activation                   | Character                                       |
|:------------------------------|:-----------------------------|:------------------------------------------------|
| 🌑 **Dark** — *Mystic Dark*   | `data-theme="dark"` (défaut) | Surfaces quasi-noires, accents néon-kinétiques  |
| ☀️ **Light** — *Arcane Light* | `data-theme="light"`         | Surfaces blanc chaud, accents saturés et riches |

---

## 2. Colors & Surface Philosophy

### Seed Colors — Source de Vérité Unique

Tous les tokens du système sont dérivés de ces quatre couleurs graines Material Design 3, telles que définies dans la
maquette "Aether Grid" :

| Rôle          | Seed      | Usage principal                              |
|:--------------|:----------|:---------------------------------------------|
| **Primary**   | `#7C4DFF` | Boutons CTA, états actifs, gradients         |
| **Secondary** | `#2196F3` | Badges de prix, accents secondaires          |
| **Tertiary**  | `#00E5FF` | Barres EDHREC, indicateurs "Trending Up"     |
| **Neutral**   | `#121212` | Dérivation de toute la hiérarchie de surface |

### The "No-Line" Rule

**Instruction explicite :** Ne jamais utiliser de bordures `1px solid`. Elles créent du bruit visuel qui distrait de
l'art des cartes.

- Définir les délimitations par des changements de fond : placer un élément `surface-container-high` sur un fond
  `surface`.
- Utiliser l'**échelle d'espacement** (`gap-8` ou `gap-10`) pour créer une séparation "par le vide".

### Surface Hierarchy & Nesting

Traiter l'UI comme des couches physiques de verre dépoli.

**Dark mode :**

| Token                       | Valeur    | Rôle                                          |
|:----------------------------|:----------|:----------------------------------------------|
| `surface-container-lowest`  | `#0e0e0e` | Élément le plus enfoncé (zebra, sunken cards) |
| `surface`                   | `#131313` | Fond principal de l'application               |
| `surface-container-low`     | `#1c1b1b` | Sidebar, navigation secondaire                |
| `surface-container`         | `#211f26` | Container par défaut, base glassmorphique     |
| `surface-container-high`    | `#2a2a2a` | Cards élevées, panneaux                       |
| `surface-container-highest` | `#353534` | Modales, panneau de détail actif              |
| `surface-bright`            | `#393939` | Cible du hover sur les grilles de cartes      |
| `surface-variant`           | `#49454f` | Fond des tooltips (semi-transparent)          |

**Light mode :**

| Token                       | Valeur    | Rôle                                           |
|:----------------------------|:----------|:-----------------------------------------------|
| `surface-container-lowest`  | `#ffffff` | Blanc pur pour les éléments les plus encastrés |
| `surface`                   | `#fdfbf9` | Fond principal (blanc chaud, légèrement beige) |
| `surface-container-low`     | `#f7f2f0` | Sidebar, navigation secondaire                 |
| `surface-container`         | `#f1ece9` | Container par défaut                           |
| `surface-container-high`    | `#ebe6e3` | Cards élevées, panneaux                        |
| `surface-container-highest` | `#e5e1de` | Modales, panneau de détail actif               |
| `surface-bright`            | `#ffffff` | Cible du hover (blanc pur sur fond chaud)      |
| `surface-variant`           | `#e8e3e0` | Fond des tooltips                              |

### The "Glass & Gradient" Rule

Pour le côté "MTG Flavor", utiliser un dégradé subtil sur les CTAs primaires :

- `linear-gradient(135deg, var(--primary), var(--primary-container))`
- Pour les widgets flottants "Price Insight" : `backdrop-filter: blur(12px)` avec 60% d'opacité sur `surface-container`
  pour un effet glassmorphique premium.

---

## 3. Typography: The Editorial Voice

Police **Inter** exclusivement, avec manipulation du poids et du tracking pour créer de l'autorité.

| Échelle       | Poids | Tracking  | Usage                                     |
|:--------------|:------|:----------|:------------------------------------------|
| Display LG/MD | 500   | `-0.02em` | Noms de sets, prix high-value             |
| Headline SM   | 700   | `0`       | Noms de cartes dans les grilles           |
| Label MD/SM   | 500   | `0`       | "EDHREC %", "Prix" — `on-surface-variant` |
| Body LG       | 400   | `0`       | Texte/flavor text, `line-height: 1.6`     |

---

## 4. Elevation & Depth

Hiérarchie par **Tonal Layering**, jamais par des ombres structurelles.

- **Principe de superposition :** Placer un élément `surface-container-lowest` à l'intérieur d'une section
  `surface-container-high` pour créer un look "enfoncé" ou "encastré".
- **Ombres ambiantes :** Pour les aperçus de cartes flottants :
  `0 24px 48px -12px rgba(0, 0, 0, 0.45)`. La couleur d'ombre doit être teintée, jamais du noir pur.
- **"Ghost Border" (fallback accessibilité) :** Si un séparateur est obligatoire, utiliser `outline-variant` à **15%
  d'opacité**. Il doit se sentir, pas se voir.

---

## 5. Components

### Card Grids

- **0 bordure :** Utiliser `rounded-lg` (0.5rem). Zéro `border`.
- **Hover :** `scale(1.02)` + passage du fond vers `surface-bright`. Pas de glow.
- **Image :** Inner-shadow subtil pour que l'art semble "enchâssé" dans le cadre numérique.

### Progress Bars & Badges

- **Barre EDHREC %** : fill `tertiary`, track `surface-container-highest`, `height: 4px`.
- **Badges de prix** : fond `secondary-container`, texte `on-secondary-container`, forme pill `rounded-full`.

### Comparison Tables

- **Zéro séparateur :** Zebra-striping avec `surface-container-low` / `surface-container-lowest`.
- **Alignement des données :** Alignement à droite pour les prix, `label-md` pour "Market", `title-sm` pour la valeur.

### Buttons — 4 variantes (maquette "Aether Grid")

| Variante      | Fond                     | Texte / Bordure                 | Usage                       |
|:--------------|:-------------------------|:--------------------------------|:----------------------------|
| **Primary**   | `primary`                | `on-primary`                    | CTA principal, états actifs |
| **Secondary** | `surface-container-high` | `on-surface`                    | Actions de support          |
| **Inverted**  | `on-surface`             | `surface`                       | Contraste inversé           |
| **Outlined**  | Transparent              | `outline-variant` (15% opacity) | Actions peu accentuées      |

- **Google Login :** `surface-container-high` + ghost border 10% `outline-variant`. Icône Google G monochrome.

### Icon Buttons — 3 rôles sémantiques

| Rôle         | Fond                 | Icône                   | Usage                |
|:-------------|:---------------------|:------------------------|:---------------------|
| **Tertiary** | `tertiary-container` | `on-tertiary-container` | Édition / actions +  |
| **Primary**  | `primary-container`  | `on-primary-container`  | Catégoriser / taguer |
| **Error**    | `error-container`    | `on-error-container`    | Supprimer / détruire |

---

## 6. Theming Strategy & Implementation

### Mécanisme

Le thème est piloté par l'attribut `data-theme` sur `<html>` :

```html

<html data-theme="dark">  <!-- ou "light" -->
```

Dark est le **défaut**. En l'absence de `data-theme`, le système respecte `prefers-color-scheme` (media query
fallback dans `styles.css`).

### Angular Service Pattern

```typescript
// theme.service.ts
@Injectable({providedIn: 'root'})
export class ThemeService {
    private theme = signal<'dark' | 'light'>('dark');

    readonly current = this.theme.asReadonly();

    toggle(): void {
        const next = this.theme() === 'dark' ? 'light' : 'dark';
        this.theme.set(next);
        document.documentElement.setAttribute('data-theme', next);
    }
}
```

### CSS Implementation (Tailwind v4)

Tous les tokens de couleur sont des CSS custom properties déclarées dans `:root` (dark) et `[data-theme="light"]`.
Tailwind les mappe via `@theme inline` dans `styles.css` :

```css
@theme inline {
    --color-primary: var(--primary);
    --color-surface: var(--surface);
    /* ... */
}
```

Cela permet aux classes `bg-primary`, `text-on-surface`, `bg-surface-container-high` de répondre aux changements
de thème à l'exécution **sans rebuild**.

---

## 7. Do's and Don'ts

### Do

- **DO** utiliser `spacing-16` (3.5rem) comme marge minimale pour les sections hero. Laisser respirer les données.
- **DO** utiliser `tertiary` avec parcimonie pour "Success" ou "Trending Up".
- **DO** utiliser des couches `surface-variant` semi-transparentes pour les tooltips (glassmorphisme).
- **DO** aligner les données de prix à droite.

### Don't

- **DON'T** utiliser du blanc pur sur fond sombre. Toujours `on-surface` (`#e5e2e1` dark / `#1c1b1f` light).
- **DON'T** utiliser des séparateurs gris 1px. Utiliser `gap-8` (1.8rem) à la place.
- **DON'T** utiliser des `box-shadow` sur les cartes dans une grille. Utiliser les décalages de couleur de fond.
- **DON'T** utiliser `border: 1px solid` pour quelque raison que ce soit.

---

## 8. Complete Token Reference

### Color Roles

| Token                    | Dark      | Light     | Usage                                  |
|:-------------------------|:----------|:----------|:---------------------------------------|
| `primary`                | `#cdbdff` | `#7c4dff` | Boutons CTA, états actifs              |
| `on-primary`             | `#370096` | `#ffffff` | Texte sur fond primary                 |
| `primary-container`      | `#4f2da7` | `#eedcff` | Fin gradient CTA, fond icon-button     |
| `on-primary-container`   | `#e9deff` | `#220066` | Icône/texte sur primary-container      |
| `secondary`              | `#a8c7fa` | `#1565c0` | Accents secondaires, textes de support |
| `on-secondary`           | `#003062` | `#ffffff` | Texte sur fond secondary               |
| `secondary-container`    | `#004a77` | `#d3e4ff` | Fond des badges de prix                |
| `on-secondary-container` | `#d3e4ff` | `#001c38` | Texte des badges de prix               |
| `tertiary`               | `#00e5ff` | `#006874` | Barres EDHREC, indicateurs tendance    |
| `on-tertiary`            | `#003545` | `#ffffff` | Texte sur fond tertiary                |
| `tertiary-container`     | `#004f57` | `#97f0ff` | Fond icon-button édition               |
| `on-tertiary-container`  | `#a8eeff` | `#001f24` | Icône icon-button édition              |
| `error`                  | `#cf6679` | `#b3261e` | Texte d'erreur, états destructifs      |
| `on-error`               | `#601410` | `#ffffff` | Texte sur fond error                   |
| `error-container`        | `#8c1d18` | `#f9dedc` | Fond icon-button suppression           |
| `on-error-container`     | `#f9dedc` | `#410002` | Icône icon-button suppression          |

### Surface Tokens

| Token                       | Dark      | Light     | Usage                              |
|:----------------------------|:----------|:----------|:-----------------------------------|
| `surface`                   | `#131313` | `#fdfbf9` | Fond global de l'application       |
| `surface-bright`            | `#393939` | `#ffffff` | Cible du hover sur les grilles     |
| `surface-variant`           | `#49454f` | `#e8e3e0` | Fond tooltips (semi-transparent)   |
| `on-surface`                | `#e5e2e1` | `#1c1b1f` | Texte principal                    |
| `on-surface-variant`        | `#cbc3d9` | `#49454f` | Texte métadonnées / labels         |
| `outline-variant`           | `#494456` | `#cac4d0` | Ghost borders (max 15% opacity)    |
| `surface-container-lowest`  | `#0e0e0e` | `#ffffff` | Élément le plus enfoncé            |
| `surface-container-low`     | `#1c1b1b` | `#f7f2f0` | Sidebar, zebra-stripe              |
| `surface-container`         | `#211f26` | `#f1ece9` | Base widget glassmorphique         |
| `surface-container-high`    | `#2a2a2a` | `#ebe6e3` | Cards élevées, boutons Secondary   |
| `surface-container-highest` | `#353534` | `#e5e1de` | Modales, panneaux de détail actifs |

### Spacing Scale

| Classe Tailwind  | Valeur    | Usage                                   |
|:-----------------|:----------|:----------------------------------------|
| `p-4` / `gap-4`  | `0.9rem`  | Padding interne des cards               |
| `gap-8`          | `1.8rem`  | Séparation "par le vide" (no-line rule) |
| `gap-10`         | `2.25rem` | Séparation entre sections               |
| `mt-16` / `p-16` | `3.5rem`  | Marge minimum pour les sections hero    |

### Border Radius

| Classe Tailwind | Valeur   | Usage                     |
|:----------------|:---------|:--------------------------|
| `rounded-lg`    | `0.5rem` | Cards et containers       |
| `rounded-full`  | `9999px` | Badges pill, icon-buttons |

