# System Monitor

[![CI](https://github.com/ryo-kozin/sysmon-tray/actions/workflows/ci.yml/badge.svg)](https://github.com/ryo-kozin/sysmon-tray/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/ryo-kozin/sysmon-tray/graph/badge.svg)](https://codecov.io/gh/ryo-kozin/sysmon-tray)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Tauri](https://img.shields.io/badge/Tauri-v2-blue?logo=tauri)](https://tauri.app)

A lightweight, cross-platform system resource monitor that lives in your menu bar / system tray — with built-in AI integration via [MCP](https://modelcontextprotocol.io/). Built with [Tauri v2](https://tauri.app), Rust, and React.

> **What makes this different?** Most system monitors just show numbers. This one lets your AI assistant understand your system too — so you can ask "Is my system too busy to run this build?" or "Why is my CPU spiking?" and get answers based on real-time data.

## Features

- **Real-time monitoring** — CPU, memory, and disk usage updated every 3 seconds (configurable)
- **Smart notifications** — Alerts when CPU stays high, memory runs low, or disk space is critically low
- **AI integration (MCP)** — Let Claude Code or Cursor query your system metrics via a privacy-first MCP server
- **System tray** — Runs quietly in your menu bar; click to view stats
- **Configurable thresholds** — Customize all alert thresholds and notification cooldowns
- **Autostart** — Optionally launch at login
- **Privacy-first** — All data stays local. MCP server shares only numerical values — no process names, paths, or PII
- **Lightweight** — ~9 MB RAM, ~0% idle CPU, 414 KB MCP binary

## Why System Monitor?

|                                | System Monitor        | Activity Monitor | Stats      | btop       | NeoHtop    |
| ------------------------------ | --------------------- | ---------------- | ---------- | ---------- | ---------- |
| Menu bar / tray                | Yes                   | No               | Yes        | No         | Yes        |
| AI assistant integration (MCP) | **Yes**               | No               | No         | No         | No         |
| Cross-platform                 | macOS, Windows, Linux | macOS only       | macOS only | All        | All        |
| Memory usage                   | ~9 MB                 | ~89 MB           | ~40 MB     | ~20 MB     | ~50 MB     |
| Smart notifications            | Yes                   | No               | Yes        | No         | No         |
| Privacy-first MCP              | **Yes**               | N/A              | N/A        | N/A        | N/A        |
| License                        | MIT (free)            | Bundled          | MIT (free) | Apache-2.0 | MIT (free) |

The combination of a **lightweight desktop tray app + local MCP server for AI** is currently unique — no other tool offers this.

## Use Cases

- **Developer workflow** — Keep an eye on resources while coding; let your AI assistant check system load before running heavy tasks
- **AI-assisted troubleshooting** — Ask Claude "Is my system busy?" and it queries real metrics via MCP
- **Silent guardian** — Get notified only when something needs attention; no dashboard to babysit
- **Resource-conscious users** — 10x lighter than Activity Monitor, always accessible from the menu bar

## Supported Platforms

| Platform | Status    | Notes                                       |
| -------- | --------- | ------------------------------------------- |
| macOS    | Supported | Xcode CLT required                          |
| Windows  | Supported | WebView2 required (pre-installed on Win 11) |
| Linux    | Supported | webkit2gtk + appindicator required          |

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/ryo-kozin/sysmon-tray.git
cd sysmon-tray

# Install dependencies
pnpm install

# Build for production
pnpm tauri build
```

### Prerequisites (for building from source)

- [Rust](https://rustup.rs/) (1.77+)
- [Node.js](https://nodejs.org/) (20+)
- [pnpm](https://pnpm.io/)
- Platform-specific:
  - **macOS**: Xcode Command Line Tools
  - **Linux**: `libwebkit2gtk-4.1-dev`, `libappindicator3-dev`, `librsvg2-dev`, `patchelf`
  - **Windows**: [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

### Development

```bash
pnpm install
pnpm tauri dev
```

## Configuration

Settings are stored in your platform's config directory:

| Platform | Path                                                       |
| -------- | ---------------------------------------------------------- |
| macOS    | `~/Library/Application Support/system-monitor/config.json` |
| Windows  | `%APPDATA%\system-monitor\config.json`                     |
| Linux    | `~/.config/system-monitor/config.json`                     |

### Default Thresholds

| Resource | Condition                           | Default  |
| -------- | ----------------------------------- | -------- |
| CPU      | Usage above threshold for N seconds | 80%, 10s |
| Memory   | Free memory below threshold         | 10%      |
| Disk     | Free space below threshold          | 10 GB    |
| Cooldown | Time between repeated alerts        | 15 min   |

## MCP Server (AI Integration)

`sysmon-mcp` is a standalone [Model Context Protocol](https://modelcontextprotocol.io/) server that exposes system metrics to AI coding assistants. **Privacy-first**: only numerical values are shared — no process names, file paths, usernames, or any PII.

See [sysmon-mcp/README.md](sysmon-mcp/README.md) for full documentation.

### Quick Setup

```bash
# Install the MCP server
cd sysmon-mcp && cargo install --path .

# Add to Claude Code config (~/.claude/claude_desktop_config.json)
```

```json
{
  "mcpServers": {
    "sysmon": {
      "command": "sysmon-mcp"
    }
  }
}
```

### What Can Your AI Do?

| Tool             | Example Question                       |
| ---------------- | -------------------------------------- |
| `get_metrics`    | "Give me a full system overview"       |
| `get_cpu`        | "How's my CPU doing?"                  |
| `get_memory`     | "How much RAM is available?"           |
| `get_disk`       | "Am I running low on disk space?"      |
| `get_load`       | "What's my system load?"               |
| `is_system_busy` | "Is it safe to run a heavy build now?" |

### Performance

| Metric   | sysmon-mcp | Activity Monitor |
| -------- | ---------- | ---------------- |
| Memory   | ~9 MB      | ~89 MB           |
| Binary   | 414 KB     | —                |
| CPU idle | ~0%        | ~1-3%            |

## Project Structure

```
├── src/                    # React frontend
│   ├── components/
│   │   ├── TrayView.tsx    # Main tray popup with tab navigation
│   │   ├── StatusBar.tsx   # Resource bars with color coding
│   │   └── Settings.tsx    # Settings panel with validation
│   └── hooks/
│       └── useSystemInfo.ts  # Real-time data fetching
├── src-tauri/              # Rust backend (Tauri v2)
│   ├── src/
│   │   ├── lib.rs          # Tauri setup & commands
│   │   ├── monitor.rs      # System info collection
│   │   ├── notifier.rs     # Threshold-based notifications
│   │   └── config.rs       # Settings persistence & validation
│   └── capabilities/
│       └── default.json    # Minimal permissions (no network, no shell)
├── sysmon-mcp/             # Standalone MCP server
│   └── src/main.rs         # Privacy-first system metrics for AI
```

## Roadmap

- **v0.2** — Process list with AI-assisted explanation, process kill with safety checks, notification history
- **v0.3** — Network bandwidth, temperature/fan, battery, themes, anomaly detection
- **Future** — Webhook/Slack integration, resource graphs over time, plugin system, i18n

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup and guidelines.

## License

[MIT](LICENSE)
