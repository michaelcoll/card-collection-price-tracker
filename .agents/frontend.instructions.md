---
applyTo: "frontend/**"
---

# Frontend Development Guide (Angular/Tailwind)

## Design System & Styling (CRITICAL)

- **Absolute Rule**: Use Tailwind CSS v4. No `.component.css` files for ordinary business components (only use the HTML
  template).
- **CSS Exceptions**: `.component.css` is only for third-party library styles or complex animations (`@keyframes`).
- **Tokens**: Use Tailwind classes mapped in `styles.css` to ensure theme consistency (e.g., `bg-surface`,
  `text-on-surface`).
    * **Key Rules (See also `design-system.instructions.md` for details):**
        * **Palette**: "Mystic Dark" theme (`surface` as base, `primary`, `tertiary`).
        * **No Borders**: Use background color changes and spacing to define boundaries.
        * **Typography**: Inter font only. Use `Display-LG` with specific tracking for titles.
        * **Elevation**: Based on the tonal shift of the background, not drop shadows.
        * **Buttons**: Primary = `primary` bg + `on_primary` text.
        * **Badges**: `foil-container` (⭑), `tertiary-container/25` (↑), `error-container/25` (↓).
        * **Text Color**: Always `on_surface` (#e5e2e1), never pure white.
        * **Interactions**: Card hover = `scale(1.02)` + `surface_bright` background.

**For complete details, refer to [design-system.instructions.md](design-system.instructions.md).**