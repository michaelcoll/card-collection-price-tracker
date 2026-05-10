# AI Agent Guide: Card Collection Price Tracker

## Codebase Exploration Tools (Prefer these over shell commands)

- **Read file:** `read_file`
- **List directory:** `list_dir`
- **Search by name/pattern:** `file_search`
- **Search content (regex):** `grep_search`
- **Understand code:** `semantic_search`
- **Check errors:** `get_errors`

**Terminal (`run_in_terminal`) use cases (Strictly necessary):**

- Building/compiling (`cargo build`, `just test`)
- Dependency installation (`pnpm install`)
- Git commands, migrations.

> ⚠️ **Avoid** `ls`, `find`, `cat`, `grep` in the terminal for codebase exploration.

## RTK — Token-Optimized CLI

**rtk** prefixes shell commands to compress output (e.g., `rtk git status`).

### Meta Commands

- `rtk gain`: Token savings dashboard.
- `rtk discover`: Find missed `rtk` opportunities.

## Architecture Overview

Clean Architecture (Rust/Angular) with strict layer separation: Domain, Application, Infrastructure. Dependencies point
inward.

## Key Concepts

- **CardId:** `set_code + collector_number + language_code + foil` (see `domain/card.rs`).
- **Integrations:** CardMarket, Scryfall, EDHRec.

## Instructions

- **Authentification** : [authentication.instructions.md](ai-instructions/authentication.instructions.md)
- **Backend** : [backend.instructions.md](ai-instructions/backend.instructions.md)
- **Frontend** : [frontend.instructions.md](ai-instructions/frontend.instructions.md)
- **Design System** : [design-system.instructions.md](ai-instructions/design-system.instructions.md)
- **Endpoints API** : [endpoints.instructions.md](ai-instructions/endpoints.instructions.md)