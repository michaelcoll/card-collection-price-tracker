---
applyTo: "frontend/**"
---

## Frontend Development

```bash
cd frontend
rtk pnpm install    # Uses pnpm workspace (see pnpm-workspace.yaml)
rtk ng serve        # Angular dev server
rtk ng test         # Vitest test runner
rtk ng build        # Production build
```

## Frontend Design System

⚠️ **MANDATORY**: Every time you work on any frontend file (Angular components, styles, templates), you **MUST**
strictly follow the design system defined in [
`.github/instructions/design-system.instructions.md`](design-system.instructions.md). No exceptions.

### Tailwind CSS — règle absolue

⚠️ **MANDATORY**: Tailwind CSS v4 est le seul système de style autorisé pour tous les composants Angular.

- **Aucun fichier `.component.css`** ne doit être créé pour les composants métier ordinaires — pas de `styleUrl` /
  `styleUrls`
- **Toutes les règles de style** s'écrivent en classes utilitaires Tailwind directement dans le template HTML
- **Exceptions autorisées** via fichier `.component.css` dédié (ne jamais les mettre dans `styles.css` global) :
    - Styles de bibliothèques tierces ciblant des classes internes (ex. Clerk `.cl-*`) → `login-dialog.component.css`
    - Animations `@keyframes` et états complexes de composants utilitaires (ex. toast) → `toast-container.component.css`
- **États de navigation actifs** (`::after`, pseudo-éléments, `.mobile-nav-pill`) → `styles.css` global

#### Tokens disponibles (mappés dans `styles.css` via `@theme inline`)

| Besoin           | Classe Tailwind                                                                     |
|------------------|-------------------------------------------------------------------------------------|
| Fond principal   | `bg-surface`                                                                        |
| Conteneurs       | `bg-surface-container`, `bg-surface-container-high`, `bg-surface-container-highest` |
| Texte principal  | `text-on-surface`                                                                   |
| Texte secondaire | `text-on-surface-variant`                                                           |
| Bouton primaire  | `bg-primary text-on-primary`                                                        |
| Badge foil ⭑     | `bg-foil-container text-on-foil-container`                                          |
| Badge prix ↑     | `bg-tertiary-container/25 text-tertiary`                                            |
| Badge prix ↓     | `bg-error-container/25 text-error`                                                  |
| Toast success    | `bg-secondary-container text-on-secondary-container`                                |
| Barre EDHREC     | `bg-tertiary`                                                                       |
| Avatar initiale  | `bg-primary-container text-on-primary-container`                                    |
| Police           | `font-sans` (Inter)                                                                 |
| Arrondi pill     | `rounded-full`                                                                      |
| Arrondi bouton   | `rounded-lg`                                                                        |

#### Checklist avant chaque commit frontend

```bash
# Seuls les fichiers CSS de composants autorisés doivent exister (login-dialog, toast-container, app)
find frontend/src/app -name "*.component.css"

# Aucun styleUrl dans les composants métier hors exceptions
grep -r "styleUrl" frontend/src/app

# Build sans erreur
cd frontend && rtk pnpm run build
```

### Key rules at a glance (read DESIGN.md for full details)

| Rule              | Constraint                                                                                          |
|-------------------|-----------------------------------------------------------------------------------------------------|
| **Colors**        | Dark "Mystic Dark" palette — `surface` (#131313) as base, `primary` (#cdbdff), `tertiary` (#00e5ff) |
| **No borders**    | Never use 1px solid borders. Use background-color shifts and spacing instead                        |
| **Typography**    | Inter font only. Display-LG with `-0.02em` tracking for headlines                                   |
| **Elevation**     | Tonal layering (background shifts), not drop shadows                                                |
| **Buttons**       | Primary = `primary` bg + `on_primary` text; Google Login = monochrome on `surface_container_high`   |
| **EDHREC bar**    | `tertiary` fill, 4px height, `surface_container_highest` track                                      |
| **Price badges**  | Foil ⭑ = `foil-container`; trending ↑ = `tertiary-container/25`; trending ↓ = `error-container/25`  |
| **Text color**    | Never pure white — always `on_surface` (#e5e2e1)                                                    |
| **Card hover**    | `scale(1.02)` + shift to `surface_bright` (#393939). No glow                                        |
| **Glassmorphism** | `backdrop-filter: blur(12px)` + 60% opacity on floating widgets                                     |

