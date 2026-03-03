# Contributing

Thank you for your interest in contributing to System Monitor!

## Development Setup

1. Install prerequisites listed in [README.md](README.md#prerequisites)
2. Fork and clone the repository
3. Run `pnpm install` to install dependencies
4. Run `pnpm tauri dev` to start development

## Pull Request Process

1. Create a feature branch from `main`
2. Make your changes with clear, focused commits
3. Ensure both `cargo check` and `npx tsc --noEmit` pass
4. Submit a pull request describing your changes

## Code Style

- **Rust**: Use `cargo fmt` before committing
- **TypeScript**: Follow the existing style; strict mode is enabled
- Keep changes minimal and focused

## Reporting Issues

- Use GitHub Issues
- Include your OS, app version, and steps to reproduce
- For security vulnerabilities, see [SECURITY.md](SECURITY.md)
