# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/), and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added

- `sysmon-mcp` — privacy-first MCP server exposing system metrics to AI assistants
- Unit tests for Rust backend (config, notifier, monitor) — 19 tests
- Unit tests for MCP server (protocol, tools, errors, PII check) — 19 tests
- Frontend tests with Vitest + Testing Library — 14 tests
- Codecov integration for coverage tracking
- Accessibility attributes (`role="progressbar"`, `aria-*`, `htmlFor`/`id` linking)
- `sysmon-mcp/README.md` with install/configure/usage docs
- Competitive comparison table in README
- Use cases and AI workflow examples in documentation
- Architecture overview and commit conventions in CONTRIBUTING.md
- Troubleshooting section in MCP server documentation

### Changed

- Refactored MCP `handle_request` for testability
- Release workflow now runs tests before building
- README restructured with AI-first messaging and clearer installation flow
- CONTRIBUTING.md expanded with architecture diagram, testing guide, and "Adding New Metrics" guide

### Fixed

- Autostart toggle now properly wired to `tauri-plugin-autostart`
- Windows disk detection (C:\\ support with fallback)
- Dynamic update interval from config (was hardcoded 3s)
- Input validation with `.clamp()` for all config fields

## [0.1.0] - 2026-03-03

### Added

- Real-time CPU, memory, and disk monitoring in system tray
- Threshold-based OS notifications with configurable cooldown
- Settings panel for customizing thresholds and update interval
- Autostart (launch at login) support
- Cross-platform support (macOS, Windows, Linux)
