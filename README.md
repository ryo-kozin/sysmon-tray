# System Monitor

[![CI](https://github.com/ryo-kozin/sysmon-tray/actions/workflows/ci.yml/badge.svg)](https://github.com/ryo-kozin/sysmon-tray/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Tauri](https://img.shields.io/badge/Tauri-v2-blue?logo=tauri)](https://tauri.app)

A lightweight, cross-platform system resource monitor that lives in your menu bar / system tray. Built with [Tauri v2](https://tauri.app), Rust, and React.

## Features

- **Real-time monitoring** — CPU, memory, and disk usage updated every 3 seconds (configurable)
- **Smart notifications** — Alerts when CPU stays high, memory runs low, or disk space is critically low
- **System tray** — Runs quietly in your menu bar; click to view stats
- **Configurable thresholds** — Customize all alert thresholds and notification cooldowns
- **Autostart** — Optionally launch at login
- **Lightweight** — Target: <30 MB RAM, <0.5% idle CPU, <15 MB binary

## Supported Platforms

| Platform | Status    |
| -------- | --------- |
| macOS    | Supported |
| Windows  | Supported |
| Linux    | Supported |

## Prerequisites

- [Rust](https://rustup.rs/) (1.77+)
- [Node.js](https://nodejs.org/) (20+)
- [pnpm](https://pnpm.io/)
- Platform-specific dependencies:
  - **macOS**: Xcode Command Line Tools
  - **Linux**: `libwebkit2gtk-4.1-dev`, `libappindicator3-dev`, `librsvg2-dev`, `patchelf`
  - **Windows**: [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (pre-installed on Windows 11)

## Getting Started

```bash
# Clone the repository
git clone https://github.com/ryo-kozin/sysmon-tray.git
cd sysmon-tray

# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev

# Build for production
pnpm tauri build
```

## Configuration

Settings are stored in your system config directory:

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

## Project Structure

```
├── src/                    # React frontend
│   ├── components/
│   │   ├── TrayView.tsx    # Main tray popup
│   │   ├── StatusBar.tsx   # Resource bars
│   │   └── Settings.tsx    # Settings panel
│   └── hooks/
│       └── useSystemInfo.ts
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── lib.rs          # Tauri setup & commands
│   │   ├── monitor.rs      # System info via sysinfo
│   │   ├── notifier.rs     # Threshold-based notifications
│   │   └── config.rs       # Settings persistence
│   └── capabilities/
│       └── default.json    # Minimal permissions
└── SPEC.md                 # Detailed specification (Japanese)
```

## Roadmap

- **v0.2** — Process list with kill, notification history
- **v0.3** — Network bandwidth, temperature/fan, battery, themes
- **Future** — Webhook/Slack integration, resource graphs, plugins, i18n

## License

[MIT](LICENSE)
