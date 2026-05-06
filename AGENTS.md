# AI Agent Guide: Card Collection Price Tracker

## IDE Tools vs Shell Commands

**Always prefer IDE tools** over shell commands for reading and exploring the codebase:

| Task                                      | Use                          |
|-------------------------------------------|------------------------------|
| Read a file                               | `read_file`                  |
| List directory contents                   | `list_dir`                   |
| Search for a file by name/pattern         | `file_search`                |
| Search for text/symbols in code           | `grep_search`                |
| Understand code by meaning                | `semantic_search`            |
| Check compile/lint errors                 | `get_errors`                 |

Only use the **terminal** (`run_in_terminal`) when strictly necessary:
- Building / compiling (`cargo build`, `just test`, …)
- Installing dependencies (`pnpm install`, `cargo install`, …)
- Running migrations or git commands
- Starting background processes (dev server, …)

> ⚠️ **Never** use `ls`, `find`, `cat`, `grep` or similar shell commands to explore the codebase.
> These tasks **must** be done with the IDE tools above (`list_dir`, `file_search`, `read_file`, `grep_search`).
> Using the terminal for exploration wastes tokens and bypasses IDE indexing.

---

## RTK — Token-Optimized CLI

**rtk** is a CLI proxy that filters and compresses command outputs, saving 60-90% tokens.

### Rule

Always prefix shell commands with `rtk`:

```bash
# Instead of:              Use:
git status                 rtk git status
git log -10                rtk git log -10
cargo test                 rtk cargo test
docker ps                  rtk docker ps
kubectl get pods           rtk kubectl get pods
```

⚠️ Important : ⚠️

When using `rtk` tool, the output is compressed and token-efficient, but the command still behaves as expected. This
allows you to save tokens when sharing command outputs with AI models, while still getting the information you need. No
need to pipe through `head` or `tail` to limit output, just run the command with `rtk` and it will handle the rest.

### Meta commands (use directly)

```bash
rtk gain              # Token savings dashboard
rtk gain --history    # Per-command savings history
rtk discover          # Find missed rtk opportunities
rtk proxy <cmd>       # Run raw (no filtering) but track usage
```

---

## Architecture Overview

This is a **Clean Architecture** Rust application with an Angular frontend for tracking Magic: The Gathering card
prices. The backend follows Domain-Driven Design with strict layer separation:

- **Domain** (`src/ccpt/domain/`) - Core business entities (`Card`, `Price`, `LanguageCode`, etc.) with no external
  dependencies
- **Application** (`src/ccpt/application/`) - Use cases, services, and repository/caller interfaces
- **Infrastructure** (`src/ccpt/infrastructure/`) - External integrations (databases, APIs) via adapter pattern

Key architectural constraint: **All dependencies point inward**. Domain never imports from application/infrastructure
layers.

## Key Domain Concepts

Cards are uniquely identified by `CardId`: `set_code + collector_number + language_code + foil` (see `domain/card.rs`).
The system integrates with:

- **CardMarket**: Price data via JSON downloads (not authenticated API)
- **Scryfall**: Card metadata and IDs
- **EDHRec**: Commander format analytics

# Other instructions

- **Authentification** : [authentication.instructions.md](ai-instructions/authentication.instructions.md)
- **Backend** : [backend.instructions.md](ai-instructions/backend.instructions.md)
- **Frontend** : [frontend.instructions.md](ai-instructions/frontend.instructions.md)
- **Design System** : [design-system.instructions.md](ai-instructions/design-system.instructions.md)
- **Endpoints API** : [endpoints.instructions.md](ai-instructions/endpoints.instructions.md)
