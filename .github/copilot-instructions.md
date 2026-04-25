# AI Agent Guide: Card Collection Price Tracker

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
