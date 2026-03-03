# Contributing

Thank you for your interest in contributing to System Monitor!

## Architecture Overview

```
┌─────────────────────────────────────────────────────┐
│                   System Monitor                     │
├──────────────────────┬──────────────────────────────┤
│   React Frontend     │   Tauri v2 Backend (Rust)    │
│                      │                              │
│  TrayView.tsx        │  lib.rs     — Commands &     │
│   ├─ StatusBar.tsx   │               window mgmt    │
│   └─ Settings.tsx    │  monitor.rs — sysinfo crate  │
│                      │  notifier.rs— Alert logic    │
│  useSystemInfo.ts    │  config.rs  — Persistence    │
│   (polling hook)     │               & validation   │
├──────────────────────┴──────────────────────────────┤
│              OS (macOS / Windows / Linux)            │
└─────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────┐
│              sysmon-mcp (standalone)                  │
│  MCP JSON-RPC 2.0 over stdio                        │
│  Privacy-first: numeric metrics only                │
│  AI assistants (Claude Code, Cursor) connect here   │
└─────────────────────────────────────────────────────┘
```

**Data flow**: Frontend polls `get_system_info` via Tauri IPC every N seconds. Backend refreshes `sysinfo` crate data and returns a `SystemInfo` struct. Notifications are checked on each poll via `check_notifications`.

**MCP server** is a separate binary — it does not depend on the Tauri app. It can be installed and used independently.

## Development Setup

1. Install prerequisites listed in [README.md](README.md#prerequisites-for-building-from-source)
2. Fork and clone the repository
3. Run `pnpm install` to install dependencies
4. Run `pnpm tauri dev` to start development

## Commit Message Conventions

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add process list view
fix: correct memory calculation on Windows
docs: update MCP server examples
test: add notifier cooldown edge cases
chore: update sysinfo to 0.35
refactor: extract threshold logic into helper
```

**Scope** (optional): `feat(mcp): add get_network tool`

## Pull Request Process

1. Create a feature branch from `main`
2. Make your changes with clear, focused commits
3. Ensure all checks pass:
   - `cargo fmt --check` (Rust formatting)
   - `cargo clippy -- -D warnings` (Rust lints)
   - `cargo test` (Rust tests — both `src-tauri` and `sysmon-mcp`)
   - `npx tsc --noEmit` (TypeScript check)
   - `pnpm lint` (ESLint)
   - `pnpm test` (Frontend tests)
4. Submit a pull request describing your changes

## Testing

**Rust backend** (src-tauri): Unit tests in each module. Run with:

```bash
cd src-tauri && cargo test
```

**MCP server** (sysmon-mcp): Unit tests covering protocol, tools, errors. Run with:

```bash
cd sysmon-mcp && cargo test
```

**Frontend**: Vitest + Testing Library. Run with:

```bash
pnpm test
```

When adding new features, include tests. Aim for coverage of:

- Happy path
- Edge cases (zero values, max values, missing data)
- Error handling

## Code Style

- **Rust**: `cargo fmt` before committing. `unsafe` code is forbidden (`forbid(unsafe_code)`)
- **TypeScript**: Strict mode enabled. Follow existing patterns
- **CSS**: Inline styles in components (no separate CSS files)
- Keep changes minimal and focused — avoid unrelated refactoring in feature PRs

## Adding New Metrics

To add a new system metric (e.g., network bandwidth):

1. **Backend** (`src-tauri/src/monitor.rs`): Add field to `SystemInfo` struct, populate in `refresh_and_get()`
2. **Frontend** (`src/components/StatusBar.tsx`): Add a new `Bar` component for the metric
3. **MCP** (`sysmon-mcp/src/main.rs`): Add a new tool (e.g., `get_network`) and include in `get_metrics`
4. **Tests**: Add unit tests in all three locations
5. **Notifications** (`src-tauri/src/notifier.rs`): Add threshold if applicable, with config field in `config.rs`

## Reporting Issues

- Use [GitHub Issues](https://github.com/ryo-kozin/sysmon-tray/issues)
- Include your OS, app version, and steps to reproduce
- For security vulnerabilities, see [SECURITY.md](SECURITY.md)
